use derive_more::Display;
use enum_as_inner::EnumAsInner;
use indextree::{Arena, NodeId};
use log::trace;
use lu_error::{AstErr, LuErr, LuResult, SourceCodeItem};
use multimap::MultiMap;
use parking_lot::RwLock;
use std::{
    collections::HashMap,
    fmt::{self, Formatter},
    rc::Rc,
    sync::Arc,
};
use tap::Tap;

pub use indextree::NodeId as ScopeFrameId;

use crate::{Command, ModInfo, ModPath, Strct, Variable};

#[derive(Clone, Debug, PartialEq, Eq, EnumAsInner, is_enum_variant, Display)]
pub enum ScopeFrameTag {
    None,

    GlobalFrame,
    /// Source File Frame with path of the source file
    #[display(fmt = "Module {}", _0)]
    ModuleFrame(ModInfo),

    BlockFrame,
    /// Frame for evaluating cmd (with command-name)
    #[display(fmt = "CmdCallFrame {}", _0)]
    CmdCallFrame(String),
    /// Fn Frame allocated during TyC
    #[display(fmt = "TyCFnFrame {}", _0)]
    TyCFnFrame(String),
    ForStmtFrame,
    IfStmtFrame,
}

impl ScopeFrameTag {}

#[derive(Debug, Clone)]
pub enum ScopeFrameState {
    /// At construction
    Initial,
    /// After Modules are loaded
    ModulesLoaded,
    /// After ValueType::StrctName is converted to ValueType::Strct
    StrctsResolved,
}

/// The default scope frame being put on the scope stack, when entering a new scope
#[derive(Clone, Debug, Display)]
#[display(fmt = "{}", tag)]
pub struct ScopeFrame<Elem>
where
    Elem: fmt::Debug,
{
    pub tag: ScopeFrameTag,
    pub elems: HashMap<String, Elem>,
    pub state: ScopeFrameState,
}

impl<Elem: fmt::Debug> ScopeFrame<Elem> {
    pub fn new(tag: ScopeFrameTag) -> Self {
        Self {
            tag,
            elems: HashMap::new(),
            state: ScopeFrameState::Initial,
        }
    }

    pub fn get_tag(&self) -> &ScopeFrameTag {
        &self.tag
    }

    pub fn get(&self, name: &str) -> Option<&Elem> {
        self.elems.get(name)
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut Elem> {
        self.elems.get_mut(name)
    }
    pub fn get_mod_tag(&self) -> &ModInfo {
        self.tag.as_module_frame().unwrap()
    }
}

impl ScopeFrame<Variable> {
    pub fn insert_var(&mut self, var: Variable) -> Option<Variable> {
        self.elems.insert(var.name.clone(), var)
    }
}

#[derive(Clone)]
pub struct Scope<T>
where
    T: fmt::Debug,
{
    pub arena: Arena<ScopeFrame<T>>,
    /// Always a valid id
    cur_frame_id: Option<NodeId>,

    /// From NodeId of SourceFile to [NodeId of SourceFile]
    pub use_stmts: MultiMap<NodeId, NodeId>,
}

impl<T: fmt::Debug + 'static> Scope<T> {
    pub fn new() -> Self {
        Scope {
            arena: Arena::new(),
            cur_frame_id: None,
            use_stmts: MultiMap::new(),
        }
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.cur_frame_id.is_none()
    }

    fn tag_of(&self, id: NodeId) -> &ScopeFrameTag {
        self.arena[id].get().get_tag()
    }

    pub fn get_cur_frame_id(&self) -> ScopeFrameId {
        self.cur_frame_id.unwrap()
    }

    fn root_id(&self) -> NodeId {
        let cur_id = self.get_cur_frame_id();
        cur_id.ancestors(&self.arena).last().unwrap()
    }

    fn get_sf_frames_parent(&self) -> NodeId {
        // All sf_frames are below the global_frame
        self.root_id()
    }

    /// Id must be valid. Panic otherwise!
    pub fn set_cur_frame_id(&mut self, id: ScopeFrameId) {
        assert!(self.arena.get(id).is_some());
        self.cur_frame_id = Some(id);
    }

    pub fn get_cur_frame(&self) -> &ScopeFrame<T> {
        self.arena[self.get_cur_frame_id()].get()
    }

    pub fn get_cur_frame_mut(&mut self) -> &mut ScopeFrame<T> {
        let id = self.get_cur_frame_id();
        self.arena[id].get_mut()
    }

    pub fn get_global_frame_mut(&mut self) -> &mut ScopeFrame<T> {
        let global_id = self.root_id();
        let frame = self.arena[global_id].get_mut();
        assert!(frame.get_tag().is_global_frame());
        frame
    }

    pub fn push_frame_(&mut self, frame: ScopeFrame<T>) -> (ScopeFrameId, &mut ScopeFrame<T>) {
        trace!("Pushing frame: {}", frame.tag);
        let prev_frame_id = self.cur_frame_id;
        let new_frame_id = self.arena.new_node(frame);
        if let Some(prev_frame_id) = prev_frame_id {
            prev_frame_id.append(new_frame_id, &mut self.arena);
        }
        self.cur_frame_id = Some(new_frame_id);

        (new_frame_id, self.get_cur_frame_mut())
    }

    pub fn push_frame(&mut self, tag: ScopeFrameTag) -> (ScopeFrameId, &mut ScopeFrame<T>) {
        self.push_frame_(ScopeFrame::new(tag))
    }

    pub fn pop_frame(&mut self, expected: &ScopeFrameTag) {
        if let Some(cur_frame_id) = self.cur_frame_id {
            let cur_frame = &self.arena[cur_frame_id];
            let cur_frame_tag = cur_frame.get().get_tag();

            trace!("Popping frame: {}, Expected: {}", cur_frame_tag, expected);
            assert_eq!(cur_frame_tag, expected);

            let parent_id = cur_frame.parent();
            cur_frame_id.remove(&mut self.arena);
            self.cur_frame_id = parent_id;
        } else {
            trace!("Tried to pop_frame, but scope is empty")
        }
    }

    pub fn select_parent_frame(&mut self) {
        trace!("Selecting parent frame");
        let cur_id = self.get_cur_frame_id();
        self.cur_frame_id = cur_id.ancestors(&self.arena).skip(1).next();
    }
}

impl Scope<Variable> {
    /// Returns the command, in which the current selected frame is.
    pub fn find_cur_command(&self) -> Option<&Rc<dyn Command>> {
        let cur_id = self.get_cur_frame_id();
        cur_id.ancestors(&self.arena).find_map(|n_id| {
            let tag = self.arena[n_id].get().get_tag();
            if let Some(func_name) = tag.as_ty_c_fn_frame().or_else(|| tag.as_cmd_call_frame()) {
                self.find_func(func_name)
                    .tap(|func| assert!(func.is_some(), "Cmd of CmdFrame has to be always found"))
            } else {
                None
            }
        })
    }

    #[allow(dead_code)]
    fn cur_source_f_id(&self) -> Option<NodeId> {
        if let Some(cur_frame_id) = self.cur_frame_id {
            cur_frame_id.ancestors(&self.arena).find_map(|n_id| {
                let frame = self.arena[n_id].get();
                if frame.get_tag().as_module_frame().is_some() {
                    Some(n_id)
                } else {
                    None
                }
            })
        } else {
            None
        }
    }

    fn frames_to_find_var_in(&self) -> Vec<NodeId> {
        let mut frames_to_find_var_in = vec![];
        for frame in self.get_cur_frame_id().ancestors(&self.arena) {
            frames_to_find_var_in.push(frame);
            if let Some(modi) = self.tag_of(frame).as_module_frame() {
                // TODO obay pub use paths (if it should land)
                frames_to_find_var_in.extend(
                    modi.use_paths
                        .iter()
                        .map(|path| self.get_nid_of_sf_frame(&path.mod_path).unwrap()),
                )
            }
        }
        frames_to_find_var_in
    }

    pub fn find_var(&self, name: &str) -> Option<&Variable> {
        trace!("Finding var {} in\n{:?}", name, self);
        let start_frame = self.get_cur_frame().get_tag();
        let frames_to_check_var_for = self.frames_to_find_var_in();
        frames_to_check_var_for
            .iter()
            .map(|frame_id| self.arena[*frame_id].get())
            .find_map(|frame| frame.get(name))
            .tap(|result| {
                trace!(
                    "Result for find_var {} from start_frame {}: {:?}",
                    name,
                    start_frame,
                    result
                )
            })
    }

    pub fn find_var_mut(&mut self, name: &str) -> Option<&mut Variable> {
        trace!("Finding var {} in\n{:?}", name, self);
        let start_frame = self.get_cur_frame().get_tag().clone();
        let frames_to_check_var_for = self.frames_to_find_var_in();
        for frame in frames_to_check_var_for {
            if self.arena[frame].get_mut().get_mut(name).is_some() {
                trace!(
                    "Result for find_var_mut {} from start_frame {}: {:?}",
                    name,
                    start_frame,
                    self.arena[frame].get_mut().get_mut(name)
                );
                return self.arena[frame].get_mut().get_mut(name);
            }
        }
        trace!(
            "Could not find var_mut {} from start_frame {}",
            name,
            start_frame
        );
        None
    }

    pub fn expect_var_mut(
        &mut self,
        var_name: &str,
        usage: SourceCodeItem,
    ) -> LuResult<&mut Variable> {
        self.find_var_mut(var_name)
            .ok_or(AstErr::VarNotInScope(usage).into())
    }

    pub fn find_func(&self, name: &str) -> Option<&Rc<dyn Command>> {
        trace!("Finding cmd {} from {} on", name, self.get_cur_frame());
        // TODO write check that no variable shadows a func name
        self.find_var(name)
            .map(|var| var.val.as_command())
            .flatten()
    }

    pub fn find_strct(&self, name: &str) -> Option<&Arc<RwLock<Strct>>> {
        trace!("Finding cmd {} from {} on", name, self.get_cur_frame());
        // TODO write check that no variable shadows a func name
        self.find_var(name)
            .map(|var| var.val.as_strct_decl().unwrap())
    }

    pub fn expect_strct(&self, name: &str, usage: SourceCodeItem) -> LuResult<&Arc<RwLock<Strct>>> {
        self.find_strct(name)
            .ok_or(AstErr::StrctNotInScope(usage).into())
    }

    pub fn push_sf_frame(&mut self, frame: ScopeFrame<Variable>) {
        assert!(frame.get_tag().is_module_frame());
        let sf_frames_parent = self.get_sf_frames_parent();
        let new_frame_id = self.arena.new_node(frame);
        sf_frames_parent.append(new_frame_id, &mut self.arena);
    }

    fn get_nid_of_sf_frame(&self, path: &ModPath) -> Option<NodeId> {
        trace!("get_nid_of_sf_frame({})", path);
        let sf_frames_parent = self.get_sf_frames_parent();
        sf_frames_parent
            .children(&self.arena)
            .filter(|sf_id| {
                let modi = self.arena[*sf_id]
                    .get()
                    .get_tag()
                    .as_module_frame()
                    .unwrap();
                modi.id == *path
            })
            .next()
    }

    pub fn select_sf_frame(&mut self, f_to_set: &ModPath) -> LuResult<()> {
        if let Some(sf_to_select) = self.get_nid_of_sf_frame(f_to_set) {
            self.cur_frame_id = Some(sf_to_select);
            Ok(())
        } else {
            Err(LuErr::Internal(format!(
                "Scope does not contain source file {} ",
                f_to_set
            )))
        }
    }
}

impl<T: fmt::Debug + 'static> fmt::Debug for Scope<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_empty() {
            return write!(f, "Empty Scope");
        }

        let mut indent = 0;
        write!(f, "\n")?;
        for elem in self.root_id().traverse(&self.arena) {
            match elem {
                indextree::NodeEdge::Start(id) => {
                    let is_selected = if id == self.cur_frame_id.unwrap() {
                        "*"
                    } else {
                        ""
                    };
                    &write!(
                        f,
                        "{:indent$}{}{}\n",
                        "",
                        is_selected,
                        self.tag_of(id),
                        indent = indent
                    )?;
                    indent = indent + 4;
                }
                indextree::NodeEdge::End(_) => indent = indent - 4,
            }
        }
        write!(f, "\n")
    }
}

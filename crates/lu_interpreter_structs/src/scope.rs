use derive_more::Display;
use enum_as_inner::EnumAsInner;
use indextree::{Arena, NodeId};
use log::debug;
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

use crate::{Command, Strct, UsePath, Value, Variable};

#[derive(Clone, Debug, PartialEq, Eq, EnumAsInner, is_enum_variant, Display)]
pub enum ScopeFrameTag {
    None,

    GlobalFrame,
    /// Source File Frame with path of the source file
    #[display(fmt = "SFFrame {}", id)]
    SFFrame {
        id: UsePath,
        use_paths: Vec<UsePath>,
    },

    BlockFrame,
    /// Frame for evaluating cmd (with command-name)
    #[display(fmt = "CmdCallFrame {}", _0)]
    CmdCallFrame(String),
    /// Fn Frame with name of fn
    FnFrame(String),
    ForStmtFrame,
    IfStmtFrame,
}

impl ScopeFrameTag {
    pub fn new_source_file_tag(id: UsePath, use_paths: Vec<UsePath>) -> Self {
        Self::SFFrame { id, use_paths }
    }
}

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

    pub fn push_frame(&mut self, tag: ScopeFrameTag) -> (ScopeFrameId, &mut ScopeFrame<T>) {
        debug!("Pushing frame: {}", tag);
        let prev_frame_id = self.cur_frame_id;
        let new_frame_id = self.arena.new_node(ScopeFrame::new(tag));
        if let Some(prev_frame_id) = prev_frame_id {
            prev_frame_id.append(new_frame_id, &mut self.arena);
        }
        self.cur_frame_id = Some(new_frame_id);

        (new_frame_id, self.get_cur_frame_mut())
    }

    pub fn pop_frame(&mut self, expected: &ScopeFrameTag) {
        if let Some(cur_frame_id) = self.cur_frame_id {
            let cur_frame = &self.arena[cur_frame_id];
            let cur_frame_tag = cur_frame.get().get_tag();

            debug!("Popping frame: {}, Expected: {}", cur_frame_tag, expected);
            assert_eq!(cur_frame_tag, expected);

            let parent_id = cur_frame.parent();
            cur_frame_id.remove(&mut self.arena);
            self.cur_frame_id = parent_id;
        } else {
            debug!("Tried to pop_frame, but scope is empty")
        }
    }

    pub fn select_parent_frame(&mut self) {
        debug!("Selecting parent frame");
        let cur_id = self.get_cur_frame_id();
        self.cur_frame_id = cur_id.ancestors(&self.arena).skip(1).next();
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.cur_frame_id.is_none()
    }
}

impl Scope<Variable> {
    /// Returns the function, in which the current selected frame is.
    pub fn get_cur_command(&self) -> Option<&Rc<dyn Command>> {
        self.cur_frame_id
            .map(|cur_frame_id| {
                cur_frame_id.ancestors(&self.arena).find_map(|n_id| {
                    self.tag_of(n_id)
                        .as_fn_frame()
                        .map(|fn_name| self.find_func(fn_name))
                })
            })
            .flatten()
            .flatten()
    }

    #[allow(dead_code)]
    fn cur_source_f_id(&self) -> Option<NodeId> {
        if let Some(cur_frame_id) = self.cur_frame_id {
            cur_frame_id.ancestors(&self.arena).find_map(|n_id| {
                let frame = self.arena[n_id].get();
                if frame.get_tag().as_sf_frame().is_some() {
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
            if let Some((_, use_stmts)) = self.tag_of(frame).as_sf_frame() {
                // TODO obay pub use paths (if it should land)
                frames_to_find_var_in.extend(
                    use_stmts
                        .iter()
                        .map(|path| self.get_id_of_sf_frame(path).unwrap()),
                )
            }
        }
        frames_to_find_var_in
    }

    pub fn find_var(&self, name: &str) -> Option<&Variable> {
        let start_frame = self.get_cur_frame().get_tag();
        let frames_to_check_var_for = self.frames_to_find_var_in();
        frames_to_check_var_for
            .iter()
            .map(|frame_id| self.arena[*frame_id].get())
            .find_map(|frame| {
                debug!("Finding var: \"{}\" in frame: {}", name, frame);
                frame.get(name)
            })
            .tap(|result| {
                debug!(
                    "Result for find_var {} from start_frame {}: {:?}",
                    name, start_frame, result
                )
            })
    }

    pub fn find_var_mut(&mut self, name: &str) -> Option<&mut Variable> {
        let frames_to_check_var_for = self.frames_to_find_var_in();
        for frame in frames_to_check_var_for {
            if self.arena[frame].get_mut().get_mut(name).is_some() {
                return self.arena[frame].get_mut().get_mut(name);
            }
        }
        None
    }

    #[deprecated]
    pub fn overwrite_var_value(&mut self, name: &str, new_value: Value) -> bool {
        for frame_id in self.frames_to_find_var_in() {
            let frame = self.arena[frame_id].get_mut();
            if let Some(var) = frame.get_mut(name) {
                debug!("Overwriting var {} with value: {:?}", name, new_value);
                var.val = new_value;
                return true;
            }
        }

        debug!(
            "Not Overwriting var {} with value: {:?}. Var not found!",
            name, new_value
        );
        false
    }

    #[allow(unused)]
    pub fn find_func(&self, name: &str) -> Option<&Rc<dyn Command>> {
        debug!("Finding cmd {} from {} on", name, self.get_cur_frame());
        // TODO write check that no variable shadows a func name
        self.find_var(name)
            .map(|var| var.val.as_command())
            .flatten()
    }

    #[allow(unused)]
    pub fn find_strct(&self, name: &str) -> Option<&Arc<RwLock<Strct>>> {
        debug!("Finding cmd {} from {:?} on", name, self.get_cur_frame());
        // TODO write check that no variable shadows a func name
        self.find_var(name).map(|var| var.val.as_strct().unwrap())
    }

    pub fn expect_strct(&self, name: &str, usage: SourceCodeItem) -> LuResult<&Arc<RwLock<Strct>>> {
        self.find_strct(name)
            .ok_or(AstErr::StrctNotInScope(usage).into())
    }

    /// Find the command, having the longest match with name_parts (where not every part of
    /// name_parts has to be matched)
    /// Example:
    /// Stored cmd name: git add
    /// name_parts:      git add my_file
    /// will return (2, <git-add-cmd>)
    // The call side can not necessarily distinguish between cmd name parts and arguments.
    // Therefore we need to do some search here
    pub fn find_cmd_with_longest_match(
        &self,
        name_parts: &[String],
    ) -> Option<(usize, &Rc<dyn Command>)> {
        let result = self
            .find_var_with_longest_match(name_parts)
            .map(|(i, var)| (i, var.val.as_command()));
        if let Some((i, Some(callable))) = result {
            Some((i, callable))
        } else {
            None
        }
    }

    /// See find_cmd_with_longest_match
    pub fn find_var_with_longest_match(&self, name_parts: &[String]) -> Option<(usize, &Variable)> {
        assert!(name_parts.len() > 0);
        // We try to find the longest matching subcommand here ...  Maybe we should use a trie as
        // the internal datastructure
        let mut result = None;
        for i in 0..name_parts.len() {
            let cmd_name = name_parts[0..i + 1].join(" ");
            if let Some(func) = self.find_var(&cmd_name) {
                result = Some((i + 1, func))
            }
        }
        result
    }

    fn get_sf_frames_parent(&self) -> NodeId {
        // All sf_frames are below the global_frame
        self.root_id()
    }

    pub fn push_sf_frame(&mut self, frame: ScopeFrame<Variable>) {
        assert!(frame.get_tag().is_sf_frame());
        let sf_frames_parent = self.get_sf_frames_parent();
        let new_frame_id = self.arena.new_node(frame);
        sf_frames_parent.append(new_frame_id, &mut self.arena);
    }

    fn get_id_of_sf_frame(&self, path: &UsePath) -> Option<NodeId> {
        let sf_frames_parent = self.get_sf_frames_parent();
        sf_frames_parent
            .children(&self.arena)
            .filter(|sf_id| {
                let (id, _) = self.arena[*sf_id].get().get_tag().as_sf_frame().unwrap();
                id == path
            })
            .next()
    }

    pub fn select_sf_frame(&mut self, f_to_set: &UsePath) -> LuResult<()> {
        if let Some(sf_to_select) = self.get_id_of_sf_frame(f_to_set) {
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

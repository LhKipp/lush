use indextree::{Arena, NodeId};
use log::debug;
use lu_error::{AstErr, LuErr, LuResult, SourceCodeItem};
use std::{collections::HashMap, fmt, path::PathBuf};
use tap::prelude::*;

pub use indextree::NodeId as ScopeFrameId;
use lu_syntax_elements::BlockType;

use crate::{Callable, Strct, Variable};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ScopeFrameTag {
    None,

    GlobalFrame,
    SourceFileFrame(PathBuf),

    BlockFrame,
    FnFrame,
    ForStmtFrame,
    IfStmtFrame,
}

impl From<BlockType> for ScopeFrameTag {
    fn from(b_type: BlockType) -> Self {
        match b_type {
            BlockType::FnBlock => Self::FnFrame,
            BlockType::ForBlock => Self::ForStmtFrame,
            _ => unreachable!("TODO"),
        }
    }
}

/// The default scope frame being put on the scope stack, when entering a new scope
#[derive(Clone, Debug)]
pub struct ScopeFrame<Elem>
where
    Elem: fmt::Debug,
{
    pub tag: ScopeFrameTag,
    pub elems: HashMap<String, Elem>,
}

impl<Elem: fmt::Debug> ScopeFrame<Elem> {
    pub fn new(tag: ScopeFrameTag) -> Self {
        Self {
            tag,
            elems: HashMap::new(),
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

    pub fn insert(&mut self, key: String, var: Elem) {
        debug!("Inserting into Scope {:?} with name {}", var, key);
        self.elems.insert(key, var);
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
}

impl<T: fmt::Debug + 'static> Scope<T> {
    pub fn new() -> Self {
        Scope {
            arena: Arena::new(),
            cur_frame_id: None,
        }
    }

    pub fn get_cur_frame_id(&self) -> ScopeFrameId {
        self.cur_frame_id.unwrap()
    }

    /// Id must be valid. Panic otherwise!
    pub fn set_cur_frame_id(&mut self, id: ScopeFrameId) {
        assert!(self.arena.get(id).is_some());
        self.cur_frame_id = Some(id);
    }

    pub fn cur_frame(&self) -> &ScopeFrame<T> {
        self.arena
            .get(self.cur_frame_id.expect("Scope is empty"))
            .unwrap()
            .get()
    }

    pub fn cur_mut_frame(&mut self) -> &mut ScopeFrame<T> {
        self.arena
            .get_mut(self.get_cur_frame_id())
            .unwrap()
            .get_mut()
    }

    pub fn global_mut_frame(&mut self) -> &mut ScopeFrame<T> {
        let ancestors: Vec<NodeId> = self.get_cur_frame_id().ancestors(&self.arena).collect();
        let global_id = ancestors.last().unwrap();
        self.arena[*global_id].get_mut()
    }

    pub fn push_frame(&mut self, tag: ScopeFrameTag) -> (ScopeFrameId, &mut ScopeFrame<T>) {
        debug!("Pushing frame: {:?}", tag);
        let prev_frame_id = self.cur_frame_id;
        let new_frame_id = self.arena.new_node(ScopeFrame::new(tag));
        if let Some(prev_frame_id) = prev_frame_id {
            prev_frame_id.append(new_frame_id, &mut self.arena);
        }
        self.cur_frame_id = Some(new_frame_id);

        (new_frame_id, self.cur_mut_frame())
    }

    pub fn pop_frame(&mut self, expected: &ScopeFrameTag) {
        if let Some(cur_frame_id) = self.cur_frame_id {
            let cur_frame = &self.arena[cur_frame_id];
            let cur_frame_tag = cur_frame.get().get_tag();

            debug!(
                "Popping frame: {:?}, Expected: {:?}",
                cur_frame_tag, expected
            );
            assert_eq!(cur_frame_tag, expected);

            let parent_id = cur_frame.parent();
            cur_frame_id.remove(&mut self.arena);
            self.cur_frame_id = parent_id;
        } else {
            debug!("Tried to pop_frame, but scope is empty")
        }
    }

    fn get_cur_tag(&self) -> ScopeFrameTag {
        if let Some(cur_frame_id) = self.cur_frame_id {
            let cur_frame = &self.arena[cur_frame_id];
            cur_frame.get().get_tag().clone()
        } else {
            ScopeFrameTag::None
        }
    }

    fn root_id(&self) -> Option<NodeId> {
        self.cur_frame_id
            .map(|id| id.ancestors(&self.arena).last())
            .flatten()
    }

    fn tag_of(&self, id: NodeId) -> &ScopeFrameTag {
        self.arena[id].get().get_tag()
    }

    fn fmt_as_string(&self) -> String {
        if self.is_empty() {
            return "Empty Scope".to_string();
        }

        let mut indent = 0;
        let mut result = "\n".to_string();
        for elem in self.root_id().unwrap().traverse(&self.arena) {
            match elem {
                indextree::NodeEdge::Start(id) => {
                    let is_selected = if id == self.cur_frame_id.unwrap() {
                        "*"
                    } else {
                        ""
                    };
                    result = result
                        + &format!(
                            "{:indent$}{}{:?}\n",
                            "",
                            is_selected,
                            self.tag_of(id),
                            indent = indent
                        );
                    indent = indent + 4;
                }
                indextree::NodeEdge::End(_) => indent = indent - 4,
            }
        }

        result
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.cur_frame_id.is_none()
    }
}

impl Scope<Variable> {
    pub fn find_var(&self, name: &str) -> Option<&Variable> {
        debug!("Finding var {} from {:?} on", name, self.get_cur_tag());
        if let Some(cur_frame_id) = self.cur_frame_id {
            cur_frame_id
                .ancestors(&self.arena)
                .map(|n_id| {
                    self.arena
                        .get(n_id)
                        .expect("Current_id should always have at least 1 ancestor")
                        .get()
                })
                .flat_map(|frame| frame.get(name))
                .next()
                .tap(|result| debug!("Found var: {:?}", result))
        } else {
            debug!("Tried to get_var, but scope is empty");
            None
        }
    }

    #[allow(unused)]
    fn find_func(&self, name: &str) -> Option<&Callable> {
        debug!("Finding cmd {} from {:?} on", name, self.get_cur_tag());
        // TODO write check that no variable shadows a func name
        self.find_var(name)
            .map(|var| var.val_as_callable())
            .flatten()
    }

    #[allow(unused)]
    pub fn find_strct(&self, name: &str) -> Option<&Strct> {
        debug!("Finding cmd {} from {:?} on", name, self.get_cur_tag());
        // TODO write check that no variable shadows a func name
        self.find_var(name).map(|var| var.val_as_strct()).flatten()
    }

    pub fn expect_strct(&self, name: &str, usage: SourceCodeItem) -> LuResult<&Strct> {
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
    pub fn find_cmd_with_longest_match(&self, name_parts: &[String]) -> Option<(usize, &Callable)> {
        let result = self
            .find_var_with_longest_match(name_parts)
            .map(|(i, var)| (i, var.val_as_callable()));
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

    pub fn set_cur_source_frame(&mut self, f_to_set: &PathBuf) -> LuResult<()> {
        debug!("set_cur_source_frame");
        if let Some(cur_frame_id) = self.cur_frame_id {
            let stages = cur_frame_id
                .ancestors(&self.arena)
                .chain(cur_frame_id.descendants(&self.arena));
            for id in stages {
                match self.arena.get(id).unwrap().get().get_tag() {
                    ScopeFrameTag::SourceFileFrame(_) => {
                        // Source file frame stage reached
                        // Now iterate over left and right to find matching frame
                        debug!("Found source file frame stage");
                        for sipling in id
                            .preceding_siblings(&self.arena)
                            .chain(id.following_siblings(&self.arena))
                        {
                            match self.arena.get(sipling).unwrap().get().get_tag() {
                                ScopeFrameTag::SourceFileFrame(f_name) => {
                                    debug!("{:?} == {:?} ???", f_name, f_to_set);
                                    if f_name == f_to_set {
                                        debug!("set_cur_frame_id found matching source file frame {:?}", f_name);
                                        self.set_cur_frame_id(sipling);
                                        return Ok(());
                                    }
                                }
                                _ => unreachable!(""),
                            }
                        }
                    }
                    _ => {}
                };
            }
        }

        debug!("set_cur_source_frame returning error");
        Err(LuErr::Internal(
            "Expected the scope to have at least 1 element".to_string(),
        ))
    }
}

impl<T: fmt::Debug + 'static> fmt::Debug for Scope<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.fmt_as_string())
    }
}

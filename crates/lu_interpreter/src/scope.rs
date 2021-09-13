use indextree::{Arena, NodeId};
use log::debug;
use lu_value::Value;
use std::collections::HashMap;
use tap::prelude::*;

pub use indextree::NodeId as ScopeFrameId;
use lu_syntax_elements::BlockType;

use crate::{Callable, Command, Variable};

pub trait ScopeFrame {
    type Elem;
    fn get_tag(&self) -> ScopeFrameTag;
    fn get(&self, name: &str) -> Option<&Self::Elem>;
    fn get_mut(&mut self, name: &str) -> Option<&mut Self::Elem>;
    fn insert(&mut self, key: String, var: Self::Elem);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopeFrameTag {
    None,
    SourceFileFrame,
    BlockFrame,
    GlobalFrame,
    FnFrame,
    ForStmtFrame,
    IfStmtFrame,
}

impl From<BlockType> for ScopeFrameTag {
    fn from(b_type: BlockType) -> Self {
        match b_type {
            BlockType::SourceFileBlock => Self::SourceFileFrame,
            BlockType::FnBlock => Self::FnFrame,
            BlockType::ForBlock => Self::ForStmtFrame,
        }
    }
}

/// The default scope frame being put on the scope stack, when entering a new scope
pub struct SimpleScopeFrame<Elem> {
    pub tag: ScopeFrameTag,
    pub vars: HashMap<String, Elem>,
}

impl<Elem> SimpleScopeFrame<Elem> {
    pub fn new(tag: ScopeFrameTag) -> Self {
        Self {
            tag,
            vars: HashMap::new(),
        }
    }
}

impl<Elem> ScopeFrame for SimpleScopeFrame<Elem> {
    type Elem = Elem;
    fn get_tag(&self) -> ScopeFrameTag {
        self.tag
    }

    fn get(&self, name: &str) -> Option<&Elem> {
        self.vars.get(name)
    }

    fn get_mut(&mut self, name: &str) -> Option<&mut Elem> {
        self.vars.get_mut(name)
    }

    fn insert(&mut self, key: String, var: Elem) {
        self.vars.insert(key, var);
    }
}

pub struct Scope<OfT> {
    pub arena: Arena<Box<dyn ScopeFrame<Elem = OfT>>>,
    /// Always a valid id
    cur_frame_id: Option<NodeId>,
}

impl<OfT: 'static> Scope<OfT> {
    pub fn new() -> Self {
        Scope {
            arena: Arena::<Box<dyn ScopeFrame<Elem = OfT>>>::new(),
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

    pub fn cur_frame(&self) -> &dyn ScopeFrame<Elem = OfT> {
        self.arena
            .get(self.cur_frame_id.expect("Scope is empty"))
            .unwrap()
            .get()
            .as_ref()
    }

    pub fn cur_mut_frame(&mut self) -> &mut dyn ScopeFrame<Elem = OfT> {
        self.arena
            .get_mut(self.cur_frame_id.expect("Scope is empty"))
            .unwrap()
            .get_mut()
            .as_mut()
    }

    pub fn global_mut_frame(&mut self) -> &mut dyn ScopeFrame<Elem = OfT> {
        let ancestors: Vec<NodeId> = self
            .cur_frame_id
            .expect("Scope is empty")
            .ancestors(&self.arena)
            .collect();
        let global_id = ancestors.last().unwrap();
        let global_frame = self.arena.get_mut(*global_id).unwrap();
        global_frame.get_mut().as_mut()
    }

    pub fn push_frame(
        &mut self,
        tag: ScopeFrameTag,
    ) -> (ScopeFrameId, &mut dyn ScopeFrame<Elem = OfT>) {
        debug!("Pushing frame: {:?}", tag);
        let prev_frame_id = self.cur_frame_id;
        let new_frame_id = self.arena.new_node(Box::new(SimpleScopeFrame::new(tag)));
        if let Some(prev_frame_id) = prev_frame_id {
            prev_frame_id.append(new_frame_id, &mut self.arena);
        }
        self.cur_frame_id = Some(new_frame_id);

        (new_frame_id, self.cur_mut_frame())
    }

    pub fn pop_frame(&mut self, expected: ScopeFrameTag) {
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
            cur_frame.get().get_tag()
        } else {
            ScopeFrameTag::None
        }
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

    fn find_func(&self, name: &str) -> Option<&Callable> {
        debug!("Finding cmd {} from {:?} on", name, self.get_cur_tag());
        if let Some(var) = self.find_var(name) {
            match &var.val {
                Value::Function(func) => Some(
                    func.downcast_ref::<Callable>()
                        .expect("Func is always castable to Callable"),
                ),
                _ => todo!("TODO found variable must not be func"),
            }
        } else {
            None
        }
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
        assert!(name_parts.len() > 0);
        // We try to find the longest matching subcommand here ...  Maybe we should use a trie as
        // the internal datastructure
        let mut result = None;
        for i in 0..name_parts.len() {
            let cmd_name = name_parts[0..i + 1].join(" ");
            if let Some(func) = self.find_func(&cmd_name) {
                result = Some((i + 1, func))
            }
        }
        debug!(
            "Match found for cmd_name_parts {:?}: ({:?}, {:?})",
            name_parts,
            result.map_or("None", |(_, func)| &func.name()),
            result.map_or(999, |(idx, _)| idx)
        );
        result
    }
}

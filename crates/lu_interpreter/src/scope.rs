use indextree::{Arena, NodeId};
use log::debug;
use std::{collections::HashMap, rc::Rc};
use tap::prelude::*;

use lu_value::Value;

pub use indextree::NodeId as ScopeFrameId;

use crate::Function;

pub trait ScopeFrame {
    fn get_tag(&self) -> ScopeFrameTag;
    fn get_var(&self, name: &str) -> Option<&Value>;
    fn get_mut_var(&mut self, name: &str) -> Option<&mut Value>;
    fn insert_var(&mut self, name: String, val: Value);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScopeFrameTag {
    None,
    BlockFrame,
    GlobalFrame,
    FnFrame,
    ForStmtFrame,
    IfStmtFrame,
}

/// The default scope frame being put on the scope stack, when entering a new scope
pub struct SimpleScopeFrame {
    pub tag: ScopeFrameTag,
    pub vars: HashMap<String, Value>,
    pub funcs: HashMap<String, Rc<Function>>,
}

impl SimpleScopeFrame {
    pub fn new(tag: ScopeFrameTag) -> Self {
        Self {
            tag,
            vars: HashMap::new(),
            funcs: HashMap::new(),
        }
    }
}

impl ScopeFrame for SimpleScopeFrame {
    fn get_tag(&self) -> ScopeFrameTag {
        self.tag
    }

    fn get_var(&self, name: &str) -> Option<&Value> {
        self.vars.get(name)
    }

    fn get_mut_var(&mut self, name: &str) -> Option<&mut Value> {
        self.vars.get_mut(name)
    }

    fn insert_var(&mut self, name: String, val: Value) {
        self.vars.insert(name, val);
    }
}

pub struct Scope {
    pub arena: Arena<Box<dyn ScopeFrame>>,
    /// Always a valid id
    cur_frame_id: Option<NodeId>,
}

impl Scope {
    pub fn new() -> Self {
        Scope {
            arena: Arena::<Box<dyn ScopeFrame>>::new(),
            cur_frame_id: None,
        }
    }

    pub fn cur_frame(&self) -> &dyn ScopeFrame {
        self.arena
            .get(self.cur_frame_id.expect("Scope is empty"))
            .unwrap()
            .get()
            .as_ref()
    }

    pub fn cur_mut_frame(&mut self) -> &mut dyn ScopeFrame {
        self.arena
            .get_mut(self.cur_frame_id.expect("Scope is empty"))
            .unwrap()
            .get_mut()
            .as_mut()
    }

    pub fn global_mut_frame(&mut self) -> &mut dyn ScopeFrame {
        let ancestors: Vec<NodeId> = self
            .cur_frame_id
            .expect("Scope is empty")
            .ancestors(&self.arena)
            .collect();
        let global_id = ancestors.last().unwrap();
        let global_frame = self.arena.get_mut(*global_id).unwrap();
        global_frame.get_mut().as_mut()
    }

    pub fn get_var(&self, name: &str) -> Option<&Value> {
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
                .flat_map(|frame| frame.get_var(name))
                .next()
                .tap(|result| debug!("Found var: {:?}", result))
        } else {
            debug!("Tried to get_var, but scope is empty");
            None
        }
    }

    pub fn push_frame(&mut self, tag: ScopeFrameTag) -> &mut dyn ScopeFrame {
        debug!("Pushing frame: {:?}", tag);
        let prev_frame_id = self.cur_frame_id;
        let new_frame_id = self.arena.new_node(Box::new(SimpleScopeFrame::new(tag)));
        if let Some(prev_frame_id) = prev_frame_id {
            prev_frame_id.append(new_frame_id, &mut self.arena);
        }
        self.cur_frame_id = Some(new_frame_id);

        self.cur_mut_frame()
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

use indextree::{Arena, NodeId};
use std::collections::HashMap;

use value::Value;

pub trait ScopeFrame {
    fn get_tag(&self) -> ScopeFrameTag;
    fn get_var(&self, name: &str) -> Option<&Value>;
    fn get_mut_var(&mut self, name: &str) -> Option<&mut Value>;
    fn insert_var(&mut self, name: String, val: Value);
}

#[derive(Clone, Copy)]
pub enum ScopeFrameTag {
    GlobalFrame,
    FnFrame,
}

/// The default scope frame being put on the scope stack, when entering a new scope
pub struct SimpleScopeFrame {
    pub tag: ScopeFrameTag,
    pub vars: HashMap<String, Value>,
}

impl SimpleScopeFrame {
    pub fn new(tag: ScopeFrameTag) -> Self {
        Self {
            tag,
            vars: HashMap::new(),
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
    pub current_id: NodeId,
}

impl Scope {
    pub fn new() -> Self {
        // We create one global frame, so that current_id is valid from ctor on.
        let mut arena = Arena::<Box<dyn ScopeFrame>>::new();
        let global_frame =
            arena.new_node(Box::new(SimpleScopeFrame::new(ScopeFrameTag::GlobalFrame)));
        Scope {
            arena,
            current_id: global_frame,
        }
    }

    pub fn cur_frame(&self) -> &dyn ScopeFrame {
        self.arena.get(self.current_id).unwrap().get().as_ref()
    }

    pub fn cur_mut_frame(&mut self) -> &mut dyn ScopeFrame {
        self.arena
            .get_mut(self.current_id)
            .unwrap()
            .get_mut()
            .as_mut()
    }

    pub fn get_var(&self, name: &str) -> Option<&Value> {
        self.current_id
            .ancestors(&self.arena)
            .map(|n_id| {
                self.arena
                    .get(n_id)
                    .expect("Current_id should always have at least 1 ancestor")
                    .get()
            })
            .flat_map(|frame| frame.get_var(name))
            .next()
    }
}

use std::{hash::Hash, rc::Rc};

use crate::{Command};
use lu_error::SourceCodeItem;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct CommandCollection {
    #[serde(skip)]
    pub cmds: Vec<Rc<dyn Command>>,
    // generic_opts
}

impl Hash for CommandCollection {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name().hash(state)
    }
}

impl CommandCollection {
    pub fn new(cmds: Vec<Rc<dyn Command>>) -> Self {
        assert!(cmds.len() > 0);
        let name = cmds[0].name();
        assert!(
            cmds.iter()
                .map(|cmd| cmd.name())
                .all(|cmd_name| cmd_name == name),
            "All cmds most have same name"
        );
        Self { cmds }
    }

    pub fn name(&self) -> &str {
        self.cmds[0].name()
    }

    pub fn pseudo_decl(&self) -> SourceCodeItem {
        // TODO maybe find first cmd and last cmd and combine these decls
        self.cmds[0].signature().decl.clone()
    }
}

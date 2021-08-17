use log::debug;
use std::collections::HashMap;

use crate::Command;

pub struct CommandStorage {
    cmds: HashMap<String, Box<dyn Command>>,
}

impl CommandStorage {
    pub fn new() -> Self {
        Self {
            cmds: HashMap::new(),
        }
    }

    pub fn insert<S: Into<String>>(&mut self, s: S, cmd: Box<dyn Command>) {
        self.cmds.insert(s.into(), cmd);
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
    ) -> Option<(usize, &dyn Command)> {
        assert!(name_parts.len() > 0);
        // We try to find the longest matching subcommand here ...  Maybe we should use a trie as
        // the internal datastructure
        let mut result = None;
        for i in 0..name_parts.len() {
            let cmd_name = name_parts[0..i + 1].join(" ");
            if let Some(cmd) = self.cmds.get(&cmd_name) {
                result = Some((i + 1, &**cmd))
            }
        }
        debug!("CmdStorage is: {:?}", self.cmds);
        debug!(
            "Match found for cmd_name_parts {:?}: {:?}",
            name_parts,
            result.map_or("None", |(_, cmd)| cmd.name())
        );
        result
    }
}

impl From<HashMap<String, Box<dyn Command>>> for CommandStorage {
    fn from(cmds: HashMap<String, Box<dyn Command>>) -> Self {
        CommandStorage { cmds }
    }
}

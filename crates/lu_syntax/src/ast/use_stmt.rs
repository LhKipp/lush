use std::path::PathBuf;

use lu_syntax_elements::constants::MOD_PATH_PLUGIN_SEP;

use super::{FileNameElement, FileNamePartElement};
use crate::{AstNode, AstToken};

use super::{support, BareWordToken, PluginUseStmtNode, UseStmtNode};

impl UseStmtNode {
    pub fn plugin_path(&self) -> Option<PluginUseStmtNode> {
        support::node_child(&self.syntax())
    }
    pub fn file_path(&self) -> Option<FileNameElement> {
        support::element_child(&self.syntax())
    }
}

impl PluginUseStmtNode {
    pub fn is_std_path(&self) -> bool {
        support::token_child::<BareWordToken>(self.syntax())
            .unwrap()
            .text()
            == "std"
    }
    pub fn as_f_path(&self) -> PathBuf {
        self.to_string().replace(MOD_PATH_PLUGIN_SEP, "/").into()
    }

    pub fn parts(&self) -> Vec<String> {
        support::element_children::<FileNamePartElement>(self.syntax())
            .map(|elem| elem.to_string())
            .collect()
    }
}

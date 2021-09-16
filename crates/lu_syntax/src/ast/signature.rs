#![allow(dead_code)]
#![allow(unused_imports)]
use crate::{AstNode, AstNodeChildren, AstToken};

use super::{
    support, FlagSignatureNode, InSignatureNode, LongFlagToken, LuTypeNode, OptModifierToken,
    ParamNameToken, ParamSignatureNode, RetSignatureNode, ShortFlagToken, SignatureNode,
    VarArgParamSignatureRuleNode,
};

impl SignatureNode {
    pub fn in_arg(&self) -> Option<InSignatureNode> {
        support::node_child(self.syntax())
    }
    pub fn ret_arg(&self) -> Option<RetSignatureNode> {
        support::node_child(self.syntax())
    }
    pub fn args(&self) -> AstNodeChildren<ParamSignatureNode> {
        support::node_children(self.syntax())
    }

    pub fn var_arg(&self) -> Option<VarArgParamSignatureRuleNode> {
        support::node_child(self.syntax())
    }

    pub fn flags(&self) -> AstNodeChildren<FlagSignatureNode> {
        support::node_children::<FlagSignatureNode>(self.syntax())
    }
}

impl ParamSignatureNode {
    pub fn name(&self) -> String {
        let name = support::token_child::<ParamNameToken>(self.syntax());
        name.map(|t| t.text().to_string()).unwrap()
    }

    pub fn type_(&self) -> Option<LuTypeNode> {
        support::node_child::<LuTypeNode>(self.syntax())
    }
}

impl VarArgParamSignatureRuleNode {
    pub fn name(&self) -> String {
        support::token_child::<ParamNameToken>(self.syntax())
            .unwrap()
            .text()
            .to_string()
    }

    pub fn type_(&self) -> Option<LuTypeNode> {
        support::node_child(self.syntax())
    }
}

impl FlagSignatureNode {
    pub fn long_name(&self) -> Option<String> {
        support::token_child::<LongFlagToken>(self.syntax()).map(|flag| flag.text().to_string())
    }
    pub fn short_name(&self) -> Option<char> {
        let short_name = support::token_child::<ShortFlagToken>(self.syntax())
            .map(|flag| flag.text().to_string());
        if let Some(mut name) = short_name {
            assert!(name.chars().count() == 1);
            name.pop()
        } else {
            None
        }
    }
    pub fn type_(&self) -> Option<LuTypeNode> {
        support::node_child(self.syntax())
    }
}

impl RetSignatureNode {
    pub fn type_(&self) -> Option<LuTypeNode> {
        support::node_child(self.syntax())
    }
}

impl InSignatureNode {
    pub fn type_(&self) -> Option<LuTypeNode> {
        support::node_child(self.syntax())
    }
}

use crate::{AstNode, AstNodeChildren, AstToken};
use lu_syntax_elements::constants::{IN_ARG_NAME, RET_ARG_NAME, VAR_ARG_START};

use super::{
    support, ArgNameToken, ArgSignatureNode, FlagSignatureNode, LongFlagToken, LuTypeNode,
    ReqKeywordToken, ShortFlagToken, SignatureNode,
};

impl SignatureNode {
    pub fn in_arg(&self) -> Option<ArgSignatureNode> {
        support::node_children::<ArgSignatureNode>(self.syntax())
            .filter(|n| n.name() == IN_ARG_NAME)
            .next()
    }
    pub fn ret_arg(&self) -> Option<ArgSignatureNode> {
        support::node_children::<ArgSignatureNode>(self.syntax())
            .filter(|n| n.name() == RET_ARG_NAME)
            .next()
    }
    pub fn args(&self) -> Vec<ArgSignatureNode> {
        support::node_children::<ArgSignatureNode>(self.syntax())
            .filter(|n| {
                let name = n.name();
                !n.is_var_arg() && name != RET_ARG_NAME && name != IN_ARG_NAME
            })
            .collect()
    }

    pub fn var_arg(&self) -> Option<ArgSignatureNode> {
        support::node_children::<ArgSignatureNode>(self.syntax())
            .filter(|n| n.is_var_arg())
            .next()
    }

    pub fn flags(&self) -> AstNodeChildren<FlagSignatureNode> {
        support::node_children::<FlagSignatureNode>(self.syntax())
    }
}

impl ArgSignatureNode {
    pub fn name(&self) -> String {
        let name = support::token_child::<ArgNameToken>(self.syntax());
        name.map(|t| t.text().trim_start_matches(VAR_ARG_START).to_string())
            .unwrap()
    }

    pub fn is_var_arg(&self) -> bool {
        let name = support::token_child::<ArgNameToken>(self.syntax());
        name.map(|t| t.text().starts_with(VAR_ARG_START)).unwrap()
    }

    pub fn type_(&self) -> Option<LuTypeNode> {
        support::node_child::<LuTypeNode>(self.syntax())
    }
}

impl FlagSignatureNode {
    pub fn long_name(&self) -> Option<String> {
        support::token_child::<LongFlagToken>(self.syntax()).map(|flag| flag.flag_name())
    }
    pub fn short_name(&self) -> Option<char> {
        support::token_child::<ShortFlagToken>(self.syntax()).map(|flag| flag.flag_name())
    }
    pub fn type_(&self) -> Option<LuTypeNode> {
        support::node_child(self.syntax())
    }

    pub fn is_required(&self) -> bool {
        support::token_child::<ReqKeywordToken>(self.syntax()).is_some()
    }
}

impl ShortFlagToken {
    pub fn flag_name(&self) -> char {
        assert!(
            self.text().len() == 2,
            "TODO handle short flags containing multiple flags"
        );
        self.text().chars().nth(1).unwrap()
    }
}

impl LongFlagToken {
    pub fn flag_name(&self) -> String {
        self.text()[2..].to_string()
    }
}

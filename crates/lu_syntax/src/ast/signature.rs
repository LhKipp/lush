#![allow(dead_code)]
#![allow(unused_imports)]
use lu_value::ValueType;

use crate::{ArgSignature, AstNode, AstToken, FlagSignature, Signature, VarArgSignature};

use super::{
    support, FlagSignatureNode, LongFlagToken, LuTypeNode, OptModifierToken, ParamNameToken,
    ParamSignatureNode, ShortFlagToken, SignatureNode, VarArgParamSignatureRuleNode,
};

impl SignatureNode {
    // pub fn to_signature(&self) -> Option<Signature> {
    // Signature::new()
    // }

    // pub fn args(&self) -> Vec<ArgSignature> {
    //     support::node_children::<ParamSignatureNode>(self.syntax())
    //         .map(|n| n.to_arg_signature())
    //         .collect()
    // }

    // pub fn var_arg(&self) -> Option<VarArgSignature> {
    //     support::node_child::<VarArgParamSignatureRuleNode>(self.syntax())
    //         .map(|n| n.to_var_arg_signature())
    // }

    // pub fn flags(&self) -> Vec<FlagSignature> {
    //     support::node_children::<FlagSignatureNode>(self.syntax())
    //         .map(|n| n.to_flag_signature())
    //         .collect()
    // }
}

// impl ParamSignatureNode {
//     pub fn to_arg_signature(&self) -> ArgSignature {
//         let name = support::token_child::<ParamNameToken>(self.syntax())
//             .unwrap()
//             .text()
//             .to_string();
//         let is_opt = support::token_child::<OptModifierToken>(self.syntax()).is_some();
//         let type_ = support::node_child::<LuTypeNode>(self.syntax())
//             .map(|n| n.to_type())
//             .unwrap(ValueType::Unspecified);

//         ArgSignature::new(name, type_, is_opt)
//     }
// }

// impl VarArgParamSignatureRuleNode {
//     pub fn to_var_arg_signature(&self) -> VarArgSignature {
//         let name = support::token_child::<ParamNameToken>(self.syntax())
//             .unwrap()
//             .text()
//             .to_string();
//         let type_ = support::token_child::<LuTypeNode>(self.syntax())
//             .map(LuTypeNode::to_type)
//             .unwrap(ValueType::Unspecified);

//         VarArgSignature::new(name, type_)
//     }
// }

// impl FlagSignatureNode {
//     pub fn to_flag_signature(&self) -> FlagSignature {
//         let long_name = support::token_child::<LongFlagToken>(self.syntax())
//             .map(|flag| flag.text().to_string());
//         let short_name = support::token_child::<ShortFlagToken>(self.syntax())
//             .map(|flag| flag.text().to_string());
//         let type_ = support::token_child::<LuTypeNode>(self.syntax())
//             .map(LuTypeNode::to_type)
//             .unwrap(ValueType::Unspecified);

//         FlagSignature::new(long_name, short_name, type_)
//     }
// }

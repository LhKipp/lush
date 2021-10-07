#![allow(unused_imports)]
use crate::{eval_function, Command, ScopeFrameId};
use lu_parser::grammar::SourceFileRule;
use lu_text_util::SourceCode;
use std::{
    path::{Path, PathBuf},
    rc::Rc,
};
use walkdir::WalkDir;

use log::debug;
use lu_error::{EvalErr, FsErr, LuResult, SourceCodeItem};
use lu_interpreter_structs::Value;
use lu_pipeline_stage::{ErrorContainer, PipelineStage};
use lu_syntax::{
    ast::{
        BlockStmtNode, FnStmtNode, IfStmtNode, LuTypeNode, SignatureNode, StatementElement,
        StrctStmtNode,
    },
    ast::{ConditionElement, IfBlockNode, SourceFileNode, UseStmtNode},
    AstElement, AstNode, AstToken, Parse,
};
use lu_syntax_elements::{
    constants::{IN_ARG_NAME, RET_ARG_NAME},
    BlockType,
};

use crate::{
    resolve::{Resolve, ResolveArg, Resolver},
    visit_arg::VisitArg,
    ArgDecl, ArgSignature, EvalArg, Evaluable, FlagSignature, Function, Interpreter, ScopeFrameTag,
    Signature, Strct, StrctField, ValueType, Variable,
};

impl Resolve for BlockStmtNode {
    fn do_resolve_dependant_names(&self, _: &[ResolveArg], _: &mut Resolver) {
        panic!()
    }
}

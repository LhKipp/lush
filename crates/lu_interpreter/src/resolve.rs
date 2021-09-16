#![allow(unused_imports)]

mod block_stmt;
mod source_file;

use std::fmt::Debug;
use std::path::PathBuf;
use std::sync::Arc;
use std::{collections::HashMap, rc::Rc};

use log::debug;
use parking_lot::Mutex;
use rusttyc::{TcErr, TcKey, VarlessTypeChecker};

use lu_error::{LuErr, TyErr};
use lu_pipeline_stage::ErrorContainer;
use lu_pipeline_stage::PipelineStage;
use lu_syntax::ast::{CmdStmtNode, SourceFileNode, ValuePathExprNode};
use lu_syntax::Parse;
use lu_syntax_elements::BlockType;

use crate::visit_arg::VisitArg;
use crate::{FlagSignature, Function, Scope, Variable};
use crate::{ScopeFrameTag, ValueType};

pub trait Resolve: Debug {
    fn do_resolve_dependant_names(&self, args: &[ResolveArg], resolver: &mut Resolver);

    fn resolve_dependant_names(&self, resolver: &mut Resolver) {
        self.resolve_dependant_names_with_args(&[], resolver)
    }

    fn resolve_dependant_names_with_args(&self, args: &[ResolveArg], resolver: &mut Resolver) {
        debug!("Resolving dependant names in: {:?}({:?})", self, args);
        let result = self.do_resolve_dependant_names(args, resolver);
        debug!(
            "Result of resolving dependant names: {:?}({:?}): {:?}",
            self,
            args,
            // TODO better debug stmt
            resolver
        );
        result
    }
}

#[derive(Educe)]
#[educe(Debug)]
pub struct Resolver {
    pub parse: Parse,
    #[educe(Debug(ignore))]
    pub scope: Arc<Mutex<Scope<Variable>>>,
    pub errors: Vec<LuErr>,
}

#[derive(Clone, Debug)]
pub enum ResolveArg {
    Arg(VisitArg),
}

impl Resolver {
    pub fn new(parse: Parse, scope: Arc<Mutex<Scope<Variable>>>) -> Self {
        Self {
            parse,
            scope,
            errors: Vec::new(),
        }
    }

    pub(crate) fn all_errors(&self) -> Vec<LuErr> {
        let mut errs = self.parse.all_errors();
        errs.extend(self.errors.clone());
        errs
    }

    pub(crate) fn resolve(&mut self) {
        let source_file = self.parse.cast::<SourceFileNode>().unwrap();
        let source_f_path = self.parse.source.path.clone();

        source_file.resolve_dependant_names_with_args(
            &[ResolveArg::Arg(VisitArg::SourceFilePath(source_f_path))],
            self,
        );
    }

    pub(crate) fn any_failed(&self) -> bool {
        self.parse.any_failed() || !self.errors.is_empty()
    }
}

impl PipelineStage for Resolver {
    fn get_prev_stage(&self) -> Option<&dyn PipelineStage> {
        Some(&self.parse)
    }
}

impl ErrorContainer for Resolver {
    fn get_mut_errors(&mut self) -> &mut Vec<LuErr> {
        &mut self.errors
    }

    fn get_errors(&self) -> &Vec<LuErr> {
        &self.errors
    }
}

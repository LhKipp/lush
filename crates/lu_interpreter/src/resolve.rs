#![allow(unused_imports)]

mod block_stmt;
mod source_file;
mod test;

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
use crate::{FlagSignature, Function, InterpreterCfg, Scope, Variable};
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
    pub parses: Vec<Parse>,
    #[educe(Debug(ignore))]
    pub scope: Arc<Mutex<Scope<Variable>>>,
    pub errors: Vec<LuErr>,

    pub config: Rc<InterpreterCfg>,
}

#[derive(Clone, Debug)]
pub enum ResolveArg {
    Arg(VisitArg),
}

impl Resolver {
    pub fn new(
        parse: Parse,
        scope: Arc<Mutex<Scope<Variable>>>,
        config: Rc<InterpreterCfg>,
    ) -> Self {
        Self {
            parses: vec![parse],
            scope,
            errors: Vec::new(),
            config,
        }
    }

    pub(crate) fn resolve(&mut self) {
        let source_file = self.get_start_parse().cast::<SourceFileNode>().unwrap();
        let source_f_path = self.get_start_parse().source.path.clone();

        source_file.resolve_dependant_names_with_args(
            &[ResolveArg::Arg(VisitArg::SourceFilePath(source_f_path))],
            self,
        );
    }

    pub fn get_start_parse(&self) -> &Parse {
        &self.parses[0]
    }
}

impl PipelineStage for Resolver {
    fn get_prev_stage(&self) -> Option<&dyn PipelineStage> {
        Some(&self.parses[0])
    }

    fn collect_all_errors_cb(&self) -> Vec<LuErr> {
        self.parses
            .iter()
            .map(|parse| parse.get_errors().clone())
            .flatten()
            .collect()
    }

    fn get_mut_errors(&mut self) -> &mut Vec<LuErr> {
        &mut self.errors
    }

    fn get_errors(&self) -> &Vec<LuErr> {
        &self.errors
    }
}

#![allow(unused_imports)]

mod block_stmt;
mod source_file;

use std::fmt::Debug;
use std::sync::Arc;
use std::{collections::HashMap, rc::Rc};

use crate::visit_arg::VisitArg;
use crate::{ScopeFrameTag, ValueType};
use log::debug;
use lu_error::{LuErr, TyErr};
use lu_syntax::ast::{CmdStmtNode, SourceFileNode, ValuePathExprNode};
use lu_syntax_elements::BlockType;
use parking_lot::Mutex;
use rusttyc::{TcErr, TcKey, VarlessTypeChecker};

use crate::{FlagSignature, Function, Scope, Variable};

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
///     Bringing all custom types, funcs into scope ==> Returns: Scope<ResoElem>
pub struct Resolver {
    #[educe(Debug(ignore))]
    pub scope: Arc<Mutex<Scope<Variable>>>,
    pub errors: Vec<LuErr>,
}

#[derive(Clone, Debug)]
pub enum ResolveArg {
    Arg(VisitArg),
}

impl Resolver {
    pub fn new(scope: Arc<Mutex<Scope<Variable>>>) -> Self {
        Self {
            scope,
            errors: Vec::new(),
        }
    }
    pub(crate) fn resolve(&mut self, source_file: &SourceFileNode, source_file_name: String) {
        source_file.resolve_dependant_names_with_args(
            &[ResolveArg::Arg(VisitArg::SourceFileName(source_file_name))],
            self,
        );
    }

    pub(crate) fn ok_or_record_err(&mut self, ty: Result<ValueType, LuErr>) -> ValueType {
        match ty {
            Ok(t) => t,
            Err(e) => {
                self.errors.push(e);
                ValueType::Error
            }
        }
    }
}

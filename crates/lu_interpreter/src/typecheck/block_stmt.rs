#![allow(unused_imports)]
use contracts::ensures;
use log::{debug, warn};
use lu_error::{EvalErr, LuResult, SourceCodeItem};
use lu_interpreter_structs::Value;
use lu_pipeline_stage::PipelineStage;
use lu_syntax::{
    ast::{BlockStmtNode, IfStmtNode, StatementElement},
    ast::{ConditionElement, IfBlockNode},
    AstElement, AstToken,
};
use rusttyc::TcKey;

use crate::{
    visit_arg::VisitArg, EvalArg, Evaluable, Function, Interpreter, ScopeFrameTag, TcFunc,
    TyCheckState, TypeCheckArg, Variable,
};

use super::TypeCheck;

impl TypeCheck for BlockStmtNode {
    fn do_typecheck(
        &self,
        args: &[super::TypeCheckArg],
        ty_state: &mut TyCheckState,
    ) -> Option<TcKey> {
        let frame_to_pop = match args.get(0) {
            Some(TypeCheckArg::Arg(VisitArg::SourceFileBlock(_))) => {
                // if let Err(e) = ty_state.scope.set_cur_source_frame(f_path) {
                //     warn!("SourceFileBlock type check error which should not happen");
                //     ty_state.push_err(e);
                //     return None;
                // }
                None
            }
            Some(TypeCheckArg::Arg(VisitArg::BlockTypeArg(b_type))) => {
                ty_state.scope.push_frame(b_type.clone());
                Some(b_type)
            }
            _ => {
                warn!("Ty checking BlockStmt without args");
                None
            }
        };

        let mut result = None;
        for stmt in self.statements() {
            result = stmt.typecheck(ty_state);
        }

        if let Some(to_pop) = frame_to_pop {
            ty_state.scope.pop_frame(&to_pop);
        }

        result
    }
}

use log::{warn};
use lu_interpreter_structs::{FlagVariant, ValueType};
use lu_syntax::{
    ast::{PipedCmdsStmtNode, ValueExprElement},
    AstElement,
};
use rusttyc::TcKey;

use crate::{TypeCheck, TypeCheckArg};

impl TypeCheck for PipedCmdsStmtNode {
    fn do_typecheck(
        &self,
        _args: &[TypeCheckArg],
        ty_state: &mut crate::TyCheckState,
    ) -> Option<TcKey> {
        let first_in_key = ty_state.new_term_key_concretiziesd(
            self.piped_args().next().unwrap().to_item(),
            ValueType::Nil,
        );

        let mut ret_key = first_in_key;
        for cmd in self.piped_args() {
            if let ValueExprElement::CmdStmt(cmd) = &cmd {
                let cmd_name = cmd.get_cmd_name();
                let passed_flags = FlagVariant::convert(cmd.get_passed_flags());
                if let Some(cmd_keys) = ty_state.get_tc_cmd_from_cmd_usage(&cmd_name, &passed_flags)
                {
                    ty_state.equate_keys(ret_key, cmd_keys.in_key);
                } // else its an external cmd, no equating of in_keys necessary
            } else {
                warn!("Not equating in key for fn like math expr");
                // in a pipeline like `[1]| map $plus_1` [1] should not be key equated
                // however in a pipeline like `[1] | $in + [2] | ...` the second math expr needs to
                // be equated
            }

            ret_key = cmd
                .typecheck_with_args(
                    &[TypeCheckArg::CmdStmt {
                        in_piped_arg_key: ret_key,
                    }],
                    ty_state,
                )
                .expect("Cmd always returns");
        }
        Some(ret_key)
    }
}

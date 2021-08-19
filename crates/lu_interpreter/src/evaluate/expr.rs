use lu_error::{EvalErr, LuResult, SourceCodeItem};
use lu_syntax::{
    ast::{
        ArrayExprNode, BareWordToken, MathExprNode, NumberToken, StringExprNode, TableExprNode,
        ValueExprNode, ValuePathExprNode,
    },
    AstNode, AstToken,
};
use lu_value::Value;

use crate::Evaluable;

impl Evaluable for ValueExprNode {
    fn evaluate(&self, state: &mut crate::Interpreter) -> LuResult<Value> {
        match self {
            ValueExprNode::BareWord(n) => n.evaluate(state),
            ValueExprNode::Number(n) => n.evaluate(state),
            ValueExprNode::MathExpr(n) => n.evaluate(state),
            ValueExprNode::StringExpr(n) => n.evaluate(state),
            ValueExprNode::ValuePathExpr(n) => n.evaluate(state),
            ValueExprNode::ArrayExpr(n) => n.evaluate(state),
            ValueExprNode::TableExpr(n) => n.evaluate(state),
        }
    }
}

impl Evaluable for BareWordToken {
    fn evaluate(&self, _state: &mut crate::Interpreter) -> LuResult<Value> {
        Ok(Value::BareWord(self.text().to_string()))
    }
}

impl Evaluable for NumberToken {
    fn evaluate(&self, _state: &mut crate::Interpreter) -> LuResult<Value> {
        Ok(self.value())
    }
}

impl Evaluable for MathExprNode {
    fn evaluate(&self, _state: &mut crate::Interpreter) -> LuResult<Value> {
        todo!()
    }
}

impl Evaluable for StringExprNode {
    fn evaluate(&self, _state: &mut crate::Interpreter) -> LuResult<Value> {
        Ok(Value::String(self.text().to_string()))
    }
}

impl Evaluable for ValuePathExprNode {
    fn evaluate(&self, state: &mut crate::Interpreter) -> LuResult<Value> {
        let name_parts = self.var_name_parts();
        assert_eq!(name_parts.len(), 1); // TODO handle indexing into table
        if let Some(var) = state.scope.lock().get_var(&name_parts[0]) {
            Ok(var.clone())
        } else {
            EvalErr::VarNotFound(SourceCodeItem::new(
                self.syntax().text_range().into(),
                self.syntax().text().to_string(),
            ))
            .into()
        }
    }
}

impl Evaluable for ArrayExprNode {
    fn evaluate(&self, state: &mut crate::Interpreter) -> LuResult<Value> {
        let mut values = Vec::new();
        for val in self.values() {
            values.push(val.evaluate(state)?);
        }
        Ok(Value::new_array(values))
    }
}

impl Evaluable for TableExprNode {
    fn evaluate(&self, _state: &mut crate::Interpreter) -> LuResult<Value> {
        todo!()
    }
}

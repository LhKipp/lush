use lu_error::LuResult;
use lu_syntax::{
    ast::{
        ArrayExprNode, BareWordToken, ExpressionNode, MathExprNode, NumberToken, StringExprNode,
        TableExprNode, ValuePathExprNode,
    },
    AstToken,
};
use lu_value::Value;

use crate::Evaluable;

impl Evaluable for ExpressionNode {
    fn evaluate(&self, state: &mut crate::Interpreter) -> LuResult<Value> {
        match self {
            ExpressionNode::BareWord(n) => n.evaluate(state),
            ExpressionNode::Number(n) => n.evaluate(state),
            ExpressionNode::MathExpr(n) => n.evaluate(state),
            ExpressionNode::StringExpr(n) => n.evaluate(state),
            ExpressionNode::ValuePathExpr(n) => n.evaluate(state),
            ExpressionNode::ArrayExpr(n) => n.evaluate(state),
            ExpressionNode::TableExpr(n) => n.evaluate(state),
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
        // TODO parsing of number as binary num (0b1110), hex (0xF) or decimal
        let val: f64 = self
            .text()
            .parse()
            .map_err(|_| "Error parsing to number".to_string())?;
        Ok(Value::Number(val))
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
    fn evaluate(&self, _state: &mut crate::Interpreter) -> LuResult<Value> {
        todo!()
    }
}

impl Evaluable for ArrayExprNode {
    fn evaluate(&self, state: &mut crate::Interpreter) -> LuResult<Value> {
        let mut values = Vec::new();
        for val in self.values() {
            values.push(val.evaluate(state)?);
        }
        Ok(Value::Array(values))
    }
}

impl Evaluable for TableExprNode {
    fn evaluate(&self, _state: &mut crate::Interpreter) -> LuResult<Value> {
        todo!()
    }
}

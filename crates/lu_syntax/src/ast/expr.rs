use crate::{AstElementChildren, AstNode, AstToken};

use super::{
    support, ArrayExprNode, BareWordToken, MathExprNode, NumberToken, StringContentToken,
    StringExprNode, TableExprNode, ValueExprNode, ValuePathExprNode,
};
use lu_value::Value;

impl ValueExprNode {
    pub fn value(&self) -> Value {
        match self {
            ValueExprNode::BareWord(n) => n.value(),
            ValueExprNode::Number(n) => n.value(),
            ValueExprNode::MathExpr(n) => n.value(),
            ValueExprNode::StringExpr(n) => n.value(),
            ValueExprNode::ValuePathExpr(n) => n.value(),
            ValueExprNode::ArrayExpr(n) => n.value(),
            ValueExprNode::TableExpr(n) => n.value(),
        }
    }
}

impl MathExprNode {
    pub fn value(&self) -> Value {
        todo!()
    }
}

impl NumberToken {
    pub fn value(&self) -> Value {
        // TODO parsing of number as binary num (0b1110), hex (0xF) or decimal
        let val: f64 = self
            .text()
            .parse()
            .expect(&format!("Error parsing {} to a number", self.text()));
        Value::Number(val.into())
    }
}

impl TableExprNode {
    pub fn value(&self) -> Value {
        todo!()
    }
}

impl ValuePathExprNode {
    pub fn value(&self) -> Value {
        todo!()
    }
}

impl BareWordToken {
    pub fn value(&self) -> Value {
        Value::BareWord(self.text().to_string())
    }
}

impl ArrayExprNode {
    pub fn value(&self) -> Value {
        Value::new_array(self.values().map(|n| n.value()).collect())
    }
    pub fn values(&self) -> AstElementChildren<ValueExprNode> {
        support::element_children(self.syntax())
    }
}

impl StringExprNode {
    pub fn value(&self) -> Value {
        Value::String(self.text())
    }
    pub fn text(&self) -> String {
        support::token_child::<StringContentToken>(self.syntax())
            .unwrap()
            .text()
            .to_string()
    }
}

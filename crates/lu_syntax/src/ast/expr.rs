use crate::{AstElementChildren, AstNode, AstToken};

use super::{
    support, ArrayExprNode, BareWordToken, NumberExprNode, NumberToken, StringContentToken,
    StringExprNode, ValueExprElement,
};

impl NumberExprNode {
    pub fn value(&self) -> f64 {
        support::token_child::<NumberToken>(self.syntax())
            .unwrap()
            .value()
    }
}

impl NumberToken {
    pub fn value(&self) -> f64 {
        // TODO parsing of number as binary num (0b1110), hex (0xF) or decimal
        self.text()
            .parse::<f64>()
            .expect(&format!("Error parsing {} to a number", self.text()))
    }
}

// impl TableExprNode {
//     pub fn value(&self) -> Value {
//         todo!()
//     }
// }

// impl ValuePathExprNode {
//     pub fn value(&self) -> Value {
//         todo!()
//     }
// }

impl BareWordToken {
    pub fn value(&self) -> String {
        self.text().to_string()
    }
}

impl ArrayExprNode {
    pub fn values(&self) -> AstElementChildren<ValueExprElement> {
        support::element_children(self.syntax())
    }
}

impl StringExprNode {
    pub fn value(&self) -> String {
        self.text()
    }
    pub fn text(&self) -> String {
        support::token_child::<StringContentToken>(self.syntax())
            .unwrap()
            .text()
            .to_string()
    }
}

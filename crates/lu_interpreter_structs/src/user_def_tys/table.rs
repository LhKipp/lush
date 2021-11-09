// use derive_new::new;
// use lu_error::{SourceCodeItem, util::Outcome};
// use lu_syntax::ast::TableExprNode;

// use crate::{Value, ValueType};
// use serde::{Deserialize, Serialize};

// #[derive(new, PartialEq, Eq, Hash, Clone, Debug, Serialize, Deserialize)]
// pub struct Table {
//     pub sign: TableSignature,
//     pub rows: Vec<Vec<(Value, SourceCodeItem)>>,
//     pub decl: SourceCodeItem,
// }

// impl Table {
//     pub fn from_node(tbl_node: &TableExprNode) -> Outcome<Table> {
//         let sign = if let Some(sign_node) = tbl_node.tbl_signature(){
//             TableSignature::from_node(sign_node)
//         }else{
//             TableSignature::empty()
//         };

//         for row in tbl_node.rows(){
//             for value in row.values(){
//                 let val = Value::
//             }
//         }
//     }
// }

// pub struct TableColSignature{
//     pub name: String,
//     pub ty: ValueType,
//     pub decl: SourceCodeItem,
// }

// pub struct TableSignature{
//     pub col_tys: Vec<TableColSignature>
// }

// impl TableSignature{
//     pub fn from_node(sign_node: &TableSignatureNode){

//     }
//     pub fn empty() ->Self{
//         TableSignature{
//             col_tys: vec![],
//         }
//     }
// }

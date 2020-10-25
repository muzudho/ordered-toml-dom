//! Expression model.  
//! 縦幅のある行 モデル。  

use crate::model::{
    layer210::Comment,
    layer225::KeyValue,
    layer230::{Expression, HeaderOfArrayOfTable, HeaderOfTable},
};
use std::fmt;

impl Expression {
    pub fn from_header_of_array_of_table(m: &HeaderOfArrayOfTable) -> Self {
        Expression::HeaderOfArrayOfTable(m.clone())
    }
    pub fn from_comment(m: &Comment) -> Self {
        Expression::Comment(m.clone())
    }
    pub fn from_key_value(m: &KeyValue) -> Self {
        Expression::KeyValue(m.clone())
    }
    pub fn from_header_of_table(m: &HeaderOfTable) -> Self {
        Expression::HeaderOfTable(m.clone())
    }
    pub fn to_debug_string(&self) -> String {
        format!("{:?}", self)
    }
    pub fn to_string(&self) -> String {
        format!("{}", self)
    }
}
impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::HeaderOfArrayOfTable(m) => write!(f, "{}", m),
            Expression::Comment(m) => write!(f, "{}", m),
            Expression::EmptyLine(ws, comment) => write!(
                f,
                "{}{}",
                ws,
                if let Some(comment) = comment {
                    comment.to_string()
                } else {
                    "".to_string()
                }
            ),
            Expression::KeyValue(m) => write!(f, "{}", m),
            Expression::HeaderOfTable(m) => write!(f, "{}", m),
        }
    }
}
impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::HeaderOfArrayOfTable(m) => write!(f, "{:?}", m),
            Expression::Comment(m) => write!(f, "{:?}", m),
            Expression::EmptyLine(ws, comment) => write!(f, "{:?}{:?}", ws, comment),
            Expression::KeyValue(m) => write!(f, "{:?}", m),
            Expression::HeaderOfTable(m) => write!(f, "{:?}", m),
        }
    }
}

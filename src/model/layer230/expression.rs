//! Expression model.  
//! 縦幅のある行 モデル。  

use crate::model::layer230::Ws;
use crate::model::{
    layer210::Comment,
    layer225::Keyval,
    layer230::{Expression, HeaderOfArrayOfTable, HeaderOfTable},
};
use std::fmt;

impl Expression {
    pub fn from_header_of_array_of_table(m: &HeaderOfArrayOfTable) -> Self {
        Expression::HeaderOfArrayOfTable(m.clone())
    }
    pub fn from_empty_line(ws: &Ws, comment: &Comment) -> Self {
        Expression::EmptyLine(ws.clone(), Some(comment.clone()))
    }
    pub fn from_keyval(ws1: &Ws, keyval: &Keyval, ws2: &Ws, comment: &Comment) -> Self {
        Expression::Keyval(
            ws1.clone(),
            keyval.clone(),
            ws2.clone(),
            Some(comment.clone()),
        )
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
            Expression::Keyval(ws1, keyval, ws2, comment) => write!(
                f,
                "{}{}{}{}",
                ws1,
                keyval,
                ws2,
                if let Some(comment) = comment {
                    comment.to_string()
                } else {
                    "".to_string()
                }
            ),
            Expression::HeaderOfTable(m) => write!(f, "{}", m),
        }
    }
}
impl fmt::Debug for Expression {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expression::HeaderOfArrayOfTable(m) => write!(f, "{:?}", m),
            Expression::EmptyLine(ws, comment) => write!(f, "{:?}{:?}", ws, comment),
            Expression::Keyval(ws1, keyval, ws2, comment) => {
                write!(f, "{:?}{:?}{:?}{:?}", ws1, keyval, ws2, comment)
            }
            Expression::HeaderOfTable(m) => write!(f, "{:?}", m),
        }
    }
}

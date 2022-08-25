use std::rc::Rc;

use crate::storage::*;

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Expression {
    B(u64, i64), // Base expression: (id of expression, variable name)
    A(u64, u64, u64), // And expression: (id, left variable id, right variable id)
    O(u64, u64, u64), // same as And expressions but Or expressions
}

#[derive(Copy, Clone, PartialEq, PartialOrd)]
pub enum Side {
    Left,
    Right,
}

impl Expression {
    pub fn id(&self) -> u64 {
        use Expression::*;

        match self {
            B(id, _) | A(id, _, _) | O(id, _, _) => *id,
        }
    }

    pub fn sub(&self, s: Side) -> Option<u64> {
        use Side::*;
        match s {
            Left => self.left(),
            Right => self.right(),
        }
    }
    
    pub fn left(&self) -> Option<u64> {
        use Expression::*;

        match self {
            A(_, l, _) |O(_, l, _) => Some(*l),
            _ => None,
        }
    }

    pub fn right(&self) -> Option<u64> {
        use Expression::*;

        match self {
            A(_, _, r) |O(_, _, r) => Some(*r),
            _ => None,
        }
    }
}

pub type Context = Storage<Expression>;

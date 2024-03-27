use std::cell::RefCell;
use std::fmt::{Debug, Display, Error, Formatter};

use serde::{Deserialize, Serialize};

/// Represents an expression in the AST.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Expr {
    /// A numeric literal.
    Number(i32),
    /// An identifier.
    Id(String),
    /// An operation with two operands.
    Op(Box<Expr>, Opcode, Box<Expr>),
    /// An error expression.
    Error,
}

/// Represents the possible rounding modes.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Rounding {
    /// The initial rounding mode.
    Init,
    /// Round up to the nearest value.
    Up,
    /// Round down to the nearest value.
    Down,
    /// An unknown rounding mode.
    Unknown,
}

/// Converts a boolean flag to a rounding mode.
///
/// # Arguments
///
/// * `flag` - A boolean flag indicating whether to round up or down.
///
/// # Returns
///
/// The corresponding rounding mode (`Rounding::Up` if `flag` is `true`, `Rounding::Down` otherwise).
pub fn bool_to_rounding(flag: bool) -> Rounding {
    if flag {
        Rounding::Up
    } else {
        Rounding::Down
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum Opcode {
    // The rounding direction of the mul/div operation will be determined on the fly
    // Might not be the most rust thing to do
    // TODO evaluate different design
    Mul(RefCell<Rounding>),
    Div(RefCell<Rounding>),
    Add,
    Sub,
    Pow,
}

impl Display for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match &self {
            Expr::Number(n) => write!(fmt, "{n}"),
            Expr::Id(ref n) => write!(fmt, "{n}"),
            Expr::Op(ref l, op, ref r) => write!(fmt, "({l} {op} {r})"),
            Expr::Error => write!(fmt, "error"),
        }
    }
}

impl Display for Opcode {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match &self {
            Opcode::Mul(r) => match *r.borrow() {
                Rounding::Init => write!(fmt, "*"),
                Rounding::Up => write!(fmt, "*↑"),
                Rounding::Down => write!(fmt, "*↓"),
                Rounding::Unknown => write!(fmt, "*↕"),
            },

            Opcode::Div(r) => match *r.borrow() {
                Rounding::Init => write!(fmt, "/"),
                Rounding::Up => write!(fmt, "/↑"),
                Rounding::Down => write!(fmt, "/↓"),
                Rounding::Unknown => write!(fmt, "/↕"),
            },
            Opcode::Add => write!(fmt, "+"),
            Opcode::Sub => write!(fmt, "-"),
            Opcode::Pow => write!(fmt, "**"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_number() {
        let expr = Expr::Number(42);
        assert_eq!(format!("{expr}"), "42");
    }

    #[test]
    fn test_display_id() {
        let expr = Expr::Id("x".to_string());
        assert_eq!(format!("{expr}"), "x");
    }

    #[test]
    fn test_display_op() {
        let left = Expr::Number(1);
        let right = Expr::Number(2);
        let op = Opcode::Add;
        let expr = Expr::Op(Box::new(left), op, Box::new(right));
        assert_eq!(format!("{expr}"), "(1 + 2)");
    }

    #[test]
    fn test_display_error() {
        let expr = Expr::Error;
        assert_eq!(format!("{expr}"), "error");
    }

    #[test]
    fn test_display_opcode_mul_init() {
        let op = Opcode::Mul(RefCell::new(Rounding::Init));
        assert_eq!(format!("{op}"), "*");
    }

    #[test]
    fn test_display_opcode_mul_up() {
        let op = Opcode::Mul(RefCell::new(Rounding::Up));
        assert_eq!(format!("{op}"), "*↑");
    }

    #[test]
    fn test_display_opcode_mul_down() {
        let op = Opcode::Mul(RefCell::new(Rounding::Down));
        assert_eq!(format!("{op}"), "*↓");
    }

    #[test]
    fn test_display_opcode_mul_unknown() {
        let op = Opcode::Mul(RefCell::new(Rounding::Unknown));
        assert_eq!(format!("{op}"), "*↕");
    }

    #[test]
    fn test_display_opcode_div_init() {
        let op = Opcode::Div(RefCell::new(Rounding::Init));
        assert_eq!(format!("{op}"), "/");
    }

    #[test]
    fn test_display_opcode_div_up() {
        let op = Opcode::Div(RefCell::new(Rounding::Up));
        assert_eq!(format!("{op}"), "/↑");
    }

    #[test]
    fn test_display_opcode_div_down() {
        let op = Opcode::Div(RefCell::new(Rounding::Down));
        assert_eq!(format!("{op}"), "/↓");
    }

    #[test]
    fn test_display_opcode_div_unknown() {
        let op = Opcode::Div(RefCell::new(Rounding::Unknown));
        assert_eq!(format!("{op}"), "/↕");
    }

    #[test]
    fn test_bool_to_rounding() {
        assert_eq!(bool_to_rounding(true), Rounding::Up);
        assert_eq!(bool_to_rounding(false), Rounding::Down);
    }

    #[test]
    fn test_display_opcode() {
        let rounding = RefCell::new(Rounding::Up);
        let opcode = Opcode::Mul(rounding);
        assert_eq!(format!("{opcode}"), "*↑");

        let rounding = RefCell::new(Rounding::Down);
        let opcode = Opcode::Div(rounding);
        assert_eq!(format!("{opcode}"), "/↓");

        let opcode = Opcode::Add;
        assert_eq!(format!("{opcode}"), "+");

        let opcode = Opcode::Sub;
        assert_eq!(format!("{opcode}"), "-");

        let opcode = Opcode::Pow;
        assert_eq!(format!("{opcode}"), "**");
    }
}

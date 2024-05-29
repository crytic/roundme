use std::cell::RefCell;

use super::ast::{Expr, Opcode, Rounding};

fn is_negative_value(expr: Box<Expr>) -> (bool, Box<Expr>) {
    match *expr {
        Expr::Negative(ne) => match *ne {
            Expr::Negative(internal) => is_negative_value(internal),
            _ => (true, ne),
        }
        // Need to handle this case separately because in case of nested negatives the 
        // inner most operation will not be simplified
        Expr::Op(l, o, r) => {
            let simplified_internal_expr = simplify_sign(Box::new(Expr::Op(l, o, r)));

            match *simplified_internal_expr {
                Expr::Negative(ne) => (true, ne),
                _ => (false, simplified_internal_expr),
            }
        },
        _ => (false, expr),
    }
}

fn simplify_add(lnv: (bool, Box<Expr>), rnv: (bool, Box<Expr>)) -> Box<Expr> {
    let expr = match (lnv.0, rnv.0) {
        (true, false) => Expr::Op(rnv.1, Opcode::Sub, lnv.1),
        (true, true) => Expr::Negative(Box::new(Expr::Op(lnv.1, Opcode::Add, rnv.1))),
        (false, true) => Expr::Op(lnv.1, Opcode::Sub, rnv.1),
        (false, false) => Expr::Op(lnv.1, Opcode::Add, rnv.1),
    };

    Box::new(expr)
}

fn simplify_sub(lnv: (bool, Box<Expr>), rnv: (bool, Box<Expr>)) -> Box<Expr> {
    let expr = match (lnv.0, rnv.0) {
        (true, false) => Expr::Negative(Box::new(Expr::Op(lnv.1, Opcode::Add, rnv.1))),
        (true, true) => Expr::Op(rnv.1, Opcode::Sub, lnv.1),
        (false, true) => Expr::Op(lnv.1, Opcode::Add, rnv.1),
        (false, false) => Expr::Op(lnv.1, Opcode::Sub, rnv.1),
    };

    Box::new(expr)
}

fn simplify_mul(lnv: (bool, Box<Expr>), rnv: (bool, Box<Expr>), r: RefCell<Rounding>) -> Box<Expr> {
    let expr = if lnv.0 ^ rnv.0 {
        Expr::Negative(Box::new(Expr::Op(lnv.1, Opcode::Mul(RefCell::new(Rounding::Init)), rnv.1)))
    } else {
        Expr::Op(lnv.1, Opcode::Mul(r), rnv.1)
    };

    Box::new(expr)
}

fn simplify_div(lnv: (bool, Box<Expr>), rnv: (bool, Box<Expr>), r: RefCell<Rounding>) -> Box<Expr> {
    let expr = if lnv.0 ^ rnv.0 {
        Expr::Negative(Box::new(Expr::Op(lnv.1, Opcode::Div(RefCell::new(Rounding::Init)), rnv.1)))
    } else {
        Expr::Op(lnv.1, Opcode::Div(r), rnv.1)
    };

    Box::new(expr)
}

/// Simplifies the signs to bring the negative sign from values to the operations
/// and finally bring it out to the expression level if possible
/// It also reagganges the addition and substration formula to make them look better
/// with the sign. For example (-a + b) will be re-arranged to (b - a)
pub fn simplify_sign(expr: Box<Expr>) -> Box<Expr> {
    match *expr {
        Expr::Op(left, op, right) => {
            let simplified_left = match *left {
                Expr::Op(..) => simplify_sign(left),
                _ => left
            };

            let simplified_right = match *right {
                Expr::Op(..) => simplify_sign(right),
                _ => right
            };

            let lnv = is_negative_value(simplified_left);
            let rnv = is_negative_value(simplified_right);

            match op {
                Opcode::Add => simplify_add(lnv, rnv),
                Opcode::Sub => simplify_sub(lnv, rnv),
                Opcode::Mul(r) => simplify_mul(lnv, rnv, r),
                Opcode::Div(r) => simplify_div(lnv, rnv, r),
                Opcode::Pow => Box::new(Expr::Op(lnv.1, op, rnv.1))
            }
        },
        _ => expr
    }
}
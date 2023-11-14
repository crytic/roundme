use anyhow::Result;
use std::cell::RefCell;

use crate::ast::bool_to_rounding;
use crate::ast::Expr;
use crate::ast::Opcode;
use crate::ast::Rounding;
use crate::config;
use crate::utils;

// Mulplication
// Up -> A * B -> A up, B up, * up
// Down  -> A * B -> A down, B down, * down
fn handle_mul(rounding_direction: bool, op_rounding: &RefCell<Rounding>) -> (bool, bool) {
    *op_rounding.borrow_mut() = bool_to_rounding(rounding_direction);
    (rounding_direction, rounding_direction)
}

// Div
// Up -> A / B -> A up, B down, / up
// Down -> A / B -> A down, B up, / down
fn handle_div(rounding_direction: bool, op_rounding: &RefCell<Rounding>) -> (bool, bool) {
    *op_rounding.borrow_mut() = bool_to_rounding(rounding_direction);
    (rounding_direction, !rounding_direction)
}

// Pow
// a ** B
// Up: if A >=1  -> A up, b up
// Up: if A <1  -> A up, b down
// Down: if A >=1  -> A down, b down
// Down: if A <1  -> A down, b up
fn handle_pow(
    left: &Expr,
    rounding_direction: bool,
    config: &mut config::Config,
) -> Result<(bool, bool)> {
    let expr_str = format!("{left}");

    if config
        .less_than_one
        .as_ref()
        .map_or(false, |vec| vec.contains(&expr_str))
    {
        return Ok((rounding_direction, !rounding_direction));
    }

    if config
        .greater_than_one
        .as_ref()
        .map_or(false, |vec| vec.contains(&expr_str))
    {
        return Ok((rounding_direction, rounding_direction));
    }

    println!("Is {} greater than 1? Y/N (yes, no)", &left);

    if utils::ask_yes_no()? {
        config.add_greater_than_one(expr_str);
        Ok((rounding_direction, rounding_direction))
    } else {
        config.add_less_than_one(expr_str);
        Ok((rounding_direction, !rounding_direction))
    }
}

/// This function visits an expression and analyze the rounding direction
/// for arithmetic operations based on the given configuration.
///
/// # Arguments
///
/// * `expr` - An expression to be visited.
/// * `rounding_direction` - A boolean value indicating the rounding direction for arithmetic operations.
/// * `config` - A reference to a configuration object containing information about rounding.
///
/// # Returns
///
/// This function returns a `Result` object with an empty Ok value if the operation is successful.
///
/// # Example
///
/// ```
/// use analyze_rounding::visit;
/// use config::Config;
///
/// let expr = Expr::Number(5);
/// let config = Config::new();
/// visit(&expr, true, &config);
/// ```
fn visit(expr: &Expr, rounding_direction: bool, config: &mut config::Config) -> Result<()> {
    match expr {
        Expr::Number(_) | Expr::Id(_) | Expr::Error => (),
        Expr::Op(left, op, right) => {
            let (left_rounding, right_rounding) = match op {
                Opcode::Add => (rounding_direction, rounding_direction),
                Opcode::Sub => (rounding_direction, !rounding_direction),
                Opcode::Mul(op_rounding) => handle_mul(rounding_direction, op_rounding),
                Opcode::Div(op_rounding) => handle_div(rounding_direction, op_rounding),
                Opcode::Pow => handle_pow(left, rounding_direction, config)?,
            };
            visit(left, left_rounding, config)?;
            visit(right, right_rounding, config)?;
        }
    };
    Ok(())
}

/// Analyzes the given expression for rounding and returns the result.
///
/// # Arguments
///
/// * `expr` - The expression to analyze.
/// * `rounding_direction` - The direction of rounding to use.
/// * `config` - The configuration to use for the analysis.
///
/// # Returns
///
/// Returns a `Result` indicating whether the analysis was successful or not.
pub fn analyze(expr: &Expr, rounding_direction: bool, config: &mut config::Config) -> Result<()> {
    visit(expr, rounding_direction, config)
}

use std::io::{self, Write};

use crate::analyzer::ast::{Expr, Opcode};

use super::arithmetic;
use super::FormulaConfig;

pub fn ask_user_formula_config() -> anyhow::Result<FormulaConfig> {
    let (formula_user, expr) = ask_formula()?;

    let rounding = ask_rounding()?;

    let mut formula_config = FormulaConfig {
        formula: formula_user,
        round_up: rounding,
        less_than_one: None,
        greater_than_one: None,
    };

    find_less_greater_than_one(&expr, &mut formula_config);

    Ok(formula_config)
}

fn ask_formula() -> anyhow::Result<(String, Box<Expr>)> {
    let mut input = String::new();

    // Ask the question
    println!("Formula to analyze: ");

    // Flush to make sure the question is displayed before getting input
    io::stdout().flush()?;

    // Get user input
    io::stdin().read_line(&mut input)?;

    // Trim the input and convert it to lowercase
    let formula = input.trim().to_lowercase();
    let parse_expression = arithmetic::ExprParser::new();

    match parse_expression.parse(&formula) {
        Ok(ast) => Ok((formula, ast)),
        Err(err) => {
            println!("Can't parse the expression: {err}");
            println!("Make sure to:");
            println!("- Have the correct number of parenthesis");
            println!("- Do not use number in the ID name (ex: do not name a variable a0)");
            println!("- Use ** for power (and not ^)");
            ask_formula()
        }
    }
}

fn ask_rounding() -> anyhow::Result<bool, io::Error> {
    println!("Should the formula round up? Y/N (yes, no)");
    ask_yes_no()
}

/// Asks the user to input a yes or no answer and returns true or false.
/// If the input is invalid, it recursively asks the user again until a valid input is received.
pub fn ask_yes_no() -> Result<bool, io::Error> {
    let mut input: String = String::new();

    // Flush to make sure the question is displayed before getting input
    io::stdout().flush()?;

    // Get user input
    io::stdin().read_line(&mut input)?;

    // Trim the input and convert it to lowercase
    let input = input.trim().to_lowercase();

    // Parse the input
    let parsed_input = match input.as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => {
            println!("Invalid input. Please enter Y, N.");
            return ask_yes_no(); // Recursively ask again if input is invalid
        }
    };

    Ok(parsed_input)
}

fn find_less_greater_than_one(expr: &Expr, formula_config: &mut FormulaConfig) {
    visit(expr, formula_config);
}

fn visit(expr: &Expr, formula_config: &mut FormulaConfig) {
    match expr {
        Expr::Number(_) | Expr::Id(_) | Expr::Error => (),
        Expr::Op(left, op, right) => {
            if let Opcode::Pow = op {
                // We ignore if the following fail
                let _ = handle_pow(left, formula_config);
            }
            visit(left, formula_config);
            visit(right, formula_config);
        }
    };
}

fn handle_pow(left: &Expr, formula_config: &mut FormulaConfig) -> anyhow::Result<()> {
    let expr_str = format!("{left}");

    if formula_config
        .less_than_one
        .as_ref()
        .map_or(false, |vec| vec.contains(&expr_str))
    {
        return Ok(());
    }

    if formula_config
        .greater_than_one
        .as_ref()
        .map_or(false, |vec| vec.contains(&expr_str))
    {
        return Ok(());
    }

    println!("Is {} greater than 1? Y/N (yes, no)", &left);

    if ask_yes_no()? {
        formula_config.add_greater_than_one(expr_str);
    } else {
        formula_config.add_less_than_one(expr_str);
    };
    Ok(())
}

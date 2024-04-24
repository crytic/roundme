mod analyze_rounding;
pub mod ast;

use crate::parser::arithmetic;
use crate::FormulaConfig;

use self::ast::Expr;

pub fn analyze(formula_config: &mut FormulaConfig) -> anyhow::Result<Box<Expr>> {
    let parse_expression = arithmetic::ExprParser::new();
    let formula = formula_config.formula.as_str();
    let ast = parse_expression.parse(formula).map_err(|e| {
        anyhow::anyhow!("Error occured while parsing the formula {}: {}", formula, e)
    })?;

    analyze_rounding::analyze(&ast, formula_config.round_up, formula_config)?;

    Ok(ast)
}

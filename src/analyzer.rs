mod analyze_rounding;
mod simplify_expr;
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

    println!("parsed    : {ast}");
    let simplified_ast = simplify_expr::simplify_sign(ast);
    println!("simplified: {simplified_ast}");

    analyze_rounding::analyze(&simplified_ast, formula_config.round_up, formula_config)?;

    Ok(simplified_ast)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mul_up() {
        let mut formula_config = FormulaConfig::new(
            "a * b".to_string(),
            true,
            None,
            None
        );
        let ast = analyze(&mut formula_config).unwrap().to_string();
        assert_eq!(ast, "(a *↑ b)");
    }

    #[test]
    fn test_mul_down() {
        let mut formula_config = FormulaConfig::new(
            "a * b".to_string(),
            false,
            None,
            None
        );
        let ast = analyze(&mut formula_config).unwrap().to_string();
        assert_eq!(ast, "(a *↓ b)");
    }

    #[test]
    fn test_div_up() {
        let mut formula_config = FormulaConfig::new(
            "a / b".to_string(),
            true,
            None,
            None
        );
        let ast = analyze(&mut formula_config).unwrap().to_string();
        assert_eq!(ast, "(a /↑ b)");
    }

    #[test]
    fn test_div_down() {
        let mut formula_config = FormulaConfig::new(
            "a / b".to_string(),
            false,
            None,
            None
        );
        let ast = analyze(&mut formula_config).unwrap().to_string();
        assert_eq!(ast, "(a /↓ b)");
    }

    #[test]
    fn test_pow_greater_than_one_up() {
        let mut formula_config = FormulaConfig::new(
            "a ** (b * c)".to_string(),
            true,
            None,
            Some(vec!["a".to_string()])
        );
        let ast = analyze(&mut formula_config).unwrap().to_string();
        assert_eq!(ast, "(a ** (b *↑ c))");
    }

    #[test]
    fn test_pow_less_than_one_up() {
        let mut formula_config = FormulaConfig::new(
            "a ** (b * c)".to_string(),
            true,
            Some(vec!["a".to_string()]),
            None
        );
        let ast = analyze(&mut formula_config).unwrap().to_string();
        assert_eq!(ast, "(a ** (b *↓ c))");
    }

    #[test]
    fn test_pow_greater_than_one_down() {
        let mut formula_config = FormulaConfig::new(
            "a ** (b * c)".to_string(),
            false,
            None,
            Some(vec!["a".to_string()])
        );
        let ast = analyze(&mut formula_config).unwrap().to_string();
        assert_eq!(ast, "(a ** (b *↓ c))");
    }

    #[test]
    fn test_pow_less_than_one_down() {
        let mut formula_config = FormulaConfig::new(
            "a ** (b * c)".to_string(),
            false,
            Some(vec!["a".to_string()]),
            None
        );
        let ast = analyze(&mut formula_config).unwrap().to_string();
        assert_eq!(ast, "(a ** (b *↑ c))");
    }

    #[test]
    fn test_negative() {
        let mut formula_config = FormulaConfig::new(
            "-(-(-a * b)) + c".to_string(),
            true,
            None,
            None
        );
        let ast = analyze(&mut formula_config).unwrap().to_string();
        assert_eq!(ast, "(c - (a *↓ b))");
    }

    #[test]
    fn test_double_negative() {
        let mut formula_config = FormulaConfig::new(
            "-(-(a * b)) + c".to_string(),
            true,
            None,
            None
        );
        let ast = analyze(&mut formula_config).unwrap().to_string();
        assert_eq!(ast, "((a *↑ b) + c)");
    }

    #[test]
    fn test_negative_exponent() {
        let mut formula_config = FormulaConfig::new(
            "a ** (-b * c)".to_string(),
            true,
            None,
            Some(vec!["a".to_string()])
        );
        let ast = analyze(&mut formula_config).unwrap().to_string();
        assert_eq!(ast, "(1 /↑ (a ** (b *↓ c)))");
    }
}
use std::process::Command;

use crate::ast::{Expr, Opcode, Rounding};
use crate::{config, constant};
use anyhow::{anyhow, Result};
use latex::{Document, DocumentClass, Section};
use std::fs::File;
use std::io::{self, Write};

/// This function takes an expression and returns a string representation of the expression in LaTeX format.
///
/// # Arguments
///
/// * `expr` - An `Expr` enum reference that represents the expression to be converted to LaTeX format.
///
/// # Returns
///
/// A string representation of the expression in LaTeX format.
fn visit(expr: &Expr) -> String {
    match expr {
        Expr::Number(n) => n.to_string(),
        Expr::Id(n) => n.to_string(),
        Expr::Op(left, op, right) => {
            let left_str = visit(left);
            let right_str = visit(right);

            let op_str = match op {
                Opcode::Div(op_rounding) => {
                    let direction: &str = match *op_rounding {
                        Rounding::Init => "",
                        Rounding::Up => r"\uparrow",
                        Rounding::Down => r"\downarrow",
                        Rounding::Unknown => "updownarrow",
                    };
                    return format!("(\\frac{{{left_str}}}{{{right_str}}}{{{direction}}})");
                }
                Opcode::Add => String::from("} + {"),
                Opcode::Sub => String::from("} - {"),
                Opcode::Mul(op_rounding) => {
                    let direction: &str = match *op_rounding {
                        Rounding::Init => "",
                        Rounding::Up => r"\uparrow",
                        Rounding::Down => r"\downarrow",
                        Rounding::Unknown => "updownarrow",
                    };
                    format!("}} *_{direction} {{")
                }
                Opcode::Pow => String::from("} ^ {"),
            };
            format!("({}{}{}{}{})", "{", left_str, op_str, right_str, "}")
        }
        Expr::Error => String::new(),
    }
}

/// Generates a LaTeX document containing the configuration and analysis of a given expression.
///
/// # Arguments
///
/// * `expr` - An `Expr` struct representing the expression to be analyzed.
/// * `config` - A `config::Config` struct containing the configuration for the analysis.
///
/// # Returns
///
/// A `Result` containing a `String` with the LaTeX document, or an `anyhow::Error` if an error occurred.
pub fn generate(expr: &Expr, config: &config::Config) -> Result<String> {
    let mut doc = Document::new(DocumentClass::Article);

    doc.preamble.use_package("hyperref");

    // Set some metadata for the document
    doc.preamble.title("Round me analysis");
    doc.preamble.author("roundme");

    let mut section_1 = Section::new("Config");
    let output = format!(
        "\\begin{{verbatim}} {} \\end{{verbatim}}",
        config::to_yaml_str(config)?
    );
    section_1.push(output.as_str());
    doc.push(section_1);

    let mut section_2 = Section::new("Rounding analysis");
    let output = format!("Expression: ${}$", visit(expr));
    section_2.push(output.as_str());
    doc.push(section_2);

    let mut section_3 = Section::new("roundme");
    let text = format!(
        "{} For more details, visit \\url{{https://github.com/crytic/roundme}}.",
        constant::DISCLAIMER
    );
    section_3.push(text.as_str());

    doc.push(section_3);

    latex::print(&doc).map_err(|e| anyhow!("Failed to print using latex::print: {}", e))
}

/// Writes the rendered string to a file, calls latexmk on it, and cleans up the intermediate files.
///
/// # Arguments
///
/// * `rendered` - A reference to a string containing the rendered LaTeX code.
///
/// # Errors
///
/// Returns an `io::Error` if the file cannot be created, if the latexmk command fails, or if the cleanup command fails.
///
/// # Examples
///
///
pub fn write(rendered: &String) -> io::Result<()> {
    // Open the file in write mode, which will create or truncate it
    let mut f = File::create("report.tex")?;
    // Write the rendered string to the file
    write!(f, "{rendered}")?;

    // Then call latexmk on it
    let exit_status = Command::new("latexmk")
        .arg("report.tex")
        .arg("-pdf")
        .status()?;

    // Check if latexmk command was successful
    if !exit_status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "latexmk command failed. Is it installed? (https://mg.readthedocs.io/latexmk.html)",
        )); // Return an error if not successful
    };

    let exit_status = Command::new("latexmk").arg("-c").status()?;

    if exit_status.success() {
        Ok(()) // Return Ok if successful
    } else {
        Err(io::Error::new(
            io::ErrorKind::Other,
            "latexmk command failed. Is it installed? (https://mg.readthedocs.io/latexmk.html)",
        )) // Return an error if not successful
    }
}

// TODO: add more tests
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_visit() {
        let expr = Expr::Op(
            Box::new(Expr::Id(String::from("a"))),
            Opcode::Add,
            Box::new(Expr::Number(3)),
        );
        let result = visit(&expr);
        assert_eq!(result, "({a} + {3})");
    }
}

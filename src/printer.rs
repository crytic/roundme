use clap::ValueEnum;

use crate::{analyzer::ast::Expr, FormulaConfig};

mod latex_generator;

/// DISCLAIMER message to be displayed to users.
pub const DISCLAIMER: &str = "round-me is a WIP, review manually all the results.";

pub struct Printer {
    output_format: OutputFormat,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Debug)]
pub enum OutputFormat {
    /// Prints the output on console
    Text,
    /// Creates a PDF file as output
    PDF,
}

impl Printer {
    pub fn new(output_format: OutputFormat) -> Printer {
        Printer { output_format }
    }
}

impl Printer {
    pub fn print(&self, ast: &Expr, formula_config: &FormulaConfig) -> anyhow::Result<()> {
        match self.output_format {
            OutputFormat::Text => Printer::print_text(ast)?,
            OutputFormat::PDF => Printer::print_pdf(ast, formula_config)?,
        }

        println!("{DISCLAIMER}");
        Ok(())
    }
}

impl Printer {
    /// Generates a PDF from the given expression AST using the provided configuration.
    /// Returns a `Result` indicating whether the operation was successful or not.
    fn print_pdf(ast: &Expr, formula_config: &FormulaConfig) -> anyhow::Result<()> {
        let latex_result = latex_generator::generate(ast, formula_config)?;
        latex_generator::write(&latex_result)?;
        Ok(())
    }

    fn print_text(ast: &Expr) -> anyhow::Result<()> {
        println!("");
        println!("Report:");
        println!("{ast}");
        Ok(())
    }
}

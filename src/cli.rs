use std::path::PathBuf;

use clap::{Parser, Subcommand, ValueHint};

use roundme::analyzer::analyze;
use roundme::parser::{InputFormat, Parser as FormulaParser};
use roundme::printer::{OutputFormat, Printer};

#[derive(Parser, Debug)]
pub struct CliArgs {
    /// Formula config file to analyze
    #[arg(value_hint = ValueHint::FilePath, default_value = "config.yaml")]
    formula_file: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// create a sample formula config file
    InitSample,

    /// create a formula config file from user provided input
    Init,

    /// analyze the specified formula config file
    Analyze {
        /// Output format, one of the [text, pdf]
        #[arg(short, long, value_enum, default_value = "text")]
        output_format: OutputFormat,
    },

    /// delete the specified formula config file
    Clean,
}

pub trait Cmd {
    fn run(&self) -> anyhow::Result<()>;
}

impl Cmd for CliArgs {
    fn run(&self) -> anyhow::Result<()> {
        // println!("{:?}", &self); // debug

        // Verify that the file_path is not empty
        self.formula_file.to_str().ok_or(anyhow::anyhow!(
            "Invalid value for the formula_file provided"
        ))?;

        let parser = FormulaParser::new(self.formula_file.clone(), InputFormat::YAML)?;

        // Create an instance of the parser
        match self.command {
            Commands::InitSample => {
                parser.init_sample()?;
            }

            Commands::Init => {
                parser.init()?;
            }

            Commands::Analyze { output_format } => {
                let mut formula_config = parser.parse()?;

                //println!("{:?}", &formula); // debug
                // analyze the formula
                let ast = analyze(&mut formula_config)?;

                // print the output
                let printer = Printer::new(output_format);
                printer.print(&ast, &formula_config)?
            }

            Commands::Clean => {
                parser.clean()?;
            }
        }

        anyhow::Ok(())
    }
}

#![warn(clippy::pedantic)]

use anyhow::Result;
use clap::{App, Arg, SubCommand};
use std::path::{Path, PathBuf};

use ast::Expr;

use lalrpop_util::lalrpop_mod;

mod analyze_rounding;
mod ast;
mod config;
mod constant;
mod expression_utils;
mod latex_generator;
mod utils;

lalrpop_mod!(pub arithmetic); // synthesized by LALRPOP

/// Initializes the configuration using the provided configuration file path.
///
/// # Arguments
///
/// * `config_path` - A `&Path` representing the path to the configuration file.
///
/// # Returns
///
/// A `Result` indicating whether the initialization was successful or not.
fn init(config_path: &Path) -> Result<()> {
    config::init(config_path.to_str().unwrap(), false)
}

/// Analyzes the configuration file and returns a tuple containing the parsed expression and the user configuration.
///
/// # Arguments
///
/// * `config_path` - A `PathBuf` representing the path to the configuration file.
///
/// # Returns
///
/// A `Result` containing a tuple of the parsed expression and the user configuration.
///
/// # Examples
///
/// ```
/// use std::path::PathBuf;
/// use round_me::{analyze, config};
///
/// let config_path = PathBuf::from("config.yaml");
/// let result = analyze(config_path);
///
/// assert!(result.is_ok());
/// ```
fn analyze(config_path: &Path) -> Result<(Box<Expr>, config::Config)> {
    // user_config is mutable, because the analyze might populate less_than_one / greater_than_one
    let mut user_config: config::Config =
        config::load_yaml(Option::unwrap_or(config_path.to_str(), "config.yaml"))?;
    let parse_expression = arithmetic::ExprParser::new();
    // TODO: remove unwrap and handle error
    let ast = parse_expression.parse(&user_config.formula).unwrap();
    analyze_rounding::analyze(&ast, user_config.round_up, &mut user_config)?;
    println!("{ast}");
    Ok((ast, user_config))
}

/// Generates a PDF from the given expression AST using the provided configuration.
/// Returns a `Result` indicating whether the operation was successful or not.
fn pdf(ast: &Expr, config: &config::Config) -> Result<()> {
    let latex_result = latex_generator::generate(ast, config)?;
    latex_generator::write(&latex_result)?;
    Ok(())
}

/// Removes the `config.yaml` file if it exists.
fn clean() -> Result<()> {
    let config_path = PathBuf::from("config.yaml");
    if config_path.exists() {
        std::fs::remove_file(config_path)?;
    }
    Ok(())
}

fn main() {
    let matches = App::new("roundme")
        .subcommand(
            SubCommand::with_name("init")
                .about("Initializes a default config.yaml")
                .arg(
                    Arg::with_name("config-path")
                        .long("config-path")
                        .value_name("FILE")
                        .help("Sets a custom config path")
                        .takes_value(true)
                        .default_value("config.yaml"), // Default value set here
                ),
        )
        .subcommand(
            SubCommand::with_name("analyze")
                .about("Analyze the formula")
                .arg(
                    Arg::with_name("config-path")
                        .long("config-path")
                        .value_name("FILE")
                        .help("Sets a custom config path")
                        .takes_value(true)
                        .default_value("config.yaml"), // Default value set here
                ),
        )
        .subcommand(
            SubCommand::with_name("pdf")
                .about("Analyze the formula and generate a PDF (require latexmk)")
                .arg(
                    Arg::with_name("config-path")
                        .long("config-path")
                        .value_name("FILE")
                        .help("Sets a custom config path")
                        .takes_value(true)
                        .default_value("config.yaml"), // Default value set here
                ),
        )
        .subcommand(
            SubCommand::with_name("config")
                .about("Generate a config file")
                .arg(
                    Arg::with_name("config-path")
                        .long("config-path")
                        .value_name("FILE")
                        .help("Sets a custom config path")
                        .takes_value(true)
                        .default_value("config.yaml"), // Default value set here
                ),
        )
        .subcommand(
            SubCommand::with_name("clean")
                .about("Clean the generated file (including the default config file)"),
        )
        .get_matches();

    let res = match matches.subcommand() {
        Some(("init", sub_m)) => {
            // Since we are now sure that "config-path" has a value, we can directly unwrap
            let config_path = PathBuf::from(sub_m.value_of("config-path").unwrap());
            init(&config_path)
        }
        Some(("analyze", sub_m)) => {
            // Since we are now sure that "config-path" has a value, we can directly unwrap
            let config_path = PathBuf::from(sub_m.value_of("config-path").unwrap());
            match analyze(&config_path) {
                Ok(_) => Ok(()),
                Err(err) => {
                    println!("{err:?}");
                    Ok(())
                }
            }
        }
        Some(("pdf", sub_m)) => {
            // Since we are now sure that "config-path" has a value, we can directly unwrap
            let config_path = PathBuf::from(sub_m.value_of("config-path").unwrap());
            match analyze(&config_path) {
                Ok((ast, config)) => pdf(&ast, &config),
                Err(err) => {
                    println!("{err:?}");
                    Ok(())
                }
            }
        }
        Some(("config", sub_m)) => {
            let config_path = PathBuf::from(sub_m.value_of("config-path").unwrap());
            match config::generate_config_user(&config_path) {
                Ok(()) => Ok(()),
                Err(err) => {
                    println!("{err:?}");
                    Ok(())
                }
            }
        }
        Some(("clean", _sub_m)) => clean(),
        _ => {
            eprintln!("Invalid subcommand. Please use either 'init', 'analyze', 'pdf' or 'clean'");
            std::process::exit(1);
        }
    };

    match res {
        Ok(()) => println!("{}", constant::DISCLAIMER),
        Err(err) => println!("{err:?}"),
    }
}

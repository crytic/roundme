use std::path::PathBuf;

mod formula_config;
mod input;
mod yaml;

use self::{
    input::ask_user_formula_config,
    yaml::{clean, from_yaml_file, to_yaml_file},
};
pub use formula_config::FormulaConfig;
pub use input::ask_yes_no;
pub use yaml::to_yaml_str;

use lalrpop_util::lalrpop_mod;
lalrpop_mod!(pub arithmetic, "/parser/arithmetic.rs"); // synthesized by LALRPOP

pub struct Parser {
    file_path: PathBuf,
    input_format: InputFormat,
}

pub enum InputFormat {
    YAML,
}

impl Parser {
    pub fn new(file_path: PathBuf, input_format: InputFormat) -> anyhow::Result<Parser> {
        let parser = Parser {
            file_path,
            input_format,
        };

        Ok(parser)
    }
}

impl Parser {
    pub fn init_sample(&self) -> anyhow::Result<()> {
        let formula = FormulaConfig::default();

        match self.input_format {
            InputFormat::YAML => to_yaml_file(self.file_path.as_path(), &formula),
        }
    }

    pub fn init(&self) -> anyhow::Result<()> {
        let formula = ask_user_formula_config()?;

        match self.input_format {
            InputFormat::YAML => to_yaml_file(self.file_path.as_path(), &formula),
        }
    }

    pub fn parse(&self) -> anyhow::Result<FormulaConfig> {
        match self.input_format {
            InputFormat::YAML => from_yaml_file(self.file_path.as_path()),
        }
    }

    pub fn clean(&self) -> anyhow::Result<()> {
        match self.input_format {
            InputFormat::YAML => clean(self.file_path.as_path()),
        }
    }
}

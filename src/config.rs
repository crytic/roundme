use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Write;
use std::path::Path;

use anyhow::{anyhow, Result};
use serde::Deserialize;
use serde::Serialize;

use crate::arithmetic;
use crate::ast::Expr;
use crate::expression_utils::find_less_greater_than_one;
use crate::utils;

/// Configuration struct for rounding numbers.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct Config {
    /// The formula used for rounding.
    pub formula: String,
    /// Whether to round up or down.
    pub round_up: bool,
    /// Optional list of values less than one to improve the rounding analysis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub less_than_one: Option<Vec<String>>,
    /// Optional list of values less than one to improve the rounding analysis.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub greater_than_one: Option<Vec<String>>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            formula: "((a * b)**(e/f)) / (c * d)".to_string(),
            round_up: true,
            less_than_one: None, // Default value is None, so it's optional and won't appear in the default YAML.
            greater_than_one: None, // Default value is None, so it's optional and won't appear in the default YAML.
        }
    }
}

impl Config {
    // Add a value to the `less_than_one` list
    pub fn add_less_than_one(&mut self, value: String) {
        match self.less_than_one {
            Some(ref mut values) => values.push(value),
            None => self.less_than_one = Some(vec![value]),
        }
    }

    // Add a value to the `greater_than_one` list
    pub fn add_greater_than_one(&mut self, value: String) {
        match self.greater_than_one {
            Some(ref mut values) => values.push(value),
            None => self.greater_than_one = Some(vec![value]),
        }
    }
}

/// Converts a `Config` struct to a YAML string.
///
/// # Arguments
///
/// * `config` - A reference to a `Config` struct.
///
/// # Returns
///
/// A `Result` containing the YAML string if successful, or an `anyhow::Error` if serialization fails.
pub fn to_yaml_str(config: &Config) -> Result<String> {
    serde_yaml::to_string(&config).map_err(|e| anyhow!("Failed to serialize default config: {}", e))
}

/// Initializes a new configuration file at the given file path with default values.
///
/// # Arguments
///
/// * `file_path` - A string slice that holds the path to the configuration file.
/// * `override_file` - Boolean, set to true if the config file can be overwritten
///
/// # Errors
///
/// This function will return an error if the configuration file already exists or if there is an issue creating or writing to the file.
///
/// # Example
///
/// ```
/// use round_me::config::init;
///
/// let file_path = "/path/to/config.yaml";
/// let result = init(file_path);
/// assert!(result.is_ok());
/// ```
pub fn init(file_path: &str, override_file: bool) -> Result<()> {
    if !override_file && Path::new(file_path).exists() {
        return Err(anyhow!(
            "Config file '{}' already exists. Consider using --config-path",
            file_path
        ));
    }

    let config = Config::default();

    let mut yaml = to_yaml_str(&config)?;

    // Manually append the comment for the less_than_one field
    yaml.push_str("# less_than_one: [\"a\", \"b\"] -- replace this if needed\n");

    let mut file = File::create(file_path)
        .map_err(|e| anyhow!("Failed to create file {}: {}", file_path, e))?;
    file.write_all(yaml.as_bytes())
        .map_err(|e| anyhow!("Failed to write to file {}: {}", file_path, e))?;

    println!("{file_path} generated.");

    Ok(())
}

fn ask_rounding() -> Result<bool, io::Error> {
    println!("Should the formula round up? Y/N (yes, no)");
    utils::ask_yes_no()
}

fn ask_user_config() -> Result<Config> {
    let (formula_user, expr) = ask_formula()?;

    let rounding = ask_rounding()?;

    let mut config = Config {
        formula: formula_user,
        round_up: rounding,
        less_than_one: None,
        greater_than_one: None,
    };

    find_less_greater_than_one(&expr, &mut config);

    Ok(config)
}

pub fn generate_config_user(config_path: &Path) -> Result<()> {
    let file_path = Option::unwrap_or(config_path.to_str(), "config.yaml");

    if Path::new(file_path).exists() {
        return Err(anyhow!(
            "Config file '{}' already exists. Consider using --config-path",
            file_path
        ));
    }
    let config = ask_user_config()?;

    let yaml = to_yaml_str(&config)?;

    let mut file = File::create(file_path)
        .map_err(|e| anyhow!("Failed to create file {}: {}", file_path, e))?;
    file.write_all(yaml.as_bytes())
        .map_err(|e| anyhow!("Failed to write to file {}: {}", file_path, e))?;

    println!("{file_path} generated.");

    Ok(())
}

pub fn load_yaml(file_path: &str) -> Result<Config> {
    let mut file =
        File::open(file_path).map_err(|e| anyhow!("Failed to open file {}: {}", file_path, e))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| anyhow!("Failed to read file {}: {}", file_path, e))?;

    let config: Config =
        serde_yaml::from_str(&contents).map_err(|e| anyhow!("Failed to parse YAML: {}", e))?;

    Ok(config)
}

pub fn ask_formula() -> Result<(String, Box<Expr>)> {
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

#[cfg(test)]
mod tests {
    use std::fs;

    use tempfile::NamedTempFile;

    use super::*;

    #[test]
    fn test_init() {
        // Create a new temporary file.
        let temp_file = NamedTempFile::new().unwrap();
        let file_path = temp_file.path();

        // Perform the initialization, which should write to the temp file.
        let result = init(file_path.to_str().unwrap(), true);
        assert!(result.is_ok());

        // Check that the file contains the expected YAML.
        let mut contents = String::new();
        temp_file.as_file().read_to_string(&mut contents).unwrap();
        let expected_yaml = "---\nformula: ((a * b)**(e/f)) / (c * d)\nround_up: true\n# less_than_one: [\"a\", \"b\"] -- replace this if needed\n";
        assert_eq!(contents, expected_yaml);
    }

    #[test]
    fn test_load_yaml() {
        // Create a temporary file
        let temp_file = NamedTempFile::new().unwrap();

        // Define the expected configuration
        let expected_config = Config {
            formula: "(a * b) / (c * d)".to_string(),
            round_up: true,
            less_than_one: None,
            greater_than_one: None,
        };

        // Serialize the expected configuration to YAML and write it to the temp file
        let yaml = serde_yaml::to_string(&expected_config).unwrap();
        fs::write(temp_file.path(), yaml).unwrap();

        // Load the YAML from the temp file
        let result = load_yaml(temp_file.path().to_str().unwrap());
        assert!(result.is_ok());
        let config = result.unwrap();

        // Compare the loaded configuration with the expected configuration
        assert_eq!(config, expected_config);
    }

    #[test]
    fn test_to_yaml_str() {
        let config = Config {
            formula: "(a * b) / (c * d)".to_string(),
            round_up: true,
            less_than_one: Some(vec!["a".to_string(), "b".to_string()]),
            greater_than_one: Some(vec!["c".to_string(), "d".to_string()]),
        };
        let expected_yaml = r#"---
formula: (a * b) / (c * d)
round_up: true
less_than_one:
  - a
  - b
greater_than_one:
  - c
  - d
"#;
        let yaml = to_yaml_str(&config).unwrap();
        assert_eq!(yaml, expected_yaml);
    }
}

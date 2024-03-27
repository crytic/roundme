use anyhow::{anyhow, Result};
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

use super::formula_config::FormulaConfig;

/// Converts a `FormulaConfig
///` struct to a YAML string.
///
/// # Arguments
///
/// * `formula_config` - A reference to a `FormulaConfig
///` struct.
///
/// # Returns
///
/// A `Result` containing the YAML string if successful, or an `anyhow::Error` if serialization fails.
pub fn to_yaml_str(formula_config: &FormulaConfig) -> Result<String> {
    serde_yaml::to_string(&formula_config)
        .map_err(|e| anyhow!("Failed to serialize default config: {}", e))
}

/// Parse a FormulaConfig object from the provided YAML file
pub fn from_yaml_file(file_path: &Path) -> Result<FormulaConfig> {
    let file_path_str = file_path.to_str().unwrap();

    let mut file = File::open(file_path)
        .map_err(|e| anyhow!("Failed to open file {}: {}", file_path_str, e))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .map_err(|e| anyhow!("Failed to read file {}: {}", file_path_str, e))?;

    let formula_config: FormulaConfig =
        serde_yaml::from_str(&contents).map_err(|e| anyhow!("Failed to parse YAML: {}", e))?;

    Ok(formula_config)
}

/// Creates a YAML file from provided FormulaConfig at provided file path
pub fn to_yaml_file(file_path: &Path, formula_config: &FormulaConfig) -> Result<()> {
    let file_path_str = file_path.to_str().unwrap();

    if file_path.exists() {
        return Err(anyhow!("Config file '{}' already exists.", file_path_str));
    }
    let yaml = to_yaml_str(&formula_config)?;

    let mut file = File::create(file_path)
        .map_err(|e| anyhow!("Failed to create file {}: {}", file_path_str, e))?;
    file.write_all(yaml.as_bytes())
        .map_err(|e| anyhow!("Failed to write to file {}: {}", file_path_str, e))?;

    println!("Generated the formula config file {}", file_path_str);

    Ok(())
}

/// Delete the YAML formula config file
pub fn clean(file_path: &Path) -> Result<()> {
    if file_path.exists() {
        std::fs::remove_file(file_path)?;
        println!(
            "Deleted the formula config file {}.",
            file_path.to_str().unwrap()
        );
    }
    Ok(())
}

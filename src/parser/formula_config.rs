use serde::Deserialize;
use serde::Serialize;

/// Configuration struct for rounding numbers.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct FormulaConfig {
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

impl Default for FormulaConfig {
    fn default() -> Self {
        Self {
            formula: "((a * b)**(e/f)) / (c * d)".to_string(),
            round_up: true,
            less_than_one: None, // Default value is None, so it's optional and won't appear in the default YAML.
            greater_than_one: None, // Default value is None, so it's optional and won't appear in the default YAML.
        }
    }
}

impl FormulaConfig {
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

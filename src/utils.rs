use std::io::{self, Write};

/// Asks the user to input a yes or no answer and returns true or false.
/// If the input is invalid, it recursively asks the user again until a valid input is received.
pub fn ask_yes_no() -> Result<bool, io::Error> {
    let mut input: String = String::new();

    // Flush to make sure the question is displayed before getting input
    io::stdout().flush()?;

    // Get user input
    io::stdin().read_line(&mut input)?;

    // Trim the input and convert it to lowercase
    let input = input.trim().to_lowercase();

    // Parse the input
    let parsed_input = match input.as_str() {
        "y" | "yes" => true,
        "n" | "no" => false,
        _ => {
            println!("Invalid input. Please enter Y, N.");
            return ask_yes_no(); // Recursively ask again if input is invalid
        }
    };

    Ok(parsed_input)
}

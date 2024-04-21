#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use math_lib;

/// Calculates a mathematical expression provided as a string.
///
/// This function serves as a Tauri command that allows the evaluation of a mathematical expression
/// provided as a string. The expression may include numbers, operators, and constants like π (pi).
/// It first replaces the π character with its numerical value before parsing and evaluating
/// the expression using the `math_lib::parse` function.
///
/// # Arguments
/// * `equation` - A string slice that holds the mathematical expression to be evaluated.
///
/// # Returns
/// A `Result` which is either:
/// - `Ok(String)` containing the result of the evaluated expression as a string.
/// - `Err(String)` containing an error message if the expression is invalid or cannot be evaluated.
///
/// # Example
/// ```
/// let result = calculate("3.14159 * 2".to_string());
/// assert_eq!(result.unwrap(), "6.28318");
/// ```
#[tauri::command]
fn calculate(equation: String) -> Result<String, String> {
    let equation = equation
        .replace("π", "3.14159265358979323846264338327950288");
    Ok(math_lib::parse(equation.as_str())?.to_string())
}

/// The main entry point for the Tauri application.
///
/// This function sets up the Tauri application with necessary configurations and command handlers.
/// It defines the command handler for the `calculate` function, enabling it to be called from the frontend.
/// The application is configured and run within this function, and an error is thrown if the application
/// fails to run correctly.
///
/// # Errors
/// If the Tauri application fails to initialize or run, it will return an error message and terminate.
///
/// # Example
/// Run this function to start the Tauri application.
/// ```no_run
/// fn main() {
///     tauri::Builder::default()
///         .invoke_handler(tauri::generate_handler![calculate])
///         .run(tauri::generate_context!())
///         .expect("error while running tauri application");
/// }
/// ```
fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![calculate])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

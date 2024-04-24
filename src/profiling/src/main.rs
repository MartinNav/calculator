use std::io::{Read, stdin};
use math_lib::parse;

/// Prepares a string to be used by the parser to calculate the standard deviation.
///
/// # Arguments
///
/// * `input` - A string of numbers separated by whitespace.
///
/// # Returns
///
/// * A string that can be used by the parser.
///
/// # Examples
/// ```
/// let input = "1 2 3 4 5".to_string();
/// let result = prepare_to_parse(input);
/// assert_eq!(result, "2√(1/4*(1^2+2^2+3^2+4^2+5^2)-1/5*(1+2+3+4+5)^2))");
/// ```
pub fn prepare_to_parse(input: String) -> String {
    let numbers = input
        .split_whitespace()
        .collect::<Vec<_>>();

    let numbers_total_char_length: usize = numbers.iter().map(|s| s.len()).sum();
    let number_count = numbers.len().to_string();
    let number_count_minus_one = (numbers.len() - 1).to_string();

    let total_length = 17 + 4 * numbers.len() + 2 * numbers_total_char_length + number_count.len() + number_count_minus_one.len();
    let mut expression = String::with_capacity(total_length);

    // 2√(
    expression.push_str("2√(");
    // 2√(1/(N-1)
    expression.push_str("1/");
    expression.push_str(number_count_minus_one.as_str());
    // 2√(1/(N-1)*(
    expression.push_str("*(");
    // 2√(1/(N-1)*(Σxi^2)
    for number in &numbers {
        expression.push_str(number);
        expression.push_str("^2+");
    }
    expression.pop();
    // 2√(1/(N-1)*(Σxi^2)-1/N
    expression.push_str("-1/");
    expression.push_str(number_count.as_str());
    // 2√(1/(N-1)*(Σxi^2)-1/N*(Σxi)^2
    expression.push_str("*(");
    for number in &numbers {
        expression.push_str(number);
        expression.push('+');
    }
    expression.pop();
    // 2√(1/(N-1)*(Σxi^2)-1/N*((Σxi)^2)))
    expression.push_str(")^2))");

    assert_eq!(expression.len(), total_length);

    expression
}

/// Reads the input from the standard input, prepares it to be used by the parser, and calculates the standard deviation.
fn main() -> Result<(), String> {
    let mut input = String::new();
    let read = stdin().read_to_string(&mut input).unwrap_or(0);
    if read == 0 {
        return Err("No input provided".to_string());
    }
    let result = parse(prepare_to_parse(input).as_str());
    if result.is_err() {
        return Err(result.err().unwrap().to_string());
    }
    Ok(println!("{}", result.unwrap()))
}

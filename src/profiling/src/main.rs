use std::io::{Read, stdin};
use math_lib::evaluate_parse_tree;

/// Takes values for a standard deviation calculation and processes it to a string that can used by the parser.
pub fn prepare_to_parse(input: String) -> String {
    let numbers = input
        .split_whitespace()
        .collect::<Vec<_>>();

    let numbers_total_char_length: usize = numbers.iter().map(|s| s.len()).sum();
    let number_count = numbers.len().to_string();
    let number_count_minus_one = (numbers.len() - 1).to_string();

    let total_length = 16 + 4 * numbers.len() + 2 * numbers_total_char_length + number_count.len() + number_count_minus_one.len();
    let mut expression = String::with_capacity(total_length);

    // s
    expression.push_str("R(");
    // 1/(N-1)
    expression.push_str("1/");
    expression.push_str(number_count_minus_one.as_str());
    // (Σxi^2 - 1/N * (xt^2))
    expression.push_str("*(");
    // Σxi^2
    for number in &numbers {
        expression.push_str(number);
        expression.push_str("^2+");
    }
    expression.pop();
    // -1/N * (xt^2)
    expression.push_str("-1/");
    expression.push_str(number_count.as_str());
    // xt = Σxi
    expression.push_str("*(");
    for number in &numbers {
        expression.push_str(number);
        expression.push('+');
    }
    expression.pop();
    expression.push_str(")^2),2)");

    assert_eq!(expression.len(), total_length);

    expression
}

/// Takes values for a standard deviation calculation and processes it to a parse tree that can be used in the evaluator.
/// Returns a [String] in the same format as the parser would output.
pub fn prepare_to_evaluate(input: String) -> String {
    let numbers = input
        .split_whitespace()
        .collect::<Vec<_>>();

    let numbers_total_char_length: usize = numbers.iter().map(|s| s.len()).sum();
    let number_count = numbers.len().to_string();
    let number_count_minus_one = (numbers.len() - 1).to_string();

    let total_length = 19 + 10 * (numbers.len()) + 2 * numbers_total_char_length + number_count.len() + number_count_minus_one.len();
    let mut expression = String::with_capacity(total_length);

    expression.push_str("1 ");
    expression.push_str(number_count_minus_one.as_str());
    expression.push_str(" / ");
    for number in &numbers {
        expression.push_str(number);
        expression.push_str(" 2 ^ ");
    }
    for _ in 1..numbers.len() {
        expression.push_str("+ ");
    }
    expression.push_str("1 ");
    expression.push_str(number_count.as_str());
    expression.push_str(" / ");
    for number in &numbers {
        expression.push_str(number);
        expression.push(' ');
    }
    for _ in 1..numbers.len() {
        expression.push_str("+ ");
    }
    expression.push_str("2 ^ * - * 2 R");

    assert_eq!(expression.len(), total_length);

    expression
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();
    println!("{}", evaluate_parse_tree(prepare_to_evaluate(input)).unwrap());
    todo!("Use prepare_to_parse once parser is implemented");
}
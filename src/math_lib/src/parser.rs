// Author: Richard Gajdosik <gajdo33@vutbr.cz> 2024 VUT FIT
// Sources: https://www.fit.vutbr.cz/study/courses/IFJ/private/prednesy/Ifj08-en.pdf
//          https://github.com/RichardGajdosik/VUTFIT_IFJ_2021_Projekt/blob/master/src/expressions.c#L182

// Operators are represented as single characters
const CHARS: [char; 8] = ['*', '/', '+', '-', '(', ')', 'i', '$'];

// Rules are represented as strings
const RULES: [&str; 6] = ["i", ")E(", "E+E", "E-E", "E*E", "E/E"];


// Operator enum representing possible operators in the expressions
#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Plus,       // Represents '+'
    Minus,      // Represents '-'
    Multiply,   // Represents '*'
    Divide,     // Represents '/'
    Power,      // Represents '^'
    Root,       // Represents '√'
    Factorial,  // Represents '!'
    OpenParen,  // Represents '('
    CloseParen, // Represents ')'
    EndOfInput, // Represents '$'
    Identifier, // Represents 'i'
    PrecedenceEnd, // Represents '>'
    PrecedenceStart, // Represents '<'
}

// Token enum representing either a value or an operator
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Operand(f64),   // For numerical values
    Operator(Operator), // For operators including parentheses and end of input
}

fn process_current_number(current_number: &mut String, output_queue: &mut Vec<Token>) -> Result<(), String> {
    if !current_number.is_empty() {
        match current_number.parse::<f64>() {
            Ok(num) => {
                output_queue.push(Token::Operand(num));
                current_number.clear();
            },
            Err(_) => return Err("Failed to parse number".to_string()),
        }
    }
    Ok(())
}


// Function to convert an infix expression to postfix notation
fn to_postfix(input: &str) -> Result<Vec<Token>, String> {
    // We define the precedence table as a 2D array
    //let precedence_table: Vec<Vec<char>> = vec![
    //       // *    /    +    -    (    )    i    $
    //    vec!['>', '>', '>', '>', '<', '>', '<', '>'], // *
    //    vec!['>', '>', '>', '>', '<', '>', '<', '>'], // /
    //    vec!['<', '<', '>', '>', '<', '>', '<', '>'], // +
    //    vec!['<', '<', '>', '>', '<', '>', '<', '>'], // -
    //    vec!['<', '<', '<', '<', '<', '=', '<', 'c'], // (
    //    vec!['>', '>', '>', '>', 'c', '>', 's', '>'], // )
    //    vec!['>', '>', '>', '>', 'c', '>', 's', '>'], // i
    //    vec!['<', '<', '<', '<', '<', 'c', '<', 'c'], // $
    //];

    let mut output_queue: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = vec![Token::Operator(Operator::EndOfInput)];
    let mut current_number = String::new();

    let mut input_chars = input.chars().peekable();

    while let Some(c) = input_chars.next() {
        match c {
            '+' => {
                output_queue.push(Token::Operator(Operator::Plus));
            }
            '-' => {
                output_queue.push(Token::Operator(Operator::Minus));
            }
            '*' => {
                output_queue.push(Token::Operator(Operator::Multiply));
            }
            '/' => {
                output_queue.push(Token::Operator(Operator::Divide));
            }
            '(' => {
                output_queue.push(Token::Operator(Operator::OpenParen));
            }
            ')' => {
                output_queue.push(Token::Operator(Operator::CloseParen));
            }
            '0'..='9' | '.' => {
                // Accumulate digit and decimal point characters into current_number
                current_number.push(c);
                // Check next character to decide if we should continue accumulating or process the number
                if let Some(&next_char) = input_chars.peek() {
                    if !next_char.is_digit(10) && next_char != '.' {
                        process_current_number(&mut current_number, &mut output_queue)?;
                    }
                } else {
                    // If this is the last character, ensure the number is processed
                    process_current_number(&mut current_number, &mut output_queue)?;
                }
            }
            _ => return Err(format!("Invalid character in input: {}", c)),
        }
    }

    output_queue.push(Token::Operator(Operator::EndOfInput));

    //for c in input.chars() {
    //    
    //    // Print the stack for each iteration
    //    println!("Current stack: {:?}", stack);
//
    //    if let Token::Operator(op) = token {
    //        match get_precedence(&stack.last().unwrap(), &token) {
    //            '<' => {
    //                // If input operator has lower precedence, push it onto the stack
    //                stack.push(token);
    //                stack.push(Operator::PrecedenceStart);
    //            },
    //            '>' => {
    //                stack.push(Operator::PrecedenceEnd);
    //            },
    //            _ => return Err("Invalid precedence".to_string()),
    //        }
    //    }
    //}

    Ok(output_queue)
}


//pub fn parse(input: &str) -> String {
//    let postfix = to_postfix(input);
//    println!("{:?}", postfix)
//    //format!("{}", input)
//}

pub fn parse(input: &str) -> String {
    match to_postfix(input) {
        Ok(postfix) => {
            let postfix_str = postfix.iter().map(|token| match token {
                Token::Operand(num) => num.to_string(),
                // Convert operators to their symbols or identifiers
                Token::Operator(op) => match op { 
                    Operator::Plus => "+".to_string(),
                    Operator::Minus => "-".to_string(),
                    Operator::Multiply => "*".to_string(),
                    Operator::Divide => "/".to_string(),
                    Operator::Power => "^".to_string(),
                    Operator::Root => "√".to_string(),
                    Operator::Factorial => "!".to_string(),
                    Operator::OpenParen => "(".to_string(),
                    Operator::CloseParen => ")".to_string(),
                    Operator::EndOfInput => "$".to_string(), 
                    Operator::Identifier => "i".to_string(), 
                    _ => " ".to_string(),
                },
            }).collect::<Vec<_>>().join(" ");
            postfix_str
        },
        Err(e) => e,
    }
}



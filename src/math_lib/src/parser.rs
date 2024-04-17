// Author: Richard Gajdosik <gajdo33@vutbr.cz> 2024 VUT FIT
// Sources: https://www.fit.vutbr.cz/study/courses/IFJ/private/prednesy/Ifj08-en.pdf
//          https://github.com/RichardGajdosik/VUTFIT_IFJ_2021_Projekt/blob/master/src/expressions.c

//TODO: Better comments
//TODO: Connect to FE and BE
//TODO: Add support for negative numbers
//TODO: Bug, "(1+2" sends it to oblivion, todo: exit gracefully instead of exiting whole program

// Operator enum representing possible operators in the expressions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
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
}

impl Operator {
    fn precedence_index(&self) -> usize {
        match self {
            Operator::Multiply => 0,
            Operator::Divide => 1,
            Operator::Plus => 2,
            Operator::Minus => 3,
            Operator::OpenParen => 4,
            Operator::CloseParen => 5,
            Operator::EndOfInput => 6,
            Operator::Power => 8,
            Operator::Root => 9,
            Operator::Factorial => 10,
        }
    }
}
// Token enum representing either a value or an operator
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Operand(f64),              // For numerical values
    Operator(Operator, usize), // For operators including parentheses and end of input
}

fn evaluate_expression(tokens: Vec<Token>) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token {
            Token::Operand(num) => {
                stack.push(num);
            }
            Token::Operator(op, _) => {
                if stack.len() < 2 {
                    return Err("Invalid expression".to_string());
                }
                let right = stack.pop().unwrap();
                let left = stack.pop().unwrap();

                let answer = match op {
                    Operator::Plus => {
                        left + right
                    }
                    Operator::Minus => {
                        left - right
                    }
                    Operator::Multiply => {
                        left * right
                    }
                    Operator::Divide => {
                        if right.abs() < f64::EPSILON {
                            return Err("Cannot divide by zero".to_string());
                        }
                        left / right
                    }
                    Operator::Power => {
                        left.powf(right)
                    }
                    Operator::Root => {
                        if right.abs() < f64::EPSILON {
                            return Err("Cannot take the 0th root".to_string());
                        }
                        if left < 0. {
                            return Err("Cannot take the root of a negative number".to_string());
                        }
                        left.powf(1. / right)
                    }
                    Operator::Factorial => {
                        if left < 0. {
                            return Err("Cannot take factorial of a negative number".to_string());
                        }
                        let mut res = 1.0;
                        (2..=(left as i64)).for_each(|i| res *= i as f64);
                        res
                    }
                    _ => {
                        return Err(format!("{op:?} is an invalid operator"));
                    }
                };

                stack.push(answer);
            }
        }
    }

    if stack.len() != 1 {
        return Err("Invalid expression".to_string());
    }

    Ok(stack.pop().unwrap())
}

fn process_current_number(
    current_number: &mut String,
    output_queue: &mut Vec<Token>,
) -> Result<(), String> {
    if !current_number.is_empty() {
        match current_number.parse::<f64>() {
            Ok(num) => {
                output_queue.push(Token::Operand(num));
                current_number.clear();
            }
            Err(_) => return Err("Failed to parse number".to_string()),
        }
    }
    Ok(())
}

// Function to convert an infix expression to postfix notation
fn to_postfix(input: &str) -> Result<Vec<Token>, String> {
    // We define the precedence table as a 2D array
    let precedence_table: Vec<Vec<char>> = vec![
        // *    /    +    -    (    )    $    i    ^    √    !
        vec!['>', '>', '>', '>', '<', '>', '>', '<', '<', '<', '<'], // *
        vec!['>', '>', '>', '>', '<', '>', '>', '<', '<', '<', '<'], // /
        vec!['<', '<', '>', '>', '<', '>', '>', '<', '<', '<', '<'], // +
        vec!['<', '<', '>', '>', '<', '>', '>', '<', '<', '<', '<'], // -
        vec!['<', '<', '<', '<', '<', '=', 'c', '<', '<', '<', '<'], // (
        vec!['>', '>', '>', '>', 'c', '>', '>', 'c', '>', '>', '>'], // )
        vec!['<', '<', '<', '<', '<', 'c', 's', '<', '<', '<', '<'], // $
        vec!['>', '>', '>', '>', 'c', '>', '>', 'c', '>', '>', '>'], // i
        vec!['>', '>', '>', '>', '<', '>', '>', '<', '>', '>', '<'], // ^
        vec!['>', '>', '>', '>', '<', '>', '>', '<', '>', '>', '<'], // √
        vec!['>', '>', '>', '>', 'c', 'c', '>', '<', '>', '>', '>'], // !
    ];

    let mut Input_queue: Vec<Token> = Vec::new();
    let mut Output_queue: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = vec![Token::Operator(
        Operator::EndOfInput,
        Operator::EndOfInput.precedence_index(),
    )];
    let mut current_number = String::new();

    let mut input_chars = input.chars().peekable();

    while let Some(c) = input_chars.next() {
        match c {
            '+' => {
                let op = Operator::Plus;
                Input_queue.push(Token::Operator(op, op.precedence_index()));
            }
            '-' => {
                let op = Operator::Minus;
                Input_queue.push(Token::Operator(op, op.precedence_index()));
            }
            '*' => {
                let op = Operator::Multiply;
                Input_queue.push(Token::Operator(op, op.precedence_index()));
            }
            '/' => {
                let op = Operator::Divide;
                Input_queue.push(Token::Operator(op, op.precedence_index()));
            }
            '(' => {
                let op = Operator::OpenParen;
                Input_queue.push(Token::Operator(op, op.precedence_index()));
            }
            ')' => {
                let op = Operator::CloseParen;
                Input_queue.push(Token::Operator(op, op.precedence_index()));
            }
            '^' => {
                let op = Operator::Power;
                Input_queue.push(Token::Operator(op, op.precedence_index()));
            }
            '√' => {
                let op = Operator::Root;
                Input_queue.push(Token::Operator(op, op.precedence_index()));
            }
            '!' => {
                let op = Operator::Factorial;
                Input_queue.push(Token::Operator(op, op.precedence_index()));
            }
            '0'..='9' | ',' | '.' => {
                // If the character is a comma, replace it with a decimal point
                let character = if c == ',' { '.' } else { c };
                // Accumulate digit and decimal point characters into current_number
                current_number.push(character);
                // Check next character to decide if we should continue accumulating or process the number
                if let Some(&next_char) = input_chars.peek() {
                    if !next_char.is_digit(10) && next_char != ',' && next_char != '.' {
                        process_current_number(&mut current_number, &mut Input_queue)?;
                    }
                } else {
                    // If this is the last character, ensure the number is processed
                    process_current_number(&mut current_number, &mut Input_queue)?;
                }
            }
            _ => return Err(format!("Invalid character in input: {}", c)),
        }
    }

    Input_queue.push(Token::Operator(
        Operator::EndOfInput,
        Operator::EndOfInput.precedence_index(),
    ));
    Input_queue.reverse();

    let mut index_first = 0;
    let mut index_second = 0;
    while let Some(token) = Input_queue.last().cloned() {
        println!("Current stack: {:?}", stack);

        // Pop top of vector stack
        let top = stack.last().cloned();
        match top {
            // If the top of the stack is an operand, set index to 7
            Some(Token::Operand(_)) => {
                index_first = 7;
            }
            // Find the index of the top of the stack in the precedence table
            Some(Token::Operator(op, _)) => {
                index_first = op.precedence_index();
            }
            // None
            _ => {
                println!("Unexpected token or empty stack");
            }
        }

        match token {
            // If first Input_queue member is an operand, set index to 7
            Token::Operand(_) => {
                index_second = 7;
            }
            // Find index of first Input_queue member in precedence table
            Token::Operator(op, _) => {
                index_second = op.precedence_index();
            }
        }

        let precedence = precedence_table[index_first][index_second];

        match precedence {
            '<' => {
                // If input operator has lower precedence, push it onto the stack
                println!("Pushing top of Input_queue to stack");
                let token_to_push_to_stack = Input_queue.pop().unwrap();
                stack.push(token_to_push_to_stack);
            }
            '>' => {
                // If input operator has higher precedence, pop operator from stack onto the Output_queue
                println!("Popping token from stack, pushing it to the output queue.");
                let token_to_push_to_queue = stack.pop().expect("Expected token on the stack");
                Output_queue.push(token_to_push_to_queue);
            }
            '=' => {
                // We pop from stack and Input_queue
                stack.pop().expect("Expected token '(' on the stack");
                Input_queue.pop().expect("Expected ')' on the stack");
            }
            's' => {
                // Special case where we matched $ with $ we end here!
                println!("End of input and end of precedence stack");
                return Ok(Output_queue);
            }
            'c' => {
                // Error
                println!("Conflict in precedence");
                std::process::exit(1);
            }
            _ => {
                println!("Unexpected precedence value");
                std::process::exit(1);
            }
        }
    }

    Ok(Output_queue)
}

pub fn parse(input: &str) -> Result<String, String> {
    let postfix_result = to_postfix(input)?;
    evaluate_expression(postfix_result)?.tostring()
}

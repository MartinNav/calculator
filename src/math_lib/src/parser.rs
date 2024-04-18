// Author: Richard Gajdosik <gajdo33@vutbr.cz> 2024 VUT FIT
// Sources: https://www.fit.vutbr.cz/study/courses/IFJ/private/prednesy/Ifj08-en.pdf
//          https://github.com/RichardGajdosik/VUTFIT_IFJ_2021_Projekt/blob/master/src/expressions.c

//TODO: Better comments
//TODO: Add support for negative numbers
//TODO: Bug, "(1+2" sends it to oblivion, todo: exit gracefully instead of exiting whole program

// Operator enum representing possible operators in the expressions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Operator {
    Plus,       // Represents '+'
    Minus,      // Represents '-'
    Multiply,   // Represents '*'
    Divide,     // Represents '/'
    Percent,    // Represents '%'
    Power,      // Represents '^'
    Root,       // Represents '√'
    Factorial,  // Represents '!'
    OpenParen,  // Represents '('
    CloseParen, // Represents ')'
    EndOfInput, // Represents '$'
}

// Token enum representing either a value or an operator
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Operand(f64),       // For numerical values
    Operator(Operator), // For operators including parentheses and end of input
}

impl Token {
    fn precedence_index(&self) -> usize {
        match self {
            Token::Operator(Operator::Multiply) => 0,
            Token::Operator(Operator::Divide) => 1,
            Token::Operator(Operator::Percent) => 2,
            Token::Operator(Operator::Plus) => 3,
            Token::Operator(Operator::Minus) => 4,
            Token::Operator(Operator::OpenParen) => 5,
            Token::Operator(Operator::CloseParen) => 6,
            Token::Operator(Operator::EndOfInput) => 7,
            Token::Operand(_) => 8,
            Token::Operator(Operator::Power) => 9,
            Token::Operator(Operator::Root) => 10,
            Token::Operator(Operator::Factorial) => 11,
        }
    }
}
fn evaluate_expression(tokens: Vec<Token>) -> Result<f64, String> {
    let mut stack: Vec<f64> = Vec::new();

    for token in tokens {
        match token {
            Token::Operand(num) => {
                stack.push(num);
            }
            Token::Operator(op) => {
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
                    Operator::Percent => {
                        if right.abs() < f64::EPSILON {
                            return Err("Cannot take the percentage of zero".to_string());
                        }
                        left / right * 100.0
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

fn tokenize(input: &str) -> Result<Vec<Token>, String> {
    let mut input_queue: Vec<Token> = Vec::new();
    let mut current_number = String::new();

    let mut input_chars = input.chars().peekable();
    while let Some(c) = input_chars.next() {
        match c {
            '+' => {
                let op = Operator::Plus;
                input_queue.push(Token::Operator(op));
            }
            '-' => {
                let op = Operator::Minus;
                input_queue.push(Token::Operator(op));
            }
            '*' => {
                let op = Operator::Multiply;
                input_queue.push(Token::Operator(op));
            }
            '/' => {
                let op = Operator::Divide;
                input_queue.push(Token::Operator(op));
            }
            '%' => {
                let op = Operator::Percent;
                input_queue.push(Token::Operator(op));
            }
            '(' => {
                let op = Operator::OpenParen;
                input_queue.push(Token::Operator(op));
            }
            ')' => {
                let op = Operator::CloseParen;
                input_queue.push(Token::Operator(op));
            }
            '^' => {
                let op = Operator::Power;
                input_queue.push(Token::Operator(op));
            }
            '√' => {
                let op = Operator::Root;
                input_queue.push(Token::Operator(op));
            }
            '!' => {
                let op = Operator::Factorial;
                input_queue.push(Token::Operator(op));
                // We implicitly add a second operand so that the factorial can be evaluated
                input_queue.push(Token::Operand(1.));
            }
            '0'..='9' | ',' | '.' => {
                // If the character is a comma, replace it with a decimal point
                let character = if c == ',' { '.' } else { c };
                // Accumulate digit and decimal point characters into current_number
                current_number.push(character);
                // Check next character to decide if we should continue accumulating or process the number
                if let Some(&next_char) = input_chars.peek() {
                    if !next_char.is_digit(10) && next_char != ',' && next_char != '.' {
                        process_current_number(&mut current_number, &mut input_queue)?;
                    }
                } else {
                    // If this is the last character, ensure the number is processed
                    process_current_number(&mut current_number, &mut input_queue)?;
                }
            }
            _ => return Err(format!("Invalid character in input: {}", c)),
        }
    }

    input_queue.push(Token::Operator(Operator::EndOfInput));
    Ok(input_queue)
}

// Function to convert an infix expression to postfix notation
fn to_postfix(input_queue: Vec<Token>) -> Result<Vec<Token>, String> {
    // We define the precedence table as a 2D array
    let precedence_table: Vec<Vec<char>> = vec![
        //    *    /    %    +    -    (    )    $    i    ^    √    !
        vec!['>', '>', '>', '>', '>', '<', '>', '>', '<', '<', '<', '<'], // *
        vec!['>', '>', '>', '>', '>', '<', '>', '>', '<', '<', '<', '<'], // /
        vec!['>', '>', '>', '>', '>', '<', '>', '>', '<', '<', '<', '<'], // %
        vec!['<', '<', '<', '>', '>', '<', '>', '>', '<', '<', '<', '<'], // +
        vec!['<', '<', '<', '>', '>', '<', '>', '>', '<', '<', '<', '<'], // -
        vec!['<', '<', '<', '<', '<', '<', '=', 'c', '<', '<', '<', '<'], // (
        vec!['>', '>', '>', '>', '>', 'c', '>', '>', 'c', '>', '>', '>'], // )
        vec!['<', '<', '<', '<', '<', '<', 'c', 's', '<', '<', '<', '<'], // $
        vec!['>', '>', '>', '>', '>', 'c', '>', '>', 'c', '>', '>', '>'], // i
        vec!['>', '>', '>', '>', '>', '<', '>', '>', '<', '>', '>', '<'], // ^
        vec!['>', '>', '>', '>', '>', '<', '>', '>', '<', '>', '>', '<'], // √
        vec!['>', '>', '>', '>', '>', 'c', 'c', '>', '<', '>', '>', '>'], // !
    ];

    let mut input_queue = input_queue;
    input_queue.reverse();
    let mut output_queue: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = vec![Token::Operator(Operator::EndOfInput)];

    while let Some(token) = input_queue.last().cloned() {
        println!("Current stack: {:?}", stack);

        // Pop top of vector stack
        let top = stack.last().cloned();

        let index_first= match top {
            Some(token) => token.precedence_index(),
            None => {
                println!("Empty stack");
                0
            }
        };
        let index_second = token.precedence_index();

        let precedence = precedence_table[index_first][index_second];

        match precedence {
            '<' => {
                // If input operator has lower precedence, push it onto the stack
                println!("Pushing top of Input_queue to stack");
                let token_to_push_to_stack = input_queue.pop().unwrap();
                stack.push(token_to_push_to_stack);
            }
            '>' => {
                // If input operator has higher precedence, pop operator from stack onto the Output_queue
                println!("Popping token from stack, pushing it to the output queue.");
                let token_to_push_to_queue = stack.pop().expect("Expected token on the stack");
                output_queue.push(token_to_push_to_queue);
            }
            '=' => {
                // We pop from stack and Input_queue
                stack.pop().expect("Expected token '(' on the stack");
                input_queue.pop().expect("Expected ')' on the stack");
            }
            's' => {
                // Special case where we matched $ with $ we end here!
                println!("End of input and end of precedence stack");
                return Ok(output_queue);
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

    Ok(output_queue)
}

pub fn parse(input: &str) -> Result<String, String> {
    let tokens = tokenize(input)?;
    let postfix_result = to_postfix(tokens)?;
    let result = evaluate_expression(postfix_result)?;
    Ok(result.to_string())
}

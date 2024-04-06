// Author: Richard Gajdosik <gajdo33@vutbr.cz> 2024 VUT FIT
// Sources: https://www.fit.vutbr.cz/study/courses/IFJ/private/prednesy/Ifj08-en.pdf
//          https://github.com/RichardGajdosik/VUTFIT_IFJ_2021_Projekt/blob/master/src/expressions.c

//TODO: Better comments
//TODO: Add support for '^' '√' '!'
//TODO: Go from english notation to czech notation in decimal numbers
//TODO: Connect to FE and BE
//TODO: Go from postfix to expression tree

// Operator enum representing possible operators in the expressions
#[derive(Debug, Clone, Copy, PartialEq)]
enum Operator {
    Plus,       // Represents '+'
    Minus,      // Represents '-'
    Multiply,   // Represents '*'
    Divide,     // Represents '/'
    //Power,      // Represents '^'
    //Root,       // Represents '√'
    //Factorial,  // Represents '!'
    OpenParen,  // Represents '('
    CloseParen, // Represents ')'
    EndOfInput, // Represents '$'
    //Identifier, // Represents 'i'
    PrecedenceEnd, // Represents '>'
    PrecedenceStart, // Represents '<'
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
            //Operator::Identifier => 7,
            Operator::PrecedenceStart => 41,
            Operator::PrecedenceEnd => 42,
        }
    }
}

// Token enum representing either a value or an operator
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Operand(f64),               // For numerical values
    Operator(Operator, usize),  // For operators including parentheses and end of input
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
    let precedence_table: Vec<Vec<char>> = vec![
           // *    /    +    -    (    )    $    i   
        vec!['>', '>', '>', '>', '<', '>', '>', '<'], // *
        vec!['>', '>', '>', '>', '<', '>', '>', '<'], // /
        vec!['<', '<', '>', '>', '<', '>', '>', '<'], // +
        vec!['<', '<', '>', '>', '<', '>', '>', '<'], // -
        vec!['<', '<', '<', '<', '<', '=', 'c', '<'], // (
        vec!['>', '>', '>', '>', 'c', '>', '>', 'c'], // )
        vec!['<', '<', '<', '<', '<', 'c', 's', '<'], // $
        vec!['>', '>', '>', '>', 'c', '>', '>', 'c'], // i
    ];

    let mut Input_queue: Vec<Token> = Vec::new();
    let mut Output_queue: Vec<Token> = Vec::new();
    let mut stack: Vec<Token> = vec![Token::Operator(Operator::EndOfInput, Operator::EndOfInput.precedence_index())];
    let mut current_number = String::new();

    let mut input_chars = input.chars().peekable();

    while let Some(c) = input_chars.next() {
        match c {
            '+' => {
                let op = Operator::Plus;
                Input_queue.push(Token::Operator(op, op.precedence_index()));
            },
            '-' => {
                let op = Operator::Minus;
                Input_queue.push(Token::Operator(op, op.precedence_index()));
            },
            '*' => {
                let op = Operator::Multiply;
                Input_queue.push(Token::Operator(op, op.precedence_index()));
            },
            '/' => {
                let op = Operator::Divide;
                Input_queue.push(Token::Operator(op, op.precedence_index()));
            },
            '(' => {
                let op = Operator::OpenParen;
                Input_queue.push(Token::Operator(op, op.precedence_index()));
            },
            ')' => {
                let op = Operator::CloseParen;
                Input_queue.push(Token::Operator(op, op.precedence_index()));
            },
            '0'..='9' | '.' => {
                // Accumulate digit and decimal point characters into current_number
                current_number.push(c);
                // Check next character to decide if we should continue accumulating or process the number
                if let Some(&next_char) = input_chars.peek() {
                    if !next_char.is_digit(10) && next_char != '.' {
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

    Input_queue.push(Token::Operator(Operator::EndOfInput, Operator::EndOfInput.precedence_index()));
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
        },
        // Find the index of the top of the stack in the precedence table
        Some(Token::Operator(op, _)) => {
            index_first = op.precedence_index();
        },
        // None
        _ => {
            println!("Unexpected token or empty stack");
        }
        }

        match token {
            // If first Input_queue member is an operand, set index to 7
            Token::Operand(_) => {
                index_second = 7;
            },
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
                //stack.push(Token::Operator(Operator::PrecedenceStart, 41));
                let token_to_push_to_stack = Input_queue.pop().unwrap();
                stack.push(token_to_push_to_stack);
            },
            '>' => {
                // If input operator has higher precedence, pop operator from stack onto the Output_queue
                println!("Popping token from stack, pushing it to the output queue.");
                let token_to_push_to_queue = stack.pop().expect("Expected token on the stack");
                Output_queue.push(token_to_push_to_queue);
            },
            '=' => {
                // We pop from stack and Input_queue
                stack.pop().expect("Expected token '(' on the stack");
                Input_queue.pop().expect("Expected ')' on the stack");
            },
            's' => {
                // Special case where we matched $ with $ we end here!
                println!("End of input and end of precedence stack");
                return Ok(Output_queue);
            },
            'c' => {
                // Error
                println!("Conflict in precedence");
            },
            _ => {
                println!("Unexpected precedence value");
            }
        }
    }

    Ok(Output_queue)
}


pub fn parse(input: &str) -> String {
    match to_postfix(input) {
        Ok(postfix) => {
            let postfix_str = postfix.iter().map(|token| match token {
                Token::Operand(num) => num.to_string(),
                // Convert operators to their symbols or identifiers
                Token::Operator(op, _) => match op { 
                    Operator::Plus => "+".to_string(),
                    Operator::Minus => "-".to_string(),
                    Operator::Multiply => "*".to_string(),
                    Operator::Divide => "/".to_string(),
                    //Operator::Power => "^".to_string(),
                    //Operator::Root => "√".to_string(),
                    //Operator::Factorial => "!".to_string(),
                    Operator::OpenParen => "(".to_string(),
                    Operator::CloseParen => ")".to_string(),
                    Operator::EndOfInput => "$".to_string(), 
                    //Operator::Identifier => "i".to_string(), 
                    _ => " ".to_string(),
                },
            }).collect::<Vec<_>>().join(" ");
            postfix_str
        },
        Err(e) => e,
    }
}



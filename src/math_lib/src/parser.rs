// Author: Richard Gajdosik <gajdo33@vutbr.cz> 2024 VUT FIT
// Sources: https://www.fit.vutbr.cz/study/courses/IFJ/private/prednesy/Ifj08-en.pdf
//          https://github.com/RichardGajdosik/VUTFIT_IFJ_2021_Projekt/blob/master/src/expressions.c#L182

const LENGTH_OF_OPERATORS: usize = 1;
const LENGTH_OF_RULES: usize = 4;
const NUMBER_OF_OPERATORS: usize = 8;

// Operators are represented as single characters
static CHARS: [char; NUMBER_OF_OPERATORS] = ['*', '/', '+', '-', '(', ')', 'i', '$'];

// Rules are represented as strings
static RULES: [&str; NUMBER_OF_OPERATORS] = ["i", ")E(", "E+E", "E-E", "E*E", "E/E"];

// We define the precedence table as a 2D array
static PRECEDENCE_TABLE: [[char; NUMBER_OF_OPERATORS]; NUMBER_OF_OPERATORS] = [
    // *    /    +    -    (    )    i    $
    ['>', '>', '>', '>', '<', '>', '<', '>'], // *
    ['>', '>', '>', '>', '<', '>', '<', '>'], // /
    ['<', '<', '>', '>', '<', '>', '<', '>'], // +
    ['<', '<', '>', '>', '<', '>', '<', '>'], // -
    ['<', '<', '<', '<', '<', '=', '<', 'c'], // (
    ['>', '>', '>', '>', 'c', '>', 's', '>'], // )
    ['>', '>', '>', '>', 'c', '>', 's', '>'], // i
    ['<', '<', '<', '<', '<', 'c', '<', 'c'], // $
];


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
}

// Token enum representing either a value or an operator
#[derive(Debug, Clone, PartialEq)]
enum Token {
    Operand(f64),   // For numerical values
    Operator(Operator), // For operators including parentheses and end of input
}

pub fn parse(input: &str) -> String {
    let postfix = to_postfix(input);
    println!("{:?}", postfix)
    //format!("{}", input)
}

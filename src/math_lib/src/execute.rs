use std::fmt;

#[derive(Debug, Clone, PartialEq)]
enum Operator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Power,
    Root,
    Factorial,
}

#[derive(Debug, Clone, PartialEq)]
enum Token
{
    Value(f64),
    Operator(Operator),
}

/// Indicates malformation in parse tree
#[derive(Debug, Clone, PartialEq)]
pub struct EvaluationError;

impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Error while evaluating parse tree occurred")
    }
}

/// This function is doing the actual math calculations on the parse tree
/// When the parse tree is not valid or is malformed it will return [EvaluationError].
/// In numerical edge cases such as division by zero `inf` or `NaN` will be returned for more information please visit [f64] documentation
pub fn evaluate_parse_tree(parse_tree: String) -> Result<f64, EvaluationError> {
    let mut tokens: Vec<Token> = parse_tree
        .as_str()
        .split_whitespace()
        .map(|s| match s {
            "+" => Token::Operator(Operator::Add),
            "-" => Token::Operator(Operator::Subtract),
            "*" => Token::Operator(Operator::Multiply),
            "/" => Token::Operator(Operator::Divide),
            "R" => Token::Operator(Operator::Root),
            "!" => Token::Operator(Operator::Factorial),
            "^" => Token::Operator(Operator::Power),
            _ => Token::Value(s.parse::<f64>().unwrap_or(0.0)),
        })
        .collect::<Vec<Token>>();

    let mut iter = 0;
    loop {
        match tokens.get(iter) {
            None => {
                return Err(EvaluationError);
            }
            Some(Token::Value(_)) => {}
            Some(Token::Operator(op)) => {
                let op = op.clone();
                if iter >= 2 && tokens.len() >= 2 {
                    let a = match tokens.remove(iter - 2)
                    {
                        Token::Value(v) => v,
                        _ => return Err(EvaluationError),
                    };
                    let b = match tokens.remove(iter - 2)
                    {
                        Token::Value(v) => v,
                        _ => return Err(EvaluationError),
                    };

                    match op {
                        Operator::Add => { tokens[iter - 2] = Token::Value(a + b); }
                        Operator::Subtract => { tokens[iter - 2] = Token::Value(a - b); }
                        Operator::Multiply => { tokens[iter - 2] = Token::Value(a * b); }
                        Operator::Divide => { tokens[iter - 2] = Token::Value(a / b); }
                        Operator::Root => { tokens[iter - 2] = Token::Value(a.powf(1. / b)); }
                        Operator::Power => { tokens[iter - 2] = Token::Value(a.powf(b)); }
                        Operator::Factorial => {
                            let mut n_fac: f64 = 1.0;
                            for i in (b.round() as i64)..=(a.round() as i64) {
                                n_fac *= i as f64;
                            }
                            tokens[iter - 2] = Token::Value(n_fac);
                        }
                    }
                    iter = 0;
                }
            }
        }
        if tokens.len() == 1 {
            return match tokens.get(0)
            {
                Some(Token::Value(v)) => Ok(*v),
                _ => Err(EvaluationError)
            };
        }

        iter += 1;
        if iter > tokens.len() {
            return Err(EvaluationError);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_values() {
        assert_eq!(evaluate_parse_tree("2 2 +".to_string()), Ok(4.0));
    }

    #[test]
    fn subtract_two_values() {
        assert_eq!(evaluate_parse_tree("2 2 -".to_string()), Ok(0.0));
    }

    #[test]
    fn multiply_two_values() {
        assert_eq!(evaluate_parse_tree("2 4 *".to_string()), Ok(8.0));
    }

    #[test]
    fn divide_two_values() {
        assert_eq!(evaluate_parse_tree("8 2 /".to_string()), Ok(4.0));
    }

    #[test]
    fn power_of_two_values() {
        assert_eq!(evaluate_parse_tree("2 5 ^".to_string()), Ok(32.0));
    }

    #[test]
    fn sqrt_value() {
        assert_eq!(evaluate_parse_tree("16 2 R".to_string()), Ok(4.0));
    }

    #[test]
    fn factorial() {
        assert_eq!(evaluate_parse_tree("3 1 !".to_string()), Ok(6.0));
    }

    #[test]
    fn add_multiple_values() {
        assert_eq!(evaluate_parse_tree("2 2 + 2 4 + +".to_string()), Ok(10.0));
    }

    #[test]
    fn composed_equation_1() {
        assert_eq!(evaluate_parse_tree("3 2 1 + *".to_string()), Ok(9.0));
    }

    #[test]
    fn composed_equation_2() {
        assert_eq!(evaluate_parse_tree("3 4 * 2 5 * +".to_string()), Ok(22.0));
    }

    //These are invalid operations
    #[test]
    fn invalid_multiple_ops() {
        assert_eq!(
            evaluate_parse_tree("2 2 + * -".to_string()),
            Err(EvaluationError)
        );
    }

    #[test]
    fn invalid_character() {
        assert_eq!(
            evaluate_parse_tree("2 2 $".to_string()),
            Err(EvaluationError)
        );
    }
}

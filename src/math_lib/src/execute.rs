use crate::parser::*;
use std::fmt;
/// # Execute parse tree
/// This function gets on input mutable [Box] pointer to [Expression].
/// In case of inability to calculate value error message of type [String] will be returned.
/// [f64::NAN] is considered as valid numeric output.
pub fn execute_parse_tree(expression: &mut Box<Expression>) -> Result<f64, String> {
    match expression.as_mut() {
        Expression::Compound(first, second, op) => {
            match first.as_mut() {
                Expression::Compound(_, _, _) => {
                    let tmp = execute_parse_tree(&mut *first)?;
                    *first = Box::new(Expression::Value(tmp));
                }
                Expression::Value(_) => {}
            }
            match second.as_mut() {
                Expression::Compound(_, _, _) => {
                    let tmp = execute_parse_tree(&mut *second)?;
                    *second = Box::new(Expression::Value(tmp));
                }
                Expression::Value(_) => {}
            }
            let a = match first.as_ref() {
                Expression::Value(v) => Ok(v),
                _ => Err("Value not found".to_string()),
            }?;
            let b = match second.as_ref() {
                Expression::Value(v) => Ok(v),
                _ => Err("Value not found".to_string()),
            }?;

            match op {
                Operator::Plus => {
                    return Ok(a + b);
                }
                Operator::Minus => {
                    return Ok(a - b);
                }
                Operator::Multiply => {
                    return Ok(a * b);
                }
                Operator::Divide => {
                    return Ok(a / b);
                }
                Operator::Power => {
                    return Ok(a.powf(*b));
                }
                Operator::Root => {
                    return Ok(f64::powf(*a, 1. / (*b)));
                }
                Operator::Factorial => {
                    if *a < 0. {
                        return Err("Invalid factorial value".to_string());
                    }
                    let mut res = 1.0;
                    if *a > 1.0 {
                        (1..=(*a as i64)).for_each(|i| res *= i as f64);
                    }
                    return Ok(res);
                }
                _ => {
                    return Err(format!("{op:?} is invalid operator on execution layer"));
                }
            }
        }
        Expression::Value(v) => {
            return Ok(*v);
        }
    }
}

#[cfg(test)]
mod executor_tests {
    use super::*;

    #[test]
    fn add_two_values() {
        assert_eq!(
            Ok(2.0f64),
            execute_parse_tree(&mut Box::new(Expression::Compound(
                Box::new(Expression::Value(1.)),
                Box::new(Expression::Value(1.)),
                Operator::Plus
            )))
        );
    }

    #[test]
    fn subtract_two_values() {
        assert_eq!(
            Ok(0.0f64),
            execute_parse_tree(&mut Box::new(Expression::Compound(
                Box::new(Expression::Value(1.)),
                Box::new(Expression::Value(1.)),
                Operator::Minus
            )))
        );
    }

    #[test]
    fn multiply_two_values() {
        assert_eq!(
            Ok(4.0f64),
            execute_parse_tree(&mut Box::new(Expression::Compound(
                Box::new(Expression::Value(2.)),
                Box::new(Expression::Value(2.)),
                Operator::Multiply
            )))
        );
    }

    #[test]
    fn divide_two_values() {
        assert_eq!(
            Ok(4.0f64),
            execute_parse_tree(&mut Box::new(Expression::Compound(
                Box::new(Expression::Value(8.)),
                Box::new(Expression::Value(2.)),
                Operator::Divide
            )))
        );
    }

    #[test]
    fn power_of_two_values() {
        assert_eq!(
            Ok(9.0f64),
            execute_parse_tree(&mut Box::new(Expression::Compound(
                Box::new(Expression::Value(3.)),
                Box::new(Expression::Value(2.)),
                Operator::Power
            )))
        );
    }

    #[test]
    fn sqrt_value() {
        assert_eq!(
            Ok(3.0f64),
            execute_parse_tree(&mut Box::new(Expression::Compound(
                Box::new(Expression::Value(9.)),
                Box::new(Expression::Value(2.)),
                Operator::Root
            )))
        );
    }

    #[test]
    fn factorial() {
        assert_eq!(
            Ok(6.0f64),
            execute_parse_tree(&mut Box::new(Expression::Compound(
                Box::new(Expression::Value(3.)),
                Box::new(Expression::Value(1.)),
                Operator::Factorial
            )))
        );
    }
    #[test]
    fn add_multiple_values() {
        let inner_exp = Box::new(Expression::Compound(
            Box::new(Expression::Value(1.)),
            Box::new(Expression::Value(1.)),
            Operator::Plus,
        ));

        assert_eq!(
            Ok(3.0f64),
            execute_parse_tree(&mut Box::new(Expression::Compound(
                inner_exp,
                Box::new(Expression::Value(1.)),
                Operator::Plus
            )))
        );
    }
    #[test]
    fn divide_by_zero() {
        assert_eq!(
            Ok(f64::INFINITY),
            execute_parse_tree(&mut Box::new(Expression::Compound(
                Box::new(Expression::Value(8.)),
                Box::new(Expression::Value(0.)),
                Operator::Divide
            )))
        );
    }

    #[test]
    fn large_composite_equation() {
        // (3-1)*2 + (3!)
        let sub_subtraction = Box::new(Expression::Compound(
            Box::new(Expression::Value(3.0)),
            Box::new(Expression::Value(1.0)),
            Operator::Minus,
        ));
        let sub_mult = Box::new(Expression::Compound(
            sub_subtraction,
            Box::new(Expression::Value(2.0)),
            Operator::Multiply,
        ));
        let fact = Box::new(Expression::Compound(
            Box::new(Expression::Value(3.0)),
            Box::new(Expression::Value(1.0)),
            Operator::Factorial,
        ));
        assert_eq!(
            Ok(10.0),
            execute_parse_tree(&mut Box::new(Expression::Compound(
                sub_mult,
                fact,
                Operator::Plus
            )))
        );
    }

    // These are invalid operations
    #[test]
    fn invalid_negative_factorial() {
        assert_eq!(
            Err("Invalid factorial value".to_string()),
            execute_parse_tree(&mut Box::new(Expression::Compound(
                Box::new(Expression::Value(-5.)),
                Box::new(Expression::Value(1.)),
                Operator::Factorial
            )))
        );
    }

    #[test]
    fn invalid_operation() {
        assert_eq!(
            Err("OpenParen is invalid operator on execution layer".to_string()),
            execute_parse_tree(&mut Box::new(Expression::Compound(
                Box::new(Expression::Value(0.)),
                Box::new(Expression::Value(0.)),
                Operator::OpenParen
            )))
        );
        assert_eq!(
            Err("CloseParen is invalid operator on execution layer".to_string()),
            execute_parse_tree(&mut Box::new(Expression::Compound(
                Box::new(Expression::Value(0.)),
                Box::new(Expression::Value(0.)),
                Operator::CloseParen
            )))
        );
        assert_eq!(
            Err("EndOfInput is invalid operator on execution layer".to_string()),
            execute_parse_tree(&mut Box::new(Expression::Compound(
                Box::new(Expression::Value(0.)),
                Box::new(Expression::Value(0.)),
                Operator::EndOfInput
            )))
        );
    }
    #[test]
    fn negative_sqrt() {
        assert_eq!(
            Ok(0.25),
            execute_parse_tree(&mut Box::new(Expression::Compound(
                Box::new(Expression::Value(16.0)),
                Box::new(Expression::Value(-2.0)),
                Operator::Root
            )))
        );
        assert_eq!(
            true,
            execute_parse_tree(&mut Box::new(Expression::Compound(
                Box::new(Expression::Value(-16.0)),
                Box::new(Expression::Value(2.0)),
                Operator::Root
            )))
            .unwrap()
            .is_nan()
        );
    }
}

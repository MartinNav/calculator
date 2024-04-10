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
                    println!("{a} + {b}");
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
                    return Ok(f64::powf(*a,1. / (*b)));
                }
                Operator::Factorial => {
                    if *a<0. {
                        return Err("Invalid factorial value".to_string())
                    }
                    let mut res = 1.0;
                    if *a>1.0 {
                     (1..=(*a as i64)).for_each(|i| res*=i as f64);
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

    Err("Unimplemented".to_string())
}

#[cfg(test)]
mod tests {
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
                Operator::Plus
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

    /*
    #[test]
    fn composed_equation_1() {}

    #[test]
    fn composed_equation_2() {}
    */

    // These are invalid operations

    /*
    #[test]
    fn invalid_multiple_ops() {}

    #[test]
    fn invalid_multiple_numbers() {}

    #[test]
    fn invalid_root_of_negative() {}

    #[test]
    fn invalid_div_by_zero() {}

    #[test]
    fn invalid_character_val() {}

    #[test]
    fn invalid_character_op() {}
    */
}

use crate::parser::*;
use std::fmt;

pub fn execute_parse_tree(expression: Expression) -> Result<f64, String> {
    Err("Unimplemented".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_two_values() {
        assert_eq!(
            Ok(2.0f64),
            execute_parse_tree(Expression::Compound(
                Box::new(Expression::Value(1.)),
                Box::new(Expression::Value(1.)),
                Operator::Plus
            ))
        );
    }

    #[test]
    fn subtract_two_values() {
        assert_eq!(
            Ok(0.0f64),
            execute_parse_tree(Expression::Compound(
                Box::new(Expression::Value(1.)),
                Box::new(Expression::Value(1.)),
                Operator::Minus
            ))
        );
    }

    #[test]
    fn multiply_two_values() {
        assert_eq!(
            Ok(4.0f64),
            execute_parse_tree(Expression::Compound(
                Box::new(Expression::Value(2.)),
                Box::new(Expression::Value(2.)),
                Operator::Multiply
            ))
        );
    }

    #[test]
    fn divide_two_values() {
        assert_eq!(
            Ok(4.0f64),
            execute_parse_tree(Expression::Compound(
                Box::new(Expression::Value(8.)),
                Box::new(Expression::Value(2.)),
                Operator::Divide
            ))
        );
    }

    #[test]
    fn power_of_two_values() {
        assert_eq!(
            Ok(9.0f64),
            execute_parse_tree(Expression::Compound(
                Box::new(Expression::Value(3.)),
                Box::new(Expression::Value(2.)),
                Operator::Power
            ))
        );
    }

    #[test]
    fn sqrt_value() {
        assert_eq!(
            Ok(3.0f64),
            execute_parse_tree(Expression::Compound(
                Box::new(Expression::Value(9.)),
                Box::new(Expression::Value(2.)),
                Operator::Power
            ))
        );
    }

    #[test]
    fn factorial() {
        assert_eq!(
            Ok(6.0f64),
            execute_parse_tree(Expression::Compound(
                Box::new(Expression::Value(3.)),
                Box::new(Expression::Value(1.)),
                Operator::Power
            ))
        );
    }
    /*
    #[test]
    fn add_multiple_values() {}

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

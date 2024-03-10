use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub struct EvaluationError;
impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
        write!(f, "Error while evaluating parse tree occurred")
    }
}

pub fn evaluate_parse_tree(parse_tree: String)->Result<f64, EvaluationError>
{
    Err(EvaluationError)
}


#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn add_two_values() {
        assert_eq!(evaluate_parse_tree("2 2 +".to_string()), Ok(4.0));
    }

}


use std::fmt;
//use std::collections::LinkedList;
use std::collections::VecDeque;

/// [EvaluationError] indicates malformation in parse tree
#[derive(Debug, Clone, PartialEq)]
pub struct EvaluationError;
impl fmt::Display for EvaluationError {
    fn fmt(&self, f: &mut fmt::Formatter)->fmt::Result{
        write!(f, "Error while evaluating parse tree occurred")
    }
}
/// This function is doing the actual math calculations on the parse tree
/// When the parse tree is not valid or is malformed it will return [EvaluationError].
/// In numerical edge cases such as division by zero `inf` or `NaN` will be returned for more information please visit [f64] documentation 
pub fn evaluate_parse_tree(parse_tree: String)->Result<f64, EvaluationError>
{
    let mut tok_vec:Vec<String> = parse_tree.as_str().split_whitespace().collect::<Vec<_>>().iter().map(|s| s.to_string()).collect();
    let mut iter = 0;
    loop {
        
        match tok_vec.get(iter){
            Some(s)=>{
                let s = s.clone();
                if "+-*/!".contains(&s) {
                    let a = tok_vec.remove(iter-2).parse::<f64>().unwrap();//Just for now
                    let b = tok_vec.remove(iter-2).parse::<f64>().unwrap();
                match s.as_str(){
                "+"=>{
                    tok_vec[iter-2] =format!("{}", a+b);
                },
                "-"=>{
                    tok_vec[iter-2] =format!("{}", a-b);
                },
                "*"=>{
                    tok_vec[iter-2] =format!("{}", a*b);
                },
                "/"=>{
                    tok_vec[iter-2] =format!("{}", a/b);
                },

                _=>{
                            todo!();
                        }
                }
                iter=0;
                }
            },
            None=>{
                return Err(EvaluationError);
            }
        }
        iter+=1;
        if tok_vec.len() == 1{
            return match tok_vec.get(0).unwrap_or(&"Error".to_string()).parse::<f64>() {
                Ok(v)=>Ok(v),
                Err(_e)=>Err(EvaluationError),
            };
        }
        if iter>tok_vec.len() {
            break;//Most likely stuck
        }
    }
    Err(EvaluationError)
}


#[cfg(test)]
mod tests{
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
        assert_eq!(evaluate_parse_tree("2 2 + * -".to_string()), Err(EvaluationError));
    }
    #[test]
    fn invalid_character() {
        assert_eq!(evaluate_parse_tree("2 2 $".to_string()), Err(EvaluationError));
    }

}


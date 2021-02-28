use std::collections::HashMap;
use super::expression;

#[derive(Clone)]
pub struct Env {
  pub data: HashMap<String, expression::Expr>,
}

pub fn default_env() -> Env {
    let mut data: HashMap<String, expression::Expr> = HashMap::new();
    data.insert(
      "+".to_string(), 
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let sum = parse_list_of_floats(args)?.iter().fold(0.0, |sum, a| sum + a);
          
          Ok(expression::Expr::Number(sum))
        }
      )
    );
    data.insert(
      "-".to_string(), 
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let floats = parse_list_of_floats(args)?;
          let first = *floats.first().ok_or(expression::Err::Reason("expected at least one number".to_string()))?;
          let sum_of_rest = floats[1..].iter().fold(0.0, |sum, a| sum + a);
          
          Ok(expression::Expr::Number(first - sum_of_rest))
        }
      )
    );
    data.insert(
      "*".to_string(), 
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let floats = parse_list_of_floats(args)?;
          let first = *floats.first().ok_or(expression::Err::Reason("expected at least one number".to_string()))?;
          let sum_of_rest = floats[1..].iter().fold(first, |sum, a| sum * a);
          
          Ok(expression::Expr::Number(sum_of_rest))
        }
      )
    );    
    data.insert(
      "/".to_string(), 
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let floats = parse_list_of_floats(args)?;
          let first = *floats.first().ok_or(expression::Err::Reason("expected at least one number".to_string()))?;
          let sum_of_rest = floats[1..].iter().fold(first, |sum, a| sum / a);
          
          Ok(expression::Expr::Number(sum_of_rest))
        }
      )
    );    
    
    Env {data}
  }

  fn parse_list_of_floats(args: &[expression::Expr]) -> Result<Vec<f64>, expression::Err> {
    args
      .iter()
      .map(|x| parse_single_float(x))
      .collect()
  }
  
  fn parse_single_float(exp: &expression::Expr) -> Result<f64, expression::Err> {
    match exp {
        expression::Expr::Number(num) => Ok(*num),
      _ => Err(expression::Err::Reason("expected a number".to_string())),
    }
  }
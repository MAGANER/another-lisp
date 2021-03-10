use super::expression;

pub fn parse_list_of_floats(args: &[expression::Expr]) -> Result<Vec<f64>, expression::Err> 
{
    args
      .iter()
      .map(|x| parse_single_float(x))
      .collect()
}  
pub fn parse_single_float(exp: &expression::Expr) -> Result<f64, expression::Err> 
{
    //return parsed float and throw error, if it's not number
    match exp {
        expression::Expr::Number(num) => Ok(*num),
      _ => Err(expression::Err::Reason("expected a number".to_string())),
    }
}

pub fn parse_list_of_booleans(args: &[expression::Expr]) -> Result<Vec<bool>, expression::Err> 
{
    args
      .iter()
      .map(|x| parse_single_bool(x))
      .collect()
} 
pub fn parse_single_bool(expr: &expression::Expr) -> Result<bool,expression::Err>
{
  //return parsed bool value or throw error if it's not one
  match expr
  {
      expression::Expr::Bool(val) => Ok(*val),
      _ => Err(expression::Err::Reason("expected a boolean value".to_string()))
  }
}

pub fn parse_list_of_strings(args:&[expression::Expr]) -> Result<Vec<String>, expression::Err>
{
  args
    .iter()
    .map(|x| parse_single_string(x))
    .collect()
}
pub fn parse_single_string(expr: &expression::Expr) -> Result<String, expression::Err>
{
  match expr
  {
    expression::Expr::Symbol(val) => Ok(String::from(val).chars().filter(|x| *x != '"').collect()),
    _ => Err(expression::Err::Reason("expected a string value".to_string()))
  }
}
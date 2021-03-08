use std::collections::HashMap;
use std::process;
use super::expression;

#[derive(Clone)]
pub struct Env {
  pub data  : HashMap<String, expression::Expr>
}

pub fn default_env() -> Env
{
    //init basic operations, functions e.t.c

    let mut data: HashMap<String, expression::Expr> = HashMap::new();
    data.insert(
      "+".to_string(), 
      expression::Expr::Func
      (
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> 
        {
          let floats = parse_list_of_floats(args);
          match floats
          {
            Ok (val) => {  
                          let result = val.iter().fold(0.0,|a,b| a + b);
                          Ok(expression::Expr::Number(result)) 
                      },
            _ => 
            {
              let strings = parse_list_of_strings(args);
              match strings
              {
                Ok(val)      => {
                                    let result = val.iter().fold(String::from(""),|a:String,b:&String| a+b );
                                    Ok(expression::Expr::Symbol(result))
                                },
                Err(err_val) => match err_val
                                {
                                  expression::Err::Reason(v) => {
                                                                    println!("{}",v);
                                                                    process::exit(-1);
                                                                }
                                } 
              }
            }
          }

      
        }
      )
    );
    data.insert(
      "-".to_string(), 
      expression::Expr::Func
      (
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> 
        {
          let floats = parse_list_of_floats(args);
          match floats
          {
            Ok(val) => {  
                          let first       = val.first().unwrap();

                          let sum_of_rest = val[1..].iter().fold(0.0, |sum, a| sum + a);
                          Ok(expression::Expr::Number(first - sum_of_rest)) 
                      },
            Err(val) =>
            {
              match val
              {
                expression::Err::Reason(v) => 
                {
                     println!("{}",v);
                    process::exit(-1);
                }
              }
            } 
          }

      
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
    data.insert(
      "<".to_string(),
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let floats = parse_list_of_floats(args);
          let less   = |a:f64,b:f64| a < b;
          apply_logic_op(less, floats)
        }
      )
    ); 
    data.insert(
      ">".to_string(),
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let floats = parse_list_of_floats(args);
          let more = |a:f64,b:f64| a > b;
          apply_logic_op(more, floats)
        }
      )
    );
    data.insert(
      "=".to_string(),
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let floats = parse_list_of_floats(args);
          match floats
          {
            Ok (val) => {  
                          let first:f64  = *val.iter().nth(0).unwrap(); 
                          let result = val.iter().all(|x| first == *x);
                          Ok(expression::Expr::Bool(result)) 
                      },
            _ => 
            {
              let strings = parse_list_of_strings(args);
              match strings
              {
                Ok(val)      => {
                                    let first:String  = val.iter().nth(0).unwrap().clone(); 
                                    let result = val.iter().all(|x| first == *x);
                                    Ok(expression::Expr::Bool(result))
                                },
                Err(err_val) => match err_val
                                {
                                  expression::Err::Reason(v) => {
                                                                    println!("{}",v);
                                                                    process::exit(-1);
                                                                }
                                } 
              }
            }
          }

        }
      )
    );
    data.insert(
      ">=".to_string(),
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let floats = parse_list_of_floats(args);
          let more_eq = |a:f64,b:f64| a >= b;
          apply_logic_op(more_eq, floats)
        }
      )
    );
    data.insert(
      "<=".to_string(),
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let floats = parse_list_of_floats(args);
          let less_eq = |a:f64,b:f64| a <= b;
          apply_logic_op(less_eq, floats)
        }
      )
    );
    data.insert(
      "&&".to_string(),
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let booleans = parse_list_of_booleans(args);

          let result =
          match booleans
          {
            Ok(val) =>  val.iter().fold(val[0],|a,b| a && *b),
            Err(err)=> {
                          match err
                          {
                          expression::Err::Reason(v) => { println!("{}",v);
                                                process::exit(-1); 
                          }
                        }
          }
          };

          Ok(expression::Expr::Bool(result)) 
        }
      )
    );
    data.insert(
      "||".to_string(),
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let booleans = parse_list_of_booleans(args);

          let result =
          match booleans
          {
            Ok(val) =>  val.iter().fold(val[0],|a,b| a || *b),
            Err(err)=> {
                          match err
                          {
                          expression::Err::Reason(v) => { println!("{}",v);
                                                process::exit(-1); 
                          }
                        }
          }
          };

          Ok(expression::Expr::Bool(result)) 
        }
      )
    );
    data.insert(
      "!".to_string(),
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let booleans = parse_list_of_booleans(args);

          let result =
          match booleans
          {
            Ok(val) =>  {
                            if val.len() != 1
                            {
                              println!("! takes 1 argument only!");
                            }
                            !val[0]
                        },
            Err(err)=> {
                          match err
                          {
                          expression::Err::Reason(v) => { println!("{}",v);
                                                process::exit(-1); 
                          }
                        }
          }
          };

          Ok(expression::Expr::Bool(result)) 
        }
      )
    );
    Env {data}
  }

fn parse_list_of_floats(args: &[expression::Expr]) -> Result<Vec<f64>, expression::Err> 
{
    args
      .iter()
      .map(|x| parse_single_float(x))
      .collect()
}  
fn parse_single_float(exp: &expression::Expr) -> Result<f64, expression::Err> 
{
    //return parsed float and throw error, if it's not number
    match exp {
        expression::Expr::Number(num) => Ok(*num),
      _ => Err(expression::Err::Reason("expected a number".to_string())),
    }
}

fn parse_list_of_booleans(args: &[expression::Expr]) -> Result<Vec<bool>, expression::Err> 
{
    args
      .iter()
      .map(|x| parse_single_bool(x))
      .collect()
} 
fn parse_single_bool(expr: &expression::Expr) -> Result<bool,expression::Err>
{
  //return parsed bool value or throw error if it's not one
  match expr
  {
      expression::Expr::Bool(val) => Ok(*val),
      _ => Err(expression::Err::Reason("expected a boolean value".to_string()))
  }
}

fn parse_list_of_strings(args:&[expression::Expr]) -> Result<Vec<String>, expression::Err>
{
  args
    .iter()
    .map(|x| parse_single_string(x))
    .collect()
}
fn parse_single_string(expr: &expression::Expr) -> Result<String, expression::Err>
{
  match expr
  {
    expression::Expr::Symbol(val) => Ok(String::from(val).chars().filter(|x| *x != '"').collect()),
    _ => Err(expression::Err::Reason("expected a string value".to_string()))
  }
}

type LogOp = fn(f64,f64) -> bool;
fn apply_logic_op(f:LogOp,floats:Result<Vec<f64>, expression::Err>) -> Result<expression::Expr, expression::Err>
{
  /*
      first of all check are values of right type,
      then compute it and return the result.
  */
  let result = 
  match floats
  {
    Ok(val) => {
                    if val.len() < 2
                    {
                      println!("not enough arguments for >");
                      process::exit(-1);
                    }
                    compute_logic_op(f, &val)
               },
    Err(err_val) => {
                        match err_val
                        {
                          expression::Err::Reason(v) => { println!("{}",v);
                                                          process::exit(-1); 
                                                        }
                        }
                    },

            
  };

  Ok(expression::Expr::Bool(result))  
}
fn compute_logic_op (f:LogOp,val:&Vec<f64>) -> bool
{
  /*
      compute boolean function, reducing arguments,
      like python's reduce
  */
    let mut results:Vec<bool> = Vec::new();
    let mut counter = 0;
    while counter < val.len()
    {
      let curr = val[counter];
      if counter+1 < val.len()
      {
        let next = val[counter+1];
        let result = f(curr,next);
        results.push(result);
      }
      counter += 1;
   }

    results.iter().fold(results[0],|a,b| a && *b)
}
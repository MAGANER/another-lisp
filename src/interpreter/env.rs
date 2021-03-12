use std::collections::HashMap;
use std::process;
use super::expression;

mod arg_parser;
mod logic_op_computer;

/*
  Envinronment is hash table, containg
  basic operations, variables, functions.

*/

#[derive(Clone)]
pub struct Env {
  pub data  : HashMap<String, expression::Expr>
}

pub fn default_env() -> Env
{
    //init basic operations, functions e.t.c

    let mut data: HashMap<String, expression::Expr> = HashMap::new();

    //arithmetic
    data.insert(
      "+".to_string(), 
      expression::Expr::Func
      (
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> 
        {
          let floats = arg_parser::parse_list_of_floats(args);
          match floats
          {
            Ok (val) => {  
                          let result = val.iter().fold(0.0,|a,b| a + b);
                          Ok(expression::Expr::Number(result)) 
                      },
            _ => 
            {
              let strings = arg_parser::parse_list_of_strings(args);
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
          let floats = arg_parser::parse_list_of_floats(args);
          
          match floats
          {
            Ok(val) => {  
                          if val.len() == 0
                          {
                            //just return zero
                            return Ok(expression::Expr::Number(0.0f64));
                          }
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
      expression::Expr::Func
      (
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> 
        {
          let floats = arg_parser::parse_list_of_floats(args);
          match floats
          {
            Ok(val) =>
            {
              if val.len() == 0
              {
                return Ok(expression::Expr::Number(0.0f64))
              }
              let first = val.first().unwrap();
              let result= val[1..].iter().fold(first.clone(),|a,b| a * b).clone();
              Ok(expression::Expr::Number(result))
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
      "/".to_string(), 
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> 
        {
          let floats = arg_parser::parse_list_of_floats(args);
          match floats
          {
            Ok(val) =>
            {
              if val.len() == 0
              {
                return Ok(expression::Expr::Number(0.0f64));
              }

              let first = val.first().unwrap();
              let result= val[1..].iter().fold(first.clone(), |a, b| a / b);

              Ok(expression::Expr::Number(result))
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
    
    //comparasion
    data.insert(
      "<".to_string(),
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let floats = arg_parser::parse_list_of_floats(args);
          let less   = |a:f64,b:f64| a < b;
          logic_op_computer::apply_logic_op(less, floats)
        }
      )
    ); 
    data.insert(
      ">".to_string(),
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let floats = arg_parser::parse_list_of_floats(args);
          let more = |a:f64,b:f64| a > b;
          logic_op_computer::apply_logic_op(more, floats)
        }
      )
    );
    data.insert(
      "=".to_string(),
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let floats = arg_parser::parse_list_of_floats(args);
          match floats
          {
            Ok (val) => {  
                          let first:f64  = *val.iter().nth(0).unwrap(); 
                          let result = val.iter().all(|x| first == *x);
                          Ok(expression::Expr::Bool(result)) 
                      },
            _ => 
            {
              let strings = arg_parser::parse_list_of_strings(args);
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
          let floats = arg_parser::parse_list_of_floats(args);
          let more_eq = |a:f64,b:f64| a >= b;
          logic_op_computer::apply_logic_op(more_eq, floats)
        }
      )
    );
    data.insert(
      "<=".to_string(),
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let floats = arg_parser::parse_list_of_floats(args);
          let less_eq = |a:f64,b:f64| a <= b;
          logic_op_computer::apply_logic_op(less_eq, floats)
        }
      )
    );
    
    //logic
    data.insert(
      "&&".to_string(),
      expression::Expr::Func(
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> {
          let booleans = arg_parser::parse_list_of_booleans(args);

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
          let booleans = arg_parser::parse_list_of_booleans(args);

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
      expression::Expr::Func
      (
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> 
        {
          let booleans = arg_parser::parse_list_of_booleans(args);

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

    //list operations
    data.insert(
      "head".to_string(),
      expression::Expr::Func
      (
        |args: &[expression::Expr]| -> Result<expression::Expr, expression::Err> 
        {
            //return list head
            if args.len() != 1
            {
              println!("head operation takes 1 argument only!");
              process::exit(-1);
            }
            
            match args[0].clone()
            {
                expression::Expr::List(val) =>
                {

                  let head = val.first();
                  match head
                  {
                      Some(val) => Ok(val.clone()),
                      None      => Ok(expression::Expr::Bool(false))
                  }
                },
                _ =>
                {
                  println!("head operation takes only List!");
                  process::exit(-1);
                }
            }
        }
      )
    );


    Env {data}
  }




pub fn unite_environments(env1:&Env, env2:&Env) -> Env
{
    let mut new_env = env1.clone();
    for (name,val) in env2.data.iter()
    {
      new_env.data.insert(name.to_string(),val.clone());
    }
    new_env
}
/*
    Sub module of eval.
    Contains functions to compute and define lambdas.


*/

use super::expression;
use super::env;
use super::eval;
use std::collections::HashMap;

pub fn define_lambda(arg_forms:&[expression::Expr]) -> Result<expression::Expr, expression::Err>
{
  //create lambda

  //get arguments
  let args = &arg_forms[0];
  let _args = match args
  {
    expression::Expr::List(_) => args,
    _ => 
    {
      println!("arguments must be list!");
      std::process::exit(-1);
    }
  };
  
  let body = &arg_forms[1..];
  let lambda = expression::LambdaStruct
  {
    arg :Box::new(_args.clone()),
    body:Box::new(body.to_vec())
  };
  Ok(expression::Expr::Lambda(lambda))
}
pub fn compute_lambda(lambda:&expression::LambdaStruct,
                     args   :&[expression::Expr],
                     env    :&mut env::Env) -> Result<expression::Expr, expression::Err> 
{
    //get the vector of expression
    let fn_args = match lambda.arg.as_ref()
    {
      expression::Expr::List(val) => val,
      _ =>
      {
          println!("can not extract lambda arguments!");
          std::process::exit(-1);
      }
    };
    if args.len() != fn_args.len()
    {
        println!("not enough arguments passed to lambda!");
        println!("passed:{}",args.len());
        println!("expected:{}",fn_args.len());
        std::process::exit(-1);
    }
  
    //create vector of arguments name
    let names = {
                  let mut names:Vec<String> = Vec::new();
                  for arg in fn_args
                  {
                      match arg
                      {
                        expression::Expr::Symbol(val) => names.push(val.clone()),
                        _ =>
                        {
                          println!("argument name must be string!");
                          std::process::exit(-1);
                        }
                      }
                  }
                  names
                };
    
    //init temporal environment
    let mut temp_env = env::Env{data:HashMap::new()};

    let max = names.len();
    for counter in 0..max
    {
        let evaled_arg =
        {
          match eval(&args[counter],env)
          {
              Ok(val)  => val,
              Err(err) =>
              {
                  match err
                  {
                      expression::Err::Reason(val) =>
                      {
                          println!("{}",val);
                          std::process::exit(-1);
                      }
                  }
              }
          }
      };
      temp_env.data.insert(names[counter].clone(),evaled_arg.clone());
    }
    temp_env = env::unite_environments(env,&temp_env);


      let mut sub_res = expression::Expr::Bool(false);
      let mut counter = 0;
      while counter < lambda.body.len()
      {
        let expr = lambda.body.iter().nth(counter).unwrap().clone();
        let result = eval(&expr.clone(),&mut temp_env);
        sub_res = match result
        {
            Ok(val) => match val
                       {
                         expression::Expr::Repeat => {counter = 0; expression::Expr::Bool(false)},
                         _ => { counter += 1;val}
                       },
            Err(err_val) =>
            {
              match err_val
              {
                  expression::Err::Reason(v) =>
                  {
                  println!("{}",v);
                  std::process::exit(-1);
                  }
              }
            }
          };
        }


  Ok(sub_res)
}
pub fn add_lambda_to_env(arg_forms: &[expression::Expr], env: &mut env::Env) -> Result<expression::Expr, expression::Err>
{
      if arg_forms.len() != 2
      {
        println!("can not define lambda!\n not enough arguments!");
        std::process::exit(-1);
      }
      let lambda_def = match &arg_forms[1]
      {
            expression::Expr::List(val) => val,
            _ =>
            {
                println!("can not process fn argument!");
                std::process::exit(-1);
            }
      };

      let lambda = define_lambda(&lambda_def[1..]);
      let name   = match arg_forms[0].clone()
      {
          expression::Expr::Symbol(v) => v,
          _ =>
          {
            println!("function name should be String!");
            std::process::exit(-1);
          }
      };
      env.data.insert(name,lambda.unwrap());
  
      Ok(expression::Expr::Bool(true))
}
pub fn process_lambda(args  :&[expression::Expr],
                      env   :&mut env::Env) -> Result<expression::Expr, expression::Err>
{
  //define and execute lambda

  if args.len() == 2
  {
    return Ok(define_lambda(&args).unwrap());
  }
  else if args.len() > 2
  {
    //define lambda and compute it
    let lambda = match define_lambda(&args[0..2]).unwrap()
                {
                  expression::Expr::Lambda(val) => val,
                  _ =>  {
                           println!("gz! you reached the impossible point!");
                           std::process::exit(-2);
                        }
                };

    let result = compute_lambda(&lambda,&args[2..],env).unwrap();
    Ok(result)
  }
  else
  {
    println!("can not process lambda!\n not enough arguments!");
    std::process::exit(-1);
  }
}
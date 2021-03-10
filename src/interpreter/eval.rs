use super::expression;
pub use super::env;
use std::collections::HashMap;

pub fn eval(exp: &expression::Expr, env: &mut env::Env) -> Result<expression::Expr, expression::Err> 
{
    match exp 
    {
      expression::Expr::Number(_) => Ok(exp.clone()),
      expression::Expr::Bool(_)   => Ok(exp.clone()),
      expression::Expr::Symbol(k) =>
      {
         if expression::is_string_value(k)
        {
             Ok(exp.clone())
        } else 
              {    
                 env.data.get(k)
                         .ok_or(expression::Err::Reason(format!("unexpected symbol k={}", k)))
                         .map(|x| x.clone())
              }
        },
      expression::Expr::List(list) => 
      {
        let first_form = list
                          .first()
                          .ok_or(expression::Err::Reason("expected a non-empty list".to_string()))?;

        let arg_forms = &list[1..];
        match eval_built_in_form(first_form, arg_forms, env)
        {
          Some(res) => res, //already computed
          None      =>      //no built-in function is used
          {
            //try to find operation in enviroment
              let first_eval = eval(first_form, env)?;
              match first_eval 
              {
                  expression::Expr::Func(f) => 
                  {
                        let args_eval = compute_expr_list(arg_forms,env);
                        f(&args_eval?)
                  },
                  _ => Err(
                        expression::Err::Reason("first form must be a function".to_string())
                          ),
              }
          }
        }
      },
      _ => Err(
        expression::Err::Reason("unexpected function!".to_string())
      ),
    }
  }
fn eval_built_in_form
(exp: &expression::Expr, arg_forms: &[expression::Expr], env: &mut env::Env) 
-> Option<Result<expression::Expr, expression::Err>> 
{
  //if expression is built in special function compute and return result
  //or return just none
  match exp 
  {
    expression::Expr::Symbol(s) => 
      match s.as_ref() {
        "if"    => Some(compute_if(arg_forms , env)),
        "def"   => Some(compute_def(arg_forms, env)),
        "fn"    => Some(process_lambda(arg_forms, env)),
        "print" => {print_data(arg_forms, env); Some(Ok(expression::Expr::Bool(true)))},
        _       => None,
      },
    _ => None,
  }
}
fn compute_if(arg_forms: &[expression::Expr], env: &mut env::Env) -> Result<expression::Expr, expression::Err>
{
  //compute it's body, if condition is true

  //get the head and tail of args
  let test_form = arg_forms.first().ok_or(
    expression::Err::Reason(
      "expected argument to check it's logical value!".to_string(),
    )
  )?;

  //compute the condition
  let test_eval = eval(test_form, env)?;
  match test_eval 
  {
    expression::Expr::Bool(b) => 
          {
              //if argument is true, then compute first expression
              //unless try to compute the second one
              let form_idx = if b { 1 } else { 2 };
              let res_form = arg_forms.get(form_idx)
                                      .ok_or(expression::Err::Reason( format!("can not find option to do={}",form_idx)))?;
              let res_eval = eval(res_form, env);
    
              res_eval
          },
    _ => Err(expression::Err::Reason(format!("unexpected argument! it should be Bool!")))
  }
}
fn compute_def(arg_forms: &[expression::Expr], env: &mut env::Env) -> Result<expression::Expr, expression::Err> 
{
  //init variable into the environment

  //it takes only variable name and it's value(atom or expression to compute)
  if arg_forms.len() != 2 
  {
    return Err(expression::Err::Reason("def can only have two arguments ".to_string()))
  } 

  //try to get head of list and arguments
  let first_form = arg_forms.first()
                            .ok_or(expression::Err::Reason("expected first argument".to_string(),))?;

       
  //get the name of variable
  let name = 
  match first_form 
  {
    expression::Expr::Symbol(s) => Ok(s.clone()),
    _                           => Err(expression::Err::Reason( "expected first argument to be a symbol".to_string()))
  }?;

  //get the value of var
  let val = arg_forms.get(1)
                    .ok_or(expression::Err::Reason("expected second argument".to_string()))?;

  //compute it and then add it to environment
  let second_eval = eval(val, env)?;
  env.data.insert(name, second_eval);
  
  Ok(first_form.clone())
}
fn define_lambda(arg_forms:&[expression::Expr]) -> Result<expression::Expr, expression::Err>
{
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

  let body = &arg_forms[1];

  let lambda = expression::LambdaStruct
  {
    arg :Box::new(_args.clone()),
    body:Box::new(body.clone())
  };
  Ok(expression::Expr::Lambda(lambda))
}
fn compute_lambda(lambda:&expression::LambdaStruct,
                  args  :&[expression::Expr],
                  env   :&mut env::Env) -> Result<expression::Expr, expression::Err> 
{

    //get the vector of expression
    let fn_args = match &*lambda.arg
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
      temp_env.data.insert(names[counter].clone(),args[counter].clone());
    }
    temp_env = env::unite_environments(env,&temp_env);

    let result = eval(&lambda.body,&mut temp_env);
    *env = env::substract_environments(env,&mut temp_env);
    match result
    {
      Ok(_) => result,
      Err(val) =>
      {
        match val
        {
          expression::Err::Reason(v) =>
          {
            println!("{}",v);
            std::process::exit(-1);
          }
        }
      }
    }

}
fn process_lambda(args  :&[expression::Expr],
                  env   :&mut env::Env) -> Result<expression::Expr, expression::Err>
{
  //define and execute lambda

  if args.len() > 2
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


fn print_data(arg_forms: &[expression::Expr],env:&mut env::Env)
{
  //print all arguments and compute it if it's required


  let mut counter = 0;
  while counter < arg_forms.len()
  {
    let arg = &arg_forms[counter];
    let result = eval(arg,env);
    match result
    {
      Ok(val) =>print(&val),
      Err(err_val) =>
      {
          match err_val
          {
            expression::Err::Reason(v) => println!("{}",v.to_string())
          }
      }
    }

    counter += 1;
  }
}
pub fn print(expr:&expression::Expr)
{
    match expr
    {
        expression::Expr::Bool(v)   => {
                                            if *v {println!("True");}
                                            else {println!("False");}
                                       },
        expression::Expr::Symbol(v) => println!("{}",v),
        expression::Expr::Number(v) => println!("{}",v),
        expression::Expr::List(v)   => {
                                            for elem in v.iter()
                                            {
                                                print(elem);
                                            }

                                       },
        _ => ()
    }
}

fn compute_expr_list(arg_forms: &[expression::Expr], env: &mut env::Env) 
-> Result<Vec<expression::Expr>, expression::Err> 
{
  arg_forms
    .iter()
    .map(|x| eval(x, env))
    .collect()
}
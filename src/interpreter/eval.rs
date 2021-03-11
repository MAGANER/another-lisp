use super::expression;
use super::env;

mod lambda;

pub fn eval(exp: &expression::Expr, env: &mut env::Env) -> Result<expression::Expr, expression::Err> 
{
  //compute expression, using passed environment
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

        //check is it function call
        match first_form
        {
            expression::Expr::Symbol(val) =>
            {
                if is_fn_call(val,env)
                {
                  let lambda = match env.data.get(&val[0..val.len()-1].to_string()).unwrap()
                  {
                    expression::Expr::Lambda(val) => val.clone(),
                    _ =>
                    {
                      println!("can not obtain lambda!");
                      std::process::exit(-1);
                    }
                  };
                  let args = match &list[1]
                  {
                      expression::Expr::List(val) => val.clone(),
                      _ =>
                      {
                        println!("arguments must be contained with List!");
                        std::process::exit(-1);
                      }
                  };
                  return lambda::compute_lambda(&lambda,&args,env);
                }
            },
            _ => ()
        };

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
        "fn"    => Some(lambda::process_lambda(arg_forms, env)),
        "defn"  => Some(lambda::add_lambda_to_env(arg_forms,env)),
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

fn is_fn_call(name:&String, env:&env::Env) -> bool
{
  name.chars().nth(name.len()-1).unwrap() == '!' && env.data.contains_key(&name[0..name.len()-1].to_string())
}
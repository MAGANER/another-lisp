use super::expression;
pub use super::env;

pub fn eval(exp: &expression::Expr, env: &mut env::Env) -> Result<expression::Expr, expression::Err> {
    match exp {
        expression::Expr::Symbol(k) =>
          
          if expression::is_string_value(k)
          {
              Ok(exp.clone())
          } else 
          {
            env.data.get(k)
            .ok_or(
              expression::Err::Reason(
                format!("unexpected symbol k={}", k)
              )
            )
            .map(|x| x.clone())
          }
      ,
      expression::Expr::Number(_a) => Ok(exp.clone()),
      expression::Expr::Bool(_a)   => Ok(exp.clone()),
      expression::Expr::List(list) => {
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
                  expression::Expr::Func(f) => {
                  let args_eval = arg_forms
                  .iter()
                  .map(|x| eval(x, env))
                  .collect::<Result<Vec<expression::Expr>, expression::Err>>();
                  f(&args_eval?)
              },
              _ => Err(
                        expression::Err::Reason("first form must be a function".to_string())
                      ),
              }
          }
        }
      },
      expression::Expr::Func(_) => Err(
        expression::Err::Reason("unexpected form".to_string())
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
        "if"  => Some(compute_if(arg_forms,env)),
        "def" => None, //Some(eval_def_args(arg_forms, env)),
        _     => None,
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
      "expected test form".to_string(),
    )
  )?;

  let test_eval = eval(test_form, env)?;
  match test_eval 
  {
    expression::Expr::Bool(b) => 
          {
              //if argument is true, then compute first expression
              //unless try to compute the second one
              let form_idx = if b { 1 } else { 2 };
              let res_form = arg_forms.get(form_idx)
                                      .ok_or(expression::Err::Reason( format!("expected form idx={}", form_idx))
                                      )?;
              let res_eval = eval(res_form, env);
    
              res_eval
          },
    _ => Err(expression::Err::Reason(format!("unexpected test form")))
  }
}
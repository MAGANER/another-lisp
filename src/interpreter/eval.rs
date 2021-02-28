use super::expression;
pub use super::env;

pub fn eval(exp: &expression::Expr, env: &mut env::Env) -> Result<expression::Expr, expression::Err> {
    match exp {
        expression::Expr::Symbol(k) =>
          env.data.get(k)
          .ok_or(
            expression::Err::Reason(
              format!("unexpected symbol k='{}'", k)
            )
          )
          .map(|x| x.clone())
      ,
      expression::Expr::Number(_a) => Ok(exp.clone()),
      expression::Expr::List(list) => {
        let first_form = list
          .first()
          .ok_or(expression::Err::Reason("expected a non-empty list".to_string()))?;
        let arg_forms = &list[1..];
        let first_eval = eval(first_form, env)?;
        match first_eval {
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
      },
      expression::Expr::Func(_) => Err(
        expression::Err::Reason("unexpected form".to_string())
      ),
    }
  }
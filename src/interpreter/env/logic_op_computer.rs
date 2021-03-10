use std::process;
use super::expression;


pub type LogOp = fn(f64,f64) -> bool;

pub fn apply_logic_op(f:LogOp,floats:Result<Vec<f64>, expression::Err>) -> Result<expression::Expr, expression::Err>
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
pub fn compute_logic_op (f:LogOp,val:&Vec<f64>) -> bool
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
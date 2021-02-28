use std::env;
use std::fs;
use std::process;

mod interpreter;
use interpreter::parser;
use interpreter::eval;
use interpreter::expression;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() ==  1 {
        println!("not enough arguments!");
        process::exit(0);
    }

    let mut env = eval::env::default_env();

    let script = fs::read_to_string(&args[1]).expect("can not open file!");
    let tokens = parser::tokenize(script);
    let tree   = parser::parse(&tokens);
    let _result = eval::eval(&tree.unwrap().0,&mut env);
    match _result
    {
        Ok(val) =>  match val
                        {
                            expression::Expr::Symbol(v) => println!("{}",v),
                            expression::Expr::Number(v) => println!("{}",v),
                            expression::Expr::List(_)   => (),
                            expression::Expr::Func(_)   => (),
                        },
        Err(err_val)=> match err_val
                        {
                            expression::Err::Reason(v) => println!("{}",v)
                        }
    };
}

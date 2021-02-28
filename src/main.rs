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
    let result = eval::eval(&tree.unwrap().0,&mut env);


    let result = result.unwrap();
    match result
    {
      expression::Expr::Symbol(val) => println!("{}",val),
      expression::Expr::Number(val) => println!("{}",val),
      expression::Expr::List(_)   => (),
      expression::Expr::Func(_)   => (),
    };
 
}

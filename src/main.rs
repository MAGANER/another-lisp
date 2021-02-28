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
    let trees  = parser::split_trees(tokens);

    run(trees,&mut env);
}
fn run(trees:Vec<Vec<String>>,env:&mut eval::env::Env)
{
    for tree in trees.iter()
    {
        let parsed   = parser::parse(&tree);
        let _tree = match parsed
                {
                    Ok(val)      => val.0,
                    Err(err_val) => match err_val
                                {
                                    expression::Err::Reason(v) => 
                                    {
                                        println!("{}",v);
                                        process::exit(-1);  
                                    }         
                                }
                };
    let _result = eval::eval(&_tree, env);
    match _result
    {
        Ok(val)     =>  print(&val),
        Err(err_val)=> match err_val
                        {
                            expression::Err::Reason(v) => println!("{}",v)
                        }
    };

    }
}
fn print(expr:&expression::Expr)
{
    match expr
    {
        expression::Expr::Symbol(v) => println!("{}",v),
        expression::Expr::Number(v) => println!("{}",v),
        expression::Expr::List(v)   => {
                                            for elem in v.iter()
                                            {
                                                print(elem);
                                            }

                                       },
        expression::Expr::Func(_)   => (),   
    }
}

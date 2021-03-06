use std::env;
use std::fs;
use std::process;

mod interpreter;
use interpreter::parser;
use interpreter::eval;
use interpreter::expression;
use interpreter::execution_options;

fn main() {
    let mut options = execution_options::init_exec_options();
    let args:Vec<String> = env::args().collect();
    if args.len() ==  1 {
        println!("not enough arguments!");
        process::exit(0);
    }
    if args.contains(&"print".to_string())
    {
        options.print_every_operation = true;
    }

    let mut env = eval::env::default_env();

    let script = fs::read_to_string(&args[1]).expect("can not open file!");
    let tokens = parser::tokenize(script);
    let trees  = parser::split_trees(tokens);

    run(trees,&mut env,&options);
}
fn run(trees:Vec<Vec<String>>,env:&mut eval::env::Env,options:&execution_options::ExecutionOptions)
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
        Ok(val)     =>  if options.print_every_operation {eval::print(&val) },
        Err(err_val)=> match err_val
                        {
                            expression::Err::Reason(v) => println!("{}",v)
                        }
    };

    }
}


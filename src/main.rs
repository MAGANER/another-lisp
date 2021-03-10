/*
    Creator and maintainter:Yan Lomakin-Gorbatenko

    A little bit about project:
      Puprose is to learn Rust and, by the way, 
      to  create maybe another dialect of Lisp.
      
    Future goal is to create language to use 
    instead of bash and its analogues.


        #Abilities
        0) Basic arithmetic: +,-,*,/
        1) Basic logic     : >, <, =, >=, <=, !, &&, ||
        2) Output          : (print a b)
                Prints all passed arguments. Compute it, if required.

        3)If    : (if (condition) var1 var2)
            If condition is true, then compute first expression,
            unless try to compute the second one, if it exists.

        4)Variable (re)definition : def
            Takes 2 arguments: name and value

        5)Lambdas: (fn (a) (+ a 1))
            Also you can execute it:
            (print
                (fn (a) (+ a 1) 2)
            )

            You can define it to use somewhere else:
                (defn lmd
                    (fn (arg) (print arg))
                )

                (lmd! (2))

        6)One line comment:
            #(print "it won't never be printed")



*/

use std::env;
use std::fs;
use std::process;

mod interpreter;
use interpreter::parser;
use interpreter::eval;
use interpreter::expression;
use interpreter::execution_options;

fn main() 
{
    //process arguments, if they are passed
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

    //init env, read, parse and execute the script
    let mut env = eval::env::default_env();

    let script = fs::read_to_string(&args[1]).expect("can not open file!");
    let tokens = parser::tokenize(script);
    let trees  = parser::split_trees(tokens);

    run(trees,&mut env,&options);
}
fn run(trees:Vec<Vec<String>>, env:&mut eval::env::Env, options:&execution_options::ExecutionOptions)
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


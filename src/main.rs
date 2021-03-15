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


        read more about:https://github.com/MAGANER/another-lisp
*/

use std::env;
use std::fs;
use std::process;
use std::io;

mod interpreter;
use interpreter::parser;
use interpreter::eval;
use interpreter::expression;
use interpreter::execution_options as exe_opt;
use interpreter::env as lisp_env;

fn main() 
{
    //process arguments, if they are passed
    let mut options = exe_opt::init_exec_options();
    let args:Vec<String> = env::args().collect();
    if args.len() ==  1 {
        println!("not enough arguments!");
        process::exit(0);
    }
    if args.contains(&"print".to_string())
    {
        options.print_every_operation = true;
    }
    if args.contains(&"repl".to_string())
    {
        options.repl = true;
    }

    //if user chooses repl mode, then don't read file
    if !options.repl 
    {
        check_script_extension(&args[1]);
    }

    //init env, read, parse and execute the script
    let mut env = lisp_env::default_env();

    //run script if it's not read-eval-print-loop
    if !options.repl
    {
        let script = fs::read_to_string(&args[1]).expect("can not open file!");
        let tokens = parser::tokenize(script);
        let trees  = parser::split_trees(tokens);

        run(trees,&mut env,&options);
    } else {
                repl(&mut env,&options);
           }
}
fn run(trees:Vec<Vec<String>>, env:&mut lisp_env::Env, options:&exe_opt::ExecutionOptions)
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
fn check_script_extension(path:&String)
{
    //break execution if extension is incorrect
    //correct extension is .lisp

    let dot_pos = path.find('.');

    match dot_pos
    {
        Some(val) =>
        {
            let start = val;
            let file_type = &path[start+1..];
            if file_type != "lisp"
            {
                println!("incorrect file type:{}",path); 
                process::exit(-1);
            }
        },
        None =>             
        {
            println!("incorrect file type:{}",path); 
            process::exit(-1);
        }
    };
}


use std::io::Write;
fn repl(env:&mut lisp_env::Env,options:&exe_opt::ExecutionOptions)
{
    //read string and execute it
    let reader = io::stdin();
    loop
    {
        let mut input = String::new();

        print!(">");
        io::stdout().flush();

        let _input = reader.read_line(&mut input);
        match _input
        {
            Ok(_) =>
            {
                let tokens = parser::tokenize(input);
                let trees  = parser::split_trees(tokens);
                run(trees,env,&options);
            },
            Err(val) =>
            {
                println!("{}",val);
                process::exit(-1);
            }
        };
    }
}
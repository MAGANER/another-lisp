use std::env;
use std::fs;
use std::process;

mod interpreter;
use interpreter::parser;


fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() ==  1 {
        println!("not enough arguments!");
        process::exit(0);
    }
    let script = fs::read_to_string(&args[1]).expect("can not open file!");
    let tokens = parser::tokenize(script);
    let dtree  = parser::parse(&tokens);
    for node in dtree.nodes.iter()
    {
        println!("{}{}",mult_str(' ', node.depth),node.token);
    }
}


fn mult_str(string:char,rep_number:i32) -> String
{
    let mut new_str:String = String::new();
    for _id in 0..rep_number+1
    {
        new_str.push(string);
    }

    new_str
}

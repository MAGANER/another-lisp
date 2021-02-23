use std::env;
use std::fs;
use std::process;

mod parser;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() ==  1 {
        println!("not enough arguments!");
        process::exit(0);
    }
    let script = fs::read_to_string(&args[1]).expect("can not open file!");
    let tokens = parser::tokenize(script);
    let proto_tree = parser::compute_token_depth(&tokens);
    for node in proto_tree.iter()
    {
        println!("{}{}",mult_str(' ', node.depth),node.token);
    }
}


fn mult_str(string:char,rep_number:i32) -> String
{
    let mut new_str:String = String::new();
    for id in 0..rep_number+1
    {
        new_str.push(string);
    }

    new_str
}

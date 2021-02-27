use std::env;
use std::fs;
use std::process;

mod interpreter;
use interpreter::parser;
use interpreter::reducer;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() ==  1 {
        println!("not enough arguments!");
        process::exit(0);
    }
    let script = fs::read_to_string(&args[1]).expect("can not open file!");
    let tokens = parser::tokenize(script);
    let trees  = parser::parse(&tokens);

    if args.contains(&String::from("tree"))
    {
        print_tree(&trees);
    }
    if args.contains(&String::from("print"))
    {
        println!("result:");
        for tree in trees.iter()
        {
            let result = reducer::compute_whole_tree(tree);
            println!("{}",result);
        }
    }

 
}

fn print_tree(trees:&Vec<parser::DTree>)
{
    for tree in trees.iter()
    {
        for node in tree.nodes.iter()
        {
            println!("{}{}",mult_str(' ', node.depth),node.token);
        }
        println!("------");
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

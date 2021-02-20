use std::env;
use std::fs;
use std::process;

fn main() {
    let args:Vec<String> = env::args().collect();
    if args.len() ==  1 {
        println!("not enough arguments!");
        process::exit(0);
    }
    let script = fs::read_to_string(&args[1]).expect("can not open file!");
    let tokens = tokenize(script);

    println!("{:?}",tokens);
}

fn tokenize(file:String) -> Vec<String> {
    let mut tokens:Vec<String> = Vec::new();

    let mut token:String = String::from("");
    for ch in file.chars() 
    {
        if ch != '\n' && ch != '\r'
        {
            if ch == '(' || ch == ')' 
            { 
               if token.len() > 0
                {
                    tokens.push(token.clone());
                    token.clear();
                }
                tokens.push(String::from(ch)); 
            }
            else if ch != ' ' { token.push(ch); }
            else
            {
                tokens.push(token.clone());
                token.clear();
            }
        }

    };

    tokens
}
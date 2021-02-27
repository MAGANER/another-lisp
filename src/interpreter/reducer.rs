use super::parser;
use super::type_checker;
use std::process;

fn get_node_of_depth(depth:i32,dtree:&parser::DTree) -> parser::DTree
{
    //return sub tree with passed depth
    let mut subtree = parser::DTree{nodes:Vec::new()};
    for node in dtree.nodes.iter()
    {
        if node.depth == depth
        {
            subtree.nodes.push((*node).clone());
        }
    }
    subtree
}
fn get_tree_max_depth(dtree:&parser::DTree) -> i32
{
    let mut all_depth_vals:Vec<i32> = Vec::new();
    for node in dtree.nodes.iter()
    {
        if !all_depth_vals.contains(&node.depth)
        {
            all_depth_vals.push(node.depth);
        }
    }
    
    let max_val = all_depth_vals.iter().max();
    if !max_val.is_none()
    {
        *max_val.unwrap()
    } else {
                -1
           }
}


fn get_arguments(subtree:&parser::DTree) -> Vec<String>
{
    let mut args:Vec<String> = Vec::new();
    for id in 1..subtree.nodes.len()
    {
        args.push(subtree.nodes[id].token.clone());
    }

    args
}
fn check_arguments_types(args:&Vec<String>)
{
    let _type = type_checker::get_value_type(&args[0]);
    for arg in args.iter()
    {
        let curr_type = type_checker::get_value_type(&arg);
        if curr_type != _type
        {
            println!("incorrect arguments:{:?}",args);
            println!("they should have the same types!");
            process::exit(-1);
        }
    }
}

fn compute_sub_tree(subtree:&parser::DTree) -> String
{
    let operation = subtree.nodes[0].token.clone();
    let args      = get_arguments(subtree);
    check_arguments_types(&args);


    let result = match operation.as_str()
    {
        "+" => compute_binary_math_operation(Operator::Plus,   &args),
        "-" => compute_binary_math_operation(Operator::Minus,  &args),
        "*" => compute_binary_math_operation(Operator::Mult,   &args),
        "/" => compute_binary_math_operation(Operator::Divide, &args),
        "&" => compute_binary_math_operation(Operator::And,    &args),
        "|" => compute_binary_math_operation(Operator::Or,     &args),
        _ => {
            println!("no such operation:{}",operation);
            process::exit(-1);
         }
    };

    result
}

enum Operator
{
    Plus,
    Minus,
    Divide,
    Mult,
    And,
    Or
}
fn compute_binary_math_operation(op:Operator,args:&Vec<String>) -> String
{
    let mut result:String = String::new();
    let _type = type_checker::get_value_type(&args[0]);
    for arg in args.iter()
    {
        if _type == type_checker::Type::Int
        {
            let val = arg.parse::<i32>().unwrap();
            if result.is_empty()
            {
                result.push_str(&val.to_string());
            } else {
                        let prev_val = result.parse::<i32>().unwrap();
                        let new_val  = match op 
                                       {
                                            Operator::Plus => prev_val + arg.parse::<i32>().unwrap(),
                                            Operator::Minus=> prev_val - arg.parse::<i32>().unwrap(),
                                            Operator::Mult => prev_val * arg.parse::<i32>().unwrap(),
                                            Operator::Divide=>prev_val / arg.parse::<i32>().unwrap(),
                                            _ => {
                                                    println!("can not apply not arithmetic operator to Int!");
                                                    process::exit(-1);
                                                 }
                                       };
                        result = new_val.to_string();
                   }
        }
        if _type == type_checker::Type::Float
        {
            let val = arg.parse::<f32>().unwrap();
            if result.is_empty()
            {
                result.push_str(&val.to_string());
            } else {
                        let prev_val = result.parse::<f32>().unwrap();
                        let new_val  = match op 
                                       {
                                            Operator::Plus => prev_val + arg.parse::<f32>().unwrap(),
                                            Operator::Minus=> prev_val - arg.parse::<f32>().unwrap(),
                                            Operator::Mult => prev_val * arg.parse::<f32>().unwrap(),
                                            Operator::Divide=>prev_val / arg.parse::<f32>().unwrap(),
                                            _ => {
                                                    println!("can not apply not arithmetic operator to Float value!");
                                                    process::exit(-1);
                                                 }
                                       };
                        result = new_val.to_string();
                   }
        }
        if _type == type_checker::Type::String
        {
            if result.is_empty()
            {
                result.push_str(&arg);
            } else {
                let new_val = match op
                                {
                                    Operator::Plus => result + arg,
                                    _ => {
                                            println!("can not apply not + to value of String type!");
                                            process::exit(-1);
                                         }
                                };
                result = new_val;
                    }
        }
        if _type == type_checker::Type::Bool
        {
            if result.is_empty()
            {
                result.push_str(&arg);
            } else {
                let new_val = match op
                                {
                                    Operator::And => {
                                                        if result == "True" && arg == "True" {"True"}
                                                        else {"False"}
                                                     },
                                    Operator::Or  => {
                                                        if result == "True" || arg == "True" {"True"}
                                                        else {"False"}
                                                     },
                                    _ => {
                                            println!("Bool doesn't support such operators!");
                                            process::exit(-1);
                                         }
                                };
                    result = String::from(new_val);
                   }
        }
    }

    result
}

pub fn compute_whole_tree(dtree:&parser::DTree) -> String
{
    let mut begin = get_tree_max_depth(dtree);
    let mut result:String = String::new();
    while begin > 0
    {
        let mut subtree = get_node_of_depth(begin, dtree);
        if !result.clone().is_empty()
        {
            subtree.nodes.push(parser::Node{token:result.clone(),depth:begin});
        }
        result = compute_sub_tree(&subtree);
        begin -= 1;
    }
    result
}


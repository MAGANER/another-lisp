use super::parser;
use super::type_checker;
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
fn get_arguments(subtree:&parser::DTree)    -> Vec<String>
{
    let mut args:Vec<String> = Vec::new();
    for id in 1..subtree.nodes.len()-1
    {
        args.push(subtree.nodes[id].token.clone());
    }

    args
}
fn compute_sub_tree(subtree:&parser::DTree) //-> Option<String>
{
    let operation = subtree.nodes[0].token.clone();
    let args      = get_arguments(subtree);
}

pub fn compute_whole_tree(dtree:&parser::DTree)
{
    let mut begin = get_tree_max_depth(dtree);
    while begin > 0
    {
        let subtree = get_node_of_depth(begin, dtree);
        begin -= 1;
    }
}
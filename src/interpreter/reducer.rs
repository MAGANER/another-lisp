use super::parser;
fn get_node_of_depth(depth:i32,dtree:&parser::DTree) -> parser::DTree
{
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

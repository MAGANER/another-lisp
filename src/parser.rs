pub fn tokenize(file:String) -> Vec<String> {
    //return vector with tokens
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
    
    delete_empty_tokens(tokens)
}
fn delete_empty_tokens(tokens:Vec<String>) -> Vec<String>
{
    let mut cleared:Vec<String> = Vec::new();
    for token in tokens.iter()
    {
        if token != ""
        {
            cleared.push(token.clone());
        }
    }
    cleared
}


/*
    Node is element of tree.
    But this parser implements depth-based tree.
    It's simply array of Node, containing token and
    depth value inside s expression. 
    To compute you should begin from the deepest node.
*/
#[derive(Debug)]
pub struct Node
{
    pub token:String,
    pub depth:i32
}

#[derive(Debug)]
pub struct DTree
{
    pub nodes:Vec<Node>
}

pub fn parse(tokens:&Vec<String>) ->DTree
{
    let mut dtree = DTree{nodes:Vec::new()};
    
    let mut depth = 0;
    for token in tokens.iter()
    {
        if token == "(" { depth += 1; }
        if token == ")" { depth -= 1; }

        let not_bracket = token != "(" && token != ")";

        if not_bracket
        {
            let curr_node = Node{token:token.clone(),depth:depth};
            dtree.nodes.push(curr_node);
        }
    }

    dtree
}
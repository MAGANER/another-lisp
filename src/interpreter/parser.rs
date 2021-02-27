/*
    Node is element of tree.
    But this parser implements depth-based tree.
    It's simply array of Node, containing token and
    depth value inside s expression. 
    To compute you should begin from the deepest node.
    I created a special structure to let everyone see,
    DTtree is more than just a vector.
*/
#[derive(Debug,Clone)]
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

pub fn parse(tokens:&Vec<String>) -> Vec<DTree>
{
    let mut trees:Vec<DTree> = Vec::new();
    let mut dtree = DTree{nodes:Vec::new()};
    
    let mut begin_counter = 0;
    let mut end_counter   = 0;

    let mut depth = 0;
    for token in tokens.iter()
    {
        if token == "(" { depth += 1; begin_counter+=1; }
        if token == ")" { depth -= 1; end_counter  +=1; }

        let not_bracket = token != "(" && token != ")";

        if not_bracket
        {
            let curr_node = Node{token:token.clone(),depth:depth};
            dtree.nodes.push(curr_node);
        }

        if begin_counter == end_counter && begin_counter != 0
        {
            begin_counter = 0;
            end_counter   = 0;
            trees.push(dtree);
            dtree = DTree{nodes:Vec::new()};
        }
    }

    trees
}

pub fn tokenize(file:String) -> Vec<String> {
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

use super::expression;

pub fn tokenize(expr:String) -> Vec<String>
{   
    //every token is split with white space
    let mut tokens:Vec<String> = Vec::new();

    let mut current_token:String = "".to_string();
    let mut str_counter = 0;
    let mut comment = false;

    for ch in expr.replace("(","( ").replace(")"," ) ").chars()
    {
      if ch == '\n' && comment
      {
        comment = false;
      }
      else if ch == '#'
      {
        comment = true;
      }

      if !comment
      {
          if ch == '"' { str_counter+= 1;}

          if ch == '(' || ch == ')'
          {
            tokens.push(ch.to_string());
          }
          else if !ch.is_whitespace() || str_counter != 0
          {
            current_token.push(ch);
          }
          else
          {
            if !current_token.chars().all(|x| x.is_whitespace()) 
            {
              tokens.push(current_token.clone());
            }
            current_token.clear();
          }

          if str_counter == 2
          {
            str_counter = 0;
          }
      
      }
    }
    
    count_brackets(&tokens);
    tokens
}
fn count_brackets(tokens:&Vec<String>)
{
  let open_number = tokens.iter().filter(|&x| x.clone() == "(".to_string()).collect::<Vec<&String>>().len();
  let close_number= tokens.iter().filter(|&x| x.clone() == ")".to_string()).collect::<Vec<&String>>().len();
  if open_number != close_number
  {
    println!("no equal number of ( and )!");
    std::process::exit(-1);
  }
}

pub fn split_trees(tokens:Vec<String>) -> Vec<Vec<String>>
{
  let mut trees:Vec<Vec<String>> = Vec::new();

  let mut list:Vec<String> = Vec::new();
  let mut begin_counter = 0;
  let mut end_counter   = 0;

  for token in tokens.iter()
  {
      if token.clone() == "(".to_string() { begin_counter+= 1; }
      if token.clone() == ")".to_string() { end_counter  += 1; }

      list.push(token.clone());

      if begin_counter == end_counter && begin_counter != 0
      {
        trees.push(list.to_vec());
        list.clear();
        begin_counter = 0;
        end_counter   = 0;
      }
  }

  trees
}


pub fn parse<'a>(tokens: &'a [String]) -> Result<(expression::Expr, &'a [String]), expression::Err> {
    //get the head and tale 
    let (token, rest) = tokens.split_first()
    .ok_or(
        expression::Err::Reason("could not get token".to_string())
    )?;
    
  match &token[..] {
    "(" => read_seq(rest),
    ")" => Err(expression::Err::Reason("unexpected `)`".to_string())),
    _   => Ok((parse_atom(token), rest)),
  }
}

fn read_seq<'a>(tokens: &'a [String]) -> Result<(expression::Expr, &'a [String]), expression::Err> {
    let mut res: Vec<expression::Expr> = vec![];
    let mut xs = tokens;
    loop {
      let (next_token, rest) = xs
        .split_first()
        .ok_or(expression::Err::Reason("could not find closing `)`".to_string()))
        ?;
      if next_token == ")" {
        return Ok((expression::Expr::List(res), rest)) // skip `)`, head to the token after
      }
      let (exp, new_xs) = parse(&xs)?;
      res.push(exp);
      xs = new_xs;
    }
  }
fn parse_atom(token: &str) -> expression::Expr {
    match token.as_ref()
    {
      "True" => expression::Expr::Bool(true),
      "False"=> expression::Expr::Bool(false),
      _ => 
      {
        //try to parse atom token as number, unless it's symbol      
        let potential_float: Result<f64, std::num::ParseFloatError> = token.parse();

        match potential_float {
            Ok(v) => expression::Expr::Number(v),
            Err(_) => expression::Expr::Symbol(token.to_string().clone())
                              }
      }
    }
  }
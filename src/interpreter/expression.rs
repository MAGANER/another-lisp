//Types of lisp expressions
#[derive(Clone)]
pub enum Expr
{
    Bool(bool),
    Symbol(String),
    Number(f64),
    List(Vec<Expr>),
    Func(fn(&[Expr]) -> Result<Expr, Err>),
}

//inner error type
#[derive(Debug)]
pub enum Err
{
    Reason(String)
}

pub fn is_string_value(val:&String) -> bool
{
    val.chars().nth(0).unwrap() == '"' && val.chars().nth(val.len()-1).unwrap() == '"'
}


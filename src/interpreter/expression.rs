//Types of lisp expressions
#[derive(Clone)]
pub enum Expr
{
    Bool(bool),
    Symbol(String),
    Number(f64),
    List(Vec<Expr>),
    Func(fn(&[Expr]) -> Result<Expr, Err>),
    Lambda(LambdaStruct),
}

//inner error type
#[derive(Debug)]
pub enum Err
{
    Reason(String)
}

#[derive(Clone)]
pub struct LambdaStruct
{
    pub arg: Box<Expr>, // Box to avoid recursive type definition
    pub body : Box<Vec<Expr>>
}


pub fn is_string_value(val:&String) -> bool
{
    let end = val.ends_with("'");
    let start = val.chars().rev().collect::<String>().ends_with("'");
    end && start
}


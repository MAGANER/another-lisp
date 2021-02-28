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


#[derive(PartialEq)]
pub enum Type
{
    Int,
    Float,
    Bool,
    String
}


pub fn get_value_type(value:&String) -> Type
{
    let mut numeric_counter = 0;
    let mut has_point = false;
    let mut is_negative = false;
    for ch in value.chars()
    {
        if ch == '-'       { is_negative = true; }
        if ch.is_numeric() { numeric_counter+=1; }
        if ch == '.'       { has_point = true  ; }   
    }

    let is_int   = numeric_counter == value.len() ;
    let is_float = has_point && numeric_counter == value.len() - 1;
    let is_bool  = value == "True" || value == "False";

    if      is_int   || is_negative ^ (!is_int   || is_negative)   { Type::Int   }
    else if is_float || is_negative ^ (!is_float || is_negative) { Type::Float }
    else if is_bool  { Type::Bool  }
    else { Type::String }
}
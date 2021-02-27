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
    for ch in value.chars()
    {
        if ch == '-'       { numeric_counter+=1; }
        if ch.is_numeric() { numeric_counter+=1; }
        if ch == '.'       { has_point = true  ; }   
    }
    if numeric_counter == value.len() { Type::Int }
    else if has_point && numeric_counter == value.len() - 1 { Type::Float }
    else if value == "True" || value == "False" { Type::Bool }
    else { Type::String }
}
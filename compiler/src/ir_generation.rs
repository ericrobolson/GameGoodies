use crate::{environment::Error, parser::ParseNode};

pub enum Ast {
    Empty,
    Struct,
    Function,
}

pub enum Types {
    Number,
    String,
}

pub fn generate(parse_nodes: &Vec<ParseNode>) -> Result<Ast, Error> {
    Ok(Ast::Empty)
}

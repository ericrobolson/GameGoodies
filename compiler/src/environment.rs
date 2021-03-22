use crate::{ir_generation, lexer, parser, syntax_analysis, File};

#[derive(Debug)]
pub struct Error {
    line: usize,
    line_pos: usize,
    message: String,
}

impl Error {
    pub fn new(line: usize, line_pos: usize, message: String) -> Self {
        Self {
            line,
            line_pos,
            message,
        }
    }

    pub(crate) fn unclosed_list(token: &lexer::Token) -> Self {
        Error::token(token, "Unclosed list!".into())
    }

    pub(crate) fn token(token: &lexer::Token, msg: String) -> Self {
        Self::new(token.line, token.line_pos, msg)
    }
}

pub struct Environment {}
impl Environment {
    pub fn new() -> Self {
        Self {}
    }

    pub fn compile(&mut self, files: Vec<File>) {
        let mut contents: Vec<(&File, Vec<parser::ParseNode>)> = files
            .iter()
            .map(|f| {
                println!("{:#?}", f);
                let tokens = match lexer::lex(&f.contents) {
                    Ok(tokens) => tokens,
                    Err(e) => {
                        println!("An error: {:?}", e);
                        todo!();
                    }
                };

                println!("Analyzing...");
                match syntax_analysis::analyze(&tokens) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("An error: {:?}", e);
                        todo!();
                    }
                }

                println!("Parsing...");
                let parse_nodes = match parser::parse(&tokens) {
                    Ok(nodes) => nodes,
                    Err(e) => {
                        println!("An error: {:?}", e);
                        todo!();
                    }
                };

                println!("{:#?}", parse_nodes);

                // NOTE: this should be moved out

                println!("Generating...");
                let ir = match ir_generation::generate(&parse_nodes) {
                    Ok(_) => {}
                    Err(e) => {
                        println!("An error: {:?}", e);
                        todo!();
                    }
                };

                (f, parse_nodes)
            })
            .collect();
    }
}

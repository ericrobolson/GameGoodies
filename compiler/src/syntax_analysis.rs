use crate::{
    environment::Error,
    lexer::{Token, Tokens},
};

pub fn analyze(tokens: &Vec<Token>) -> Result<(), Error> {
    println!("Syntax analysis");
    let mut lparens = 0;
    let mut rparens = 0;

    let mut last_paren = None;

    for token in tokens {
        match &token.token {
            Tokens::LParen => {
                lparens += 1;
                last_paren = Some(token.clone());
            }
            Tokens::RParen => {
                rparens += 1;
                last_paren = Some(token.clone());
            }
            Tokens::Number(_) => {}
            Tokens::String(_) => {}
            Tokens::Identifier(id) => match validate_identifier(id, token) {
                Some(e) => {
                    return Err(e);
                }
                None => {}
            },
            Tokens::Comment(_) => {}
            Tokens::TypeSeparator => {}
        }
    }

    if lparens != rparens {
        let msg = {
            if lparens > rparens {
                return Err(Error::unclosed_list(&last_paren.unwrap()));
            } else {
                "Too many right parenthesis!"
            }
        };

        return Err(Error::token(&last_paren.unwrap(), msg.into()));
    }

    Ok(())
}

fn validate_identifier(id: &String, token: &Token) -> Option<Error> {
    if id.len() == 0 {
        return Some(Error::token(token, "Invalid length string!".into()));
    }

    let first_char = id.chars().next().unwrap();
    match first_char {
        '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
            return Some(Error::token(
                token,
                format!(
                    "Invalid starting character '{}' for identifier '{}'",
                    first_char, id
                ),
            ));
        }
        _ => {}
    }
    None
}

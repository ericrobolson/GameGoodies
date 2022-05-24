use crate::{
    environment::Error,
    lexer::{Token, Tokens},
};

#[derive(Debug)]
pub struct ParseNode {
    pub token: Token,
    pub node: Node,
}

impl ParseNode {
    fn new(token: &Token, node: Node) -> Self {
        Self {
            token: token.clone(),
            node,
        }
    }
}

#[derive(Debug)]
pub enum Node {
    List(Vec<ParseNode>),
    Number(i32),
    Identifier(String),
    String(String),
    Comment(String),
    TypeSeparator,
}

pub fn parse(tokens: &Vec<Token>) -> Result<Vec<ParseNode>, Error> {
    let nodes = make_tree(tokens)?;

    Ok(nodes)
}

fn make_tree(tokens: &Vec<Token>) -> Result<Vec<ParseNode>, Error> {
    let mut nodes = vec![];
    let mut token_stack: Vec<(Token, Vec<ParseNode>)> = vec![];

    for token in tokens {
        let node = match &token.token {
            // Start building a new list
            Tokens::LParen => {
                token_stack.push((token.clone(), vec![]));
                None
            }
            // Add the list to the nodes
            Tokens::RParen => {
                match token_stack.pop() {
                    Some((token, node_list)) => {
                        let new_list = ParseNode::new(&token, Node::List(node_list));

                        match token_stack.pop() {
                            // If there's a previous list, add it to the that one
                            Some((t, mut stack)) => {
                                stack.push(new_list);
                                token_stack.push((t, stack));
                            }
                            // Otherwise put it in the nodes
                            None => {
                                nodes.push(new_list);
                            }
                        }
                    }
                    None => {
                        return Err(Error::unclosed_list(token));
                    }
                }

                None
            }
            Tokens::Number(n) => Some(ParseNode::new(token, Node::Number(*n))),
            Tokens::String(s) => Some(ParseNode::new(token, Node::String(s.clone()))),
            Tokens::Identifier(id) => Some(ParseNode::new(token, Node::Identifier(id.clone()))),
            Tokens::Comment(s) => Some(ParseNode::new(token, Node::Comment(s.clone()))),
            Tokens::TypeSeparator => Some(ParseNode::new(token, Node::TypeSeparator)),
        };

        match node {
            Some(node) => {
                // Push it to the lists if possible
                match token_stack.pop() {
                    Some((t, mut stack)) => {
                        stack.push(node);
                        token_stack.push((t, stack));
                    }
                    None => {
                        nodes.push(node);
                    }
                }
            }
            None => {}
        }
    }

    if token_stack.is_empty() == false {
        todo!("Token list stack not empty!");
    }

    Ok(nodes)
}

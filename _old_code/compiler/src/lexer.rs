pub type Int = i32;

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
    LParen,
    RParen,
    TypeSeparator,
    Number(Int),
    String(String),
    Identifier(String),
    Comment(String),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub token: Tokens,
    pub line: usize,
    pub line_pos: usize,
}

impl Token {
    fn make(line: usize, line_pos: usize, token: Tokens) -> Self {
        Self {
            line,
            line_pos,
            token,
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum LexErr {
    UnclosedString,
}

/// Given a program, lex it into some tokens.
pub fn lex(program: &String) -> Result<Vec<Token>, LexErr> {
    let mut state = LexState {
        current_line: 0,
        current_line_pos: 0,

        token_line: 0,
        token_line_pos: 0,

        tokens: vec![],
        states: vec![],
    };

    for c in program.chars() {
        lex_char(c, &mut state)?;
    }

    if let Some(s) = state.states.pop() {
        match s {
            State::BuildString(s) => {
                return Err(LexErr::UnclosedString);
            }
            State::BuildIdentifier(s) => {
                state.add_token(Tokens::Identifier(s));
            }
            State::BuildComment(comment) => {
                // This is OK, we can just assume it's an ended comment.
                state.add_token(Tokens::Comment(comment));
            }
        }
    }

    if let Some(state) = state.states.pop() {
        todo!("Uh, you have more than 1 state that was unhandled. Need to come up with an error.");
    }

    Ok(state.tokens)
}

const COMMENT: &'static str = ";;";

struct LexState {
    current_line: usize,
    current_line_pos: usize,

    token_line: usize,
    token_line_pos: usize,
    tokens: Vec<Token>,
    states: Vec<State>,
}

impl LexState {
    fn add_newline(&mut self) -> (usize, usize) {
        let current = (self.token_line, self.token_line_pos);

        self.current_line += 1;
        self.current_line_pos = 0;
        self.reset_token_lines();

        current
    }

    fn reset_token_lines(&mut self) {
        self.token_line = self.current_line;
        self.token_line_pos = self.current_line_pos;
    }

    fn add_token(&mut self, token: Tokens) {
        self.tokens
            .push(Token::make(self.token_line, self.token_line_pos, token));

        self.reset_token_lines();
    }
}

#[derive(Debug, PartialEq)]
enum State {
    BuildString(String),
    BuildIdentifier(String),
    BuildComment(String),
}

fn lex_char(c: char, lex_state: &mut LexState) -> Result<(), LexErr> {
    let is_str_char = c == '"';
    let is_paren = c == '(' || c == ')';
    let is_newline = c == '\n';
    let is_type_separator = c == ':';
    let is_terminal_char = is_paren || is_type_separator || is_str_char || c.is_whitespace();

    // Increment lexing position
    let mut prev_pos = (0, 0);
    if is_newline {
        prev_pos = lex_state.add_newline();
    } else {
        lex_state.current_line_pos += 1;
    }
    let prev_pos = prev_pos;

    // Attempt to parse + build out the proper state
    if let Some(state) = lex_state.states.pop() {
        let mut put_back = None;
        match state {
            State::BuildString(mut s) => {
                // TODO: nested strings?
                if is_str_char {
                    lex_state.add_token(Tokens::String(s));
                } else {
                    s.push(c);
                    lex_state.states.push(State::BuildString(s));
                }
            }
            State::BuildIdentifier(mut i) => {
                // Building an identifier. If it's terminal, attempt to make a number. If unable, make an identifier.
                if is_terminal_char {
                    match i.parse::<Int>() {
                        Ok(i) => {
                            lex_state.add_token(Tokens::Number(i));
                        }
                        Err(e) => {
                            lex_state.add_token(Tokens::Identifier(i));
                        }
                    }

                    // Guard against some weird line offsetting
                    if (is_paren || is_type_separator) && lex_state.token_line_pos > 0 {
                        lex_state.token_line_pos -= 1;
                    }
                } else {
                    i.push(c);

                    // If it's a comment, make a comment otherwise continue making identifier
                    if i == COMMENT {
                        put_back = Some(State::BuildComment("".into()));
                    } else {
                        put_back = Some(State::BuildIdentifier(i));
                    }
                }
            }
            State::BuildComment(mut comment) => {
                //
                if is_newline {
                    lex_state.tokens.push(Token::make(
                        prev_pos.0,
                        prev_pos.1,
                        Tokens::Comment(comment.trim().into()),
                    ));
                    lex_state.reset_token_lines();
                } else {
                    comment.push(c);
                    put_back = Some(State::BuildComment(comment))
                }
            }
        }

        // Add the in-progress token back to the states
        if let Some(state) = put_back {
            lex_state.states.push(state);
        }
    }
    // Start building a string
    else if is_str_char {
        lex_state.states.push(State::BuildString("".into()));
    }
    // Start building an identifier
    else if !c.is_whitespace() && !is_paren {
        lex_state
            .states
            .push(State::BuildIdentifier(format!("{}", c)));
    } else if c.is_whitespace() {
        lex_state.reset_token_lines();
    }

    // Single token things that should always be added
    match c {
        '(' => {
            lex_state.add_token(Tokens::LParen);
        }
        ')' => {
            lex_state.add_token(Tokens::RParen);
        }
        ':' => {
            lex_state.add_token(Tokens::TypeSeparator);
        }
        _ => {}
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lex_lparen() {
        let input = "(";
        let expected = vec![Token::make(0, 0, Tokens::LParen)];
        let actual = lex(&input.into()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_rparen() {
        let input = ")";
        let expected = vec![Token::make(0, 0, Tokens::RParen)];
        let actual = lex(&input.into()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_string() {
        let input = "\"a string!\"";
        let expected = vec![Token::make(0, 0, Tokens::String("a string!".into()))];
        let actual = lex(&input.into()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_add() {
        let input = "(+ 1 2 3)";
        let expected = vec![
            Token::make(0, 0, Tokens::LParen),
            Token::make(0, 1, Tokens::Identifier("+".into())),
            Token::make(0, 3, Tokens::Number(1)),
            Token::make(0, 5, Tokens::Number(2)),
            Token::make(0, 7, Tokens::Number(3)),
            Token::make(0, 8, Tokens::RParen),
        ];
        let actual = lex(&input.into()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_separator() {
        let input = "(a:num)";
        let expected = vec![
            Token::make(0, 0, Tokens::LParen),
            Token::make(0, 1, Tokens::Identifier("a".into())),
            Token::make(0, 2, Tokens::TypeSeparator),
            Token::make(0, 3, Tokens::Identifier("num".into())),
            Token::make(0, 6, Tokens::RParen),
        ];
        let actual = lex(&input.into()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_comment() {
        let input = ";; Test comment bruh\r\n";
        let expected = vec![Token::make(
            0,
            0,
            Tokens::Comment("Test comment bruh".into()),
        )];
        let actual = lex(&input.into()).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn lex_complex_add() {
        let input = "(* 1 (+ 2 3))";
        let expected = vec![
            Token::make(0, 0, Tokens::LParen),
            Token::make(0, 1, Tokens::Identifier("*".into())),
            Token::make(0, 3, Tokens::Number(1)),
            Token::make(0, 5, Tokens::LParen),
            Token::make(0, 6, Tokens::Identifier("+".into())),
            Token::make(0, 8, Tokens::Number(2)),
            Token::make(0, 10, Tokens::Number(3)),
            Token::make(0, 11, Tokens::RParen),
            Token::make(0, 12, Tokens::RParen),
        ];
        let actual = lex(&input.into()).unwrap();

        assert_eq!(expected, actual);
    }
    #[test]
    fn lex_valid_program() {
        let input = "
            (define (test prog)
                (display prog))
        ";

        let expected = vec![
            Token::make(1, 12, Tokens::LParen),
            Token::make(1, 13, Tokens::Identifier("define".into())),
            Token::make(1, 20, Tokens::LParen),
            Token::make(1, 21, Tokens::Identifier("test".into())),
            Token::make(1, 26, Tokens::Identifier("prog".into())),
            Token::make(1, 30, Tokens::RParen),
            Token::make(2, 16, Tokens::LParen),
            Token::make(2, 17, Tokens::Identifier("display".into())),
            Token::make(2, 25, Tokens::Identifier("prog".into())),
            Token::make(2, 29, Tokens::RParen),
            Token::make(2, 30, Tokens::RParen),
        ];
        let actual = lex(&input.into()).unwrap();

        assert_eq!(expected, actual);
    }
}

use crate::token::{Token, TokenType};

pub struct Lexer<'b> {
    pub input: &'b str,
}

impl Lexer<'_> {
    pub fn lex(&self) -> Vec<Token> {
        let mut tokens: Vec<Token> = Vec::new();
        let mut chars_iter = self.input.chars().enumerate().peekable();
        while let Some((pos, lookahead)) = chars_iter.peek().cloned() {
            let token_start_pos = pos;
            if lookahead.is_ascii_whitespace() {
                chars_iter.next();
            } else if lookahead == '+' {
                let text = lookahead.to_string();
                let token = Token{ tpe: TokenType::Plus, text, start_pos: token_start_pos};
                tokens.push(token);
                chars_iter.next();
            } else if lookahead == ';' {
                let text = lookahead.to_string();
                let token = Token{ tpe: TokenType::SemiColon, text, start_pos: token_start_pos};
                tokens.push(token);
                chars_iter.next();
            }else if lookahead == '*' {
                let text = lookahead.to_string();
                let token = Token{ tpe: TokenType::Times, text, start_pos: token_start_pos};
                tokens.push(token);
                chars_iter.next();
            } else if lookahead == '=' {
                let text = lookahead.to_string();
                let token = Token{ tpe: TokenType::Equals, text, start_pos: token_start_pos};
                tokens.push(token);
                chars_iter.next();
            } else if lookahead.is_ascii_digit() {
                let mut text = String::new();
                while let Some((_pos, next_char)) = chars_iter.peek().cloned() {
                    if !next_char.is_ascii_digit() {
                        break;
                    }
                    text.push(next_char);
                    chars_iter.next();
                }
                tokens.push(Token{ tpe: TokenType::Num, text, start_pos: token_start_pos});
            } else if lookahead.is_ascii_alphabetic() {
                let mut text = String::new();
                while let Some((_pos, next_char)) = chars_iter.peek().cloned() {
                    if !next_char.is_ascii_alphabetic() {
                        break;
                    }
                    text.push(next_char);
                    chars_iter.next();
                }
                let tpe = match text.as_str() {
                    "true" => TokenType::True,
                    "false" => TokenType::False,
                    _ => TokenType::Identifier,
                };
                tokens.push(Token{ tpe, text, start_pos: token_start_pos });
            } else {
                panic!("Unknown character {} at position {}", lookahead, pos);
            }
        }
        tokens.push(Token{tpe: TokenType::EOF, text: "<EOF>".to_string(), start_pos: self.input.chars().count()});
        return tokens;
    }
}


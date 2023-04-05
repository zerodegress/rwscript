use crate::lexer::range;

use super::{ Lexer, Lex, Position, LexError };
use lazy_regex::regex_captures;

pub struct NumberLexer;

impl Lexer for NumberLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> Result<(&'a str, Lex), LexError> {
        match regex_captures!(
            r#"^((?:(?:[1-9][0-9]*)|[0-9])(?:\.[0-9]+)?)(.*)$"#,
            text,
        ) {
            Some((_, num, rest)) => {
                Ok((
                    rest,
                    Lex::new("number", num, range(num, start))
                ))
            }
            None => {
                Err(LexError::new(text, start, "number", "unknown"))
            }
        }
    }
}

pub struct StringLexer;

impl Lexer for StringLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> anyhow::Result<(&'a str, Lex), LexError> {
        match regex_captures!(
            r#"^("(?:.*[^\\])?")(.*)$"#,
            text,
        ) {
            Some((_, string, rest)) => {
                Ok((
                    rest,
                    Lex::new("string", string, range(string, start))
                ))
            }
            None => {
                match regex_captures!(
                    r#"^('(?:.*[^\\])?')(.*)$"#,
                    text,
                ) {
                    Some((_, string, rest)) => {
                        Ok((
                            rest,
                            Lex::new("string", string, range(string, start))
                        ))
                    }
                    None => {
                        Err(LexError::new(text, start, "string", "unknown"))
                    }
                }
            }
        }
    }
}

pub struct BooleanLexer;

impl Lexer for BooleanLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> anyhow::Result<(&'a str, Lex), LexError> {
        match regex_captures!(
            r#"^((?:true)|(?:false))(.*)$"#,
            text
        ) {
            Some((_, boolean, rest)) => {
                Ok((
                    rest,
                    Lex::new("boolean", boolean, range(boolean, start)),
                ))
            }
            None => {
                Err(LexError::new(text, start, "boolean", "unknown"))
            }
        }
    }
}
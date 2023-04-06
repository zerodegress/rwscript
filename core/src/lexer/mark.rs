use crate::lexer::range;

use super::{ Lexer, Lex, Position, LexError };
use lazy_regex::regex_captures;

pub struct ParrenLeftLexer;

impl Lexer for ParrenLeftLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> Result<(&'a str, Lex), LexError> {
        match regex_captures!(
            r#"^(\()(.*)$"#,
            text,
        ) {
            Some((_, parren_left, rest)) => {
                Ok((
                    rest,
                    Lex::new("parren_left", parren_left, range(parren_left, start))
                ))
            }
            None => {
                Err(LexError::new(text, start, "parren_left", "unknown"))
            }
        }
    }
}

pub struct ParrenRightLexer;

impl Lexer for ParrenRightLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> Result<(&'a str, Lex), LexError> {
        match regex_captures!(
            r#"^(\))(.*)$"#,
            text,
        ) {
            Some((_, parren_right, rest)) => {
                Ok((
                    rest,
                    Lex::new("parren_right", parren_right, range(parren_right, start))
                ))
            }
            None => {
                Err(LexError::new(text, start, "parren_right", "unknown"))
            }
        }
    }
}

pub struct BraceLeftLexer;

impl Lexer for BraceLeftLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> Result<(&'a str, Lex), LexError> {
        match regex_captures!(
            r#"^(\{)(.*)$"#,
            text,
        ) {
            Some((_, brace_left, rest)) => {
                Ok((
                    rest,
                    Lex::new("brace_left", brace_left, range(brace_left, start))
                ))
            }
            None => {
                Err(LexError::new(text, start, "brace_left", "unknown"))
            }
        }
    }
}

pub struct BraceRightLexer;

impl Lexer for BraceRightLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> Result<(&'a str, Lex), LexError> {
        match regex_captures!(
            r#"^(\})(.*)$"#,
            text,
        ) {
            Some((_, brace_right, rest)) => {
                Ok((
                    rest,
                    Lex::new("brace_right", brace_right, range(brace_right, start))
                ))
            }
            None => {
                Err(LexError::new(text, start, "brace_right", "unknown"))
            }
        }
    }
}

pub struct BracketLeftLexer;

impl Lexer for BracketLeftLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> Result<(&'a str, Lex), LexError> {
        match regex_captures!(
            r#"^(\[)(.*)$"#,
            text,
        ) {
            Some((_, bracket_left, rest)) => {
                Ok((
                    rest,
                    Lex::new("bracket_left", bracket_left, range(bracket_left, start))
                ))
            }
            None => {
                Err(LexError::new(text, start, "bracket_left", "unknown"))
            }
        }
    }
}

pub struct BracketRightLexer;

impl Lexer for BracketRightLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> Result<(&'a str, Lex), LexError> {
        match regex_captures!(
            r#"^(\])(.*)$"#,
            text,
        ) {
            Some((_, bracket_right, rest)) => {
                Ok((
                    rest,
                    Lex::new("bracket_right", bracket_right, range(bracket_right, start))
                ))
            }
            None => {
                Err(LexError::new(text, start, "bracket_right", "unknown"))
            }
        }
    }
}
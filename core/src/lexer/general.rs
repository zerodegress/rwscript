use crate::lexer::range;

use super::{Lexer, Position, Lex, LexError};
use regex::{RegexBuilder, Regex};
use anyhow::Result;

pub struct RegexLexer {
    regex: Regex,
    r#type: String,
}

impl RegexLexer {
    pub fn new(regex: &str, r#type: &str) -> Result<Self> {
        let exp = RegexBuilder::new(format!("^({})(.*)$", regex).as_str())
            .dot_matches_new_line(true)
            .build();
        match exp {
            Ok(exp) => {
                Ok(Self {
                    regex: exp,
                    r#type: r#type.to_string(),
                })
            }
            Err(err) => {
                Err(err.into())
            }
        }
    }
}

impl Lexer for RegexLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> Result<(&'a str, Lex), LexError> {
        match self.regex.captures(text) {
            Some(caps) => {
                let (content, rest) = (caps.get(1).unwrap(), caps.get(caps.len() - 1).unwrap());
                Ok((
                    rest.as_str(),
                    Lex::new(&self.r#type, content.as_str(), range(content.as_str(), start)),
                ))
            }
            None => {
                Err(LexError::new(text, start, &self.r#type, "unknown"))
            }
        }
    }
}
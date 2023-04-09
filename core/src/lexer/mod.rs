use std::fmt::Display;
use crate::{NEWLINE, lexer::{literal::literal_lexers, keyword::keyword_lexers, mark::mark_lexers, general::RegexLexer, special::special_lexers}};

// TODO:实现分词器
use anyhow::Result;

pub mod literal;
pub mod keyword;
pub mod mark;
pub mod general;
pub mod special;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    pub line: usize,
    pub column: usize,
}

impl Position {
    pub fn new(line: usize, column: usize) -> Self {
        Self {
            line,
            column,
        }
    }
}

pub type Range = (Position, Position);

pub fn range(text: &str, start: Position) -> Range {
    let lines = text.split(NEWLINE);
    let line_num = lines.clone().count() - 1;
    let pos = match lines.last() {
        Some(last) => {
            start.column + if last.len() > 0  { last.len() } else { 1 }
        }
        None => {
            start.column + 1
        }
    };
    (start, Position::new(start.line + line_num, pos))
}

#[derive(Debug, PartialEq, Eq)]
pub struct Lex {
    r#type: String,
    content: String,
    range: Range,
}

impl Lex {
    pub fn new(r#type: &str, content: &str, range: Range) -> Self {
        Self {
            r#type: r#type.to_string(),
            content: content.to_string(),
            range,
        }
    }

    pub fn r#type(&self) -> &str {
        &self.r#type
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn range(&self) -> &Range {
        &self.range
    }
}

pub trait Lexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> Result<(&'a str, Lex), LexError>;
}

#[derive(Debug, PartialEq, Eq)]
pub struct LexError {
    content: String,
    position: Position,
    expected: String,
    found: String,
}

impl LexError {
    pub fn new(
        content: &str, 
        position: Position, 
        expected: &str,
        found: &str,
    ) -> Self {
        Self {
            content: content.to_string(),
            position,
            expected: expected.to_string(),
            found: found.to_string(),
        }
    }
}

impl Display for LexError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f, 
            "{}:{}\n> {}\nexpected\"{}\",found\"{}\"", 
            self.position.line,
            self.position.column, 
            self.content, 
            self.expected, 
            self.found
        )
    }
}

impl std::error::Error for LexError {
}

pub fn lex(text: &str) -> Result<Vec<Lex>, LexError> {
    let mut lexers = Vec::new();
    lexers.append(&mut special_lexers());
    lexers.append(&mut literal_lexers());
    lexers.append(&mut keyword_lexers());
    lexers.append(&mut mark_lexers());
    lexers.push(Box::new(RegexLexer::new(r"\S+", "identifier").unwrap()));

    let mut text = text.to_string();
    let mut start = Position::new(1, 1);
    let mut error = None;
    let mut lexs = Vec::new();
    loop {
        let mut flag = false;
        for lexer in &mut lexers {
            match lexer.lex(&text, start) {
                Ok((rest, lex)) => {
                    text = rest.to_string();
                    let (_, next_start) = lex.range;
                    start = next_start;
                    flag = true;
                    lexs.push(lex);
                    break;
                }
                Err(err) => {
                    error = Some(err);
                }
            }
        }
        if !flag {
            return Err(error.unwrap())
        } else if text.is_empty() {
            return Ok(lexs);
        }
    }
}
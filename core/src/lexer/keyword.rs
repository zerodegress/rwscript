use crate::lexer::range;

use super::{ Lexer, Lex, Position, LexError };
use lazy_regex::regex_captures;

pub struct FnLexer;

impl Lexer for FnLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> anyhow::Result<(&'a str, Lex), LexError> {
        match regex_captures!(
            r#"^(fn)(.*)$"#,
            text,
        ) {
            Some((_, fn_, rest)) => {
                Ok((
                    rest,
                    Lex::new("fn", fn_, range(fn_, start))
                ))
            }
            None => {
                Err(LexError::new(text, start, "fn", "unknown"))
            }
        }
    }
}

pub struct StructLexer;

impl Lexer for StructLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> anyhow::Result<(&'a str, Lex), LexError> {
        match regex_captures!(
            r#"^(struct)(.*)$"#,
            text,
        ) {
            Some((_, struct_, rest)) => {
                Ok((
                    rest,
                    Lex::new("struct", struct_, range(struct_, start))
                ))
            }
            None => {
                Err(LexError::new(text, start, "struct", "unknown"))
            }
        }
    }
}

pub struct TraitLexer;

impl Lexer for TraitLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> anyhow::Result<(&'a str, Lex), LexError> {
        match regex_captures!(
            r#"^(trait)(.*)$"#,
            text,
        ) {
            Some((_, trait_, rest)) => {
                Ok((
                    rest,
                    Lex::new("trait", trait_, range(trait_, start))
                ))
            }
            None => {
                Err(LexError::new(text, start, "trait", "unknown"))
            }
        }
    }
}

pub struct MutLexer;

impl Lexer for MutLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> anyhow::Result<(&'a str, Lex), LexError> {
        match regex_captures!(
            r#"^(mut)(.*)$"#,
            text,
        ) {
            Some((_, mut_, rest)) => {
                Ok((
                    rest,
                    Lex::new("mut", mut_, range(mut_, start))
                ))
            }
            None => {
                Err(LexError::new(text, start, "mut", "unknown"))
            }
        }
    }
}

pub struct ConstLexer;

impl Lexer for ConstLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> anyhow::Result<(&'a str, Lex), LexError> {
        match regex_captures!(
            r#"^(const)(.*)$"#,
            text,
        ) {
            Some((_, const_, rest)) => {
                Ok((
                    rest,
                    Lex::new("const", const_, range(const_, start))
                ))
            }
            None => {
                Err(LexError::new(text, start, "const", "unknown"))
            }
        }
    }
}

pub struct ImportLexer;

impl Lexer for ImportLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> anyhow::Result<(&'a str, Lex), LexError> {
        match regex_captures!(
            r#"^(import)(.*)$"#,
            text,
        ) {
            Some((_, import, rest)) => {
                Ok((
                    rest,
                    Lex::new("import", import, range(import, start))
                ))
            }
            None => {
                Err(LexError::new(text, start, "import", "unknown"))
            }
        }
    }
}

pub struct ExportLexer;

impl Lexer for ExportLexer {
    fn lex<'a>(&mut self, text: &'a str, start: Position) -> anyhow::Result<(&'a str, Lex), LexError> {
        match regex_captures!(
            r#"^(export)(.*)$"#,
            text,
        ) {
            Some((_, export, rest)) => {
                Ok((
                    rest,
                    Lex::new("export", export, range(export, start))
                ))
            }
            None => {
                Err(LexError::new(text, start, "export", "unknown"))
            }
        }
    }
}
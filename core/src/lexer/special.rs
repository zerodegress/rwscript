use crate::lexer::general::RegexLexer;

use super::Lexer;

pub fn special_lexers() -> Vec<Box<dyn Lexer>> {
    vec![
        Box::new(RegexLexer::new(r"\n", "special_newline").unwrap()),
        Box::new(RegexLexer::new(r"\s+", "special_whitespace").unwrap()),
        Box::new(RegexLexer::new(r"//[^\n]*(:?\n)?", "special_single_note").unwrap()),
        Box::new(RegexLexer::new(r"/\*.*\*/", "special_multi_note").unwrap()),
    ]
}
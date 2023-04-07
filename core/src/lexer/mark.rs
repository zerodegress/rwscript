use crate::lexer::general::RegexLexer;

use super::Lexer;

pub fn mark_lexers() -> Vec<Box<dyn Lexer>> {
    vec![
        Box::new(RegexLexer::new("(", "parren_left").unwrap()),
        Box::new(RegexLexer::new(")", "parren_right").unwrap()),
        Box::new(RegexLexer::new("[", "bracket_left").unwrap()),
        Box::new(RegexLexer::new("]", "bracket_right").unwrap()),
        Box::new(RegexLexer::new("{", "brace_left").unwrap()),
        Box::new(RegexLexer::new("}", "brace_right").unwrap()),
    ]
}
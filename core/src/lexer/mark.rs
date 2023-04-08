use crate::lexer::general::RegexLexer;

use super::Lexer;

pub fn mark_lexers() -> Vec<Box<dyn Lexer>> {
    vec![
        Box::new(RegexLexer::new("(", "mark_parren_left").unwrap()),
        Box::new(RegexLexer::new(")", "mark_parren_right").unwrap()),
        Box::new(RegexLexer::new("[", "mark_bracket_left").unwrap()),
        Box::new(RegexLexer::new("]", "mark_bracket_right").unwrap()),
        Box::new(RegexLexer::new("{", "mark_brace_left").unwrap()),
        Box::new(RegexLexer::new("}", "mark_brace_right").unwrap()),
        Box::new(RegexLexer::new("+", "mark_add").unwrap()),
        Box::new(RegexLexer::new("-", "mark_sub").unwrap()),
        Box::new(RegexLexer::new("*", "mark_mul").unwrap()),
        Box::new(RegexLexer::new("/", "mark_div").unwrap()),
        Box::new(RegexLexer::new("=", "mark_assign").unwrap()),
        Box::new(RegexLexer::new(".", "mark_dot").unwrap()),
    ]
}
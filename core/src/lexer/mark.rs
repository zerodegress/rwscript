use crate::lexer::general::RegexLexer;

use super::Lexer;

pub fn mark_lexers() -> Vec<Box<dyn Lexer>> {
    vec![
        Box::new(RegexLexer::new(r"\(", "mark_parren_left").unwrap()),
        Box::new(RegexLexer::new(r"\)", "mark_parren_right").unwrap()),
        Box::new(RegexLexer::new(r"\[", "mark_bracket_left").unwrap()),
        Box::new(RegexLexer::new(r"\]", "mark_bracket_right").unwrap()),
        Box::new(RegexLexer::new(r"\{", "mark_brace_left").unwrap()),
        Box::new(RegexLexer::new(r"\}", "mark_brace_right").unwrap()),
        Box::new(RegexLexer::new(r"\+", "mark_add").unwrap()),
        Box::new(RegexLexer::new("-", "mark_sub").unwrap()),
        Box::new(RegexLexer::new(r"\*\*", "mark_pow").unwrap()),
        Box::new(RegexLexer::new(r"\*", "mark_mul").unwrap()),
        Box::new(RegexLexer::new("/", "mark_div").unwrap()),
        Box::new(RegexLexer::new("%", "mark_mod").unwrap()),
        Box::new(RegexLexer::new("!=", "mark_not_equal").unwrap()),
        Box::new(RegexLexer::new("==", "mark_equal").unwrap()),
        Box::new(RegexLexer::new(">=", "mark_greater_equal_than").unwrap()),
        Box::new(RegexLexer::new(">", "mark_greater_than").unwrap()),
        Box::new(RegexLexer::new("<=", "mark_less_equal_than").unwrap()),
        Box::new(RegexLexer::new("<", "mark_less_than").unwrap()),
        Box::new(RegexLexer::new("=", "mark_assign").unwrap()),
        Box::new(RegexLexer::new(r"\.", "mark_dot").unwrap()),
    ]
}
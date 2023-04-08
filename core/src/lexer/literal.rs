use super::{ Lexer, general::RegexLexer };

pub fn literal_lexers() -> Vec<Box<dyn Lexer>> {
    vec![
        Box::new(RegexLexer::new(r"(?:(?:[1-9][0-9]*)|[0-9])(?:\.[0-9]+)?", "literal_number").unwrap()),
        Box::new(RegexLexer::new("\"(?:.*[^\\\\])?\"", "literal_string").unwrap()),
        Box::new(RegexLexer::new(r"'(?:.*[^\\])?'", "literal_string").unwrap()),
        Box::new(RegexLexer::new(r"(?:true)|(?:false)", "literal_boolean").unwrap()),
    ]
}
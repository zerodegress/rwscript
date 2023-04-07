use super::{ Lexer, general::RegexLexer };

pub fn keyword_lexers() -> Vec<Box<dyn Lexer>> {
    vec![
        Box::new(RegexLexer::new(r"(?:(?:[1-9][0-9]*)|[0-9])(?:\.[0-9]+)?", "number").unwrap()),
        Box::new(RegexLexer::new("\"(?:.*[^\\\\])?\"", "string").unwrap()),
        Box::new(RegexLexer::new(r"'(?:.*[^\\])?'", "string").unwrap()),
        Box::new(RegexLexer::new(r"(?:true)|(?:false)", "boolean").unwrap()),
        Box::new(RegexLexer::new(r"const", "const").unwrap()),
        Box::new(RegexLexer::new(r"import", "import").unwrap()),
        Box::new(RegexLexer::new(r"export", "export").unwrap()),
    ]
}
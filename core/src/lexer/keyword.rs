use super::{ general::RegexLexer, Lexer };

pub fn keyword_lexers() -> Vec<Box<dyn Lexer>> {
    vec![
        Box::new(RegexLexer::new("fn", "fn").unwrap()),
        Box::new(RegexLexer::new("struct", "struct").unwrap()),
        Box::new(RegexLexer::new("trait", "trait").unwrap()),
        Box::new(RegexLexer::new("mut", "mut").unwrap()),
        Box::new(RegexLexer::new("const", "const").unwrap()),
        Box::new(RegexLexer::new("import", "import").unwrap()),
        Box::new(RegexLexer::new("export", "export").unwrap()),
    ]
}
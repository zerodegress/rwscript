use super::{ general::RegexLexer, Lexer };

pub fn keyword_lexers() -> Vec<Box<dyn Lexer>> {
    vec![
        Box::new(RegexLexer::new("fn", "keyword_fn").unwrap()),
        Box::new(RegexLexer::new("struct", "keyword_struct").unwrap()),
        Box::new(RegexLexer::new("trait", "keyword_trait").unwrap()),
        Box::new(RegexLexer::new("mut", "keyword_mut").unwrap()),
        Box::new(RegexLexer::new("const", "keyword_const").unwrap()),
        Box::new(RegexLexer::new("import", "keyword_import").unwrap()),
        Box::new(RegexLexer::new("export", "keyword_export").unwrap()),
    ]
}
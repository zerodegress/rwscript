use super::{ general::RegexLexer, Lexer };

pub fn keyword_lexers() -> Vec<Box<dyn Lexer>> {
    vec![
        Box::new(RegexLexer::new("func", "keyword_func").unwrap()),
        Box::new(RegexLexer::new("struct", "keyword_struct").unwrap()),
        Box::new(RegexLexer::new("trait", "keyword_trait").unwrap()),
        Box::new(RegexLexer::new("mut", "keyword_mut").unwrap()),
        Box::new(RegexLexer::new("const", "keyword_const").unwrap()),
        Box::new(RegexLexer::new("import", "keyword_import").unwrap()),
        Box::new(RegexLexer::new("export", "keyword_export").unwrap()),
        Box::new(RegexLexer::new("not", "keyword_not").unwrap()),
        Box::new(RegexLexer::new("or", "keyword_or").unwrap()),
        Box::new(RegexLexer::new("and", "keyword_and").unwrap()),
        Box::new(RegexLexer::new("async", "keyword_async").unwrap()),
        Box::new(RegexLexer::new("await", "keyword_await").unwrap()),
        Box::new(RegexLexer::new("method", "keyword_method").unwrap()),
        Box::new(RegexLexer::new("override", "keyword_override").unwrap()),
    ]
}
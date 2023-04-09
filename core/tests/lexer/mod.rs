mod literal_test;

#[test]
fn test_lexer() {
    use rwscript_core::lexer::{lex, Lex, Position, range};
    assert_eq!(
        lex("async await"), 
        Ok(vec![
            Lex::new("keyword_async", "async", range("async", Position::new(1, 1))),
            Lex::new("special_whitespace", " ", range(" ", Position::new(1, 6))),
            Lex::new("keyword_await", "await", range("await", Position::new(1, 7))),
        ]),
    );
    assert_eq!(
        lex("//test"), 
        Ok(vec![
            Lex::new("special_single_note", "//test", range("//test", Position::new(1, 1))),
        ]),
    );
}
use rwscript_core::lexer::{
    Lexer,
    Position,
    Lex,
};

#[test]
fn test_number_lexer() {
    use rwscript_core::lexer::literal::NumberLexer;

    let mut lexer = NumberLexer;
    assert_eq!(
        lexer.lex("1.0", Position::new(1, 1)), 
        Ok((
            "", 
            Lex::new(
                "number", 
                "1.0", 
                (Position::new(1, 1), Position::new(1, 4))
            )
        ))
    );
    assert_eq!(
        lexer.lex("32768", Position::new(1, 1)), 
        Ok((
            "", 
            Lex::new(
                "number", 
                "32768", 
                (Position::new(1, 1), Position::new(1, 6))
            )
        ))
    );
    assert_eq!(
        lexer.lex("32768 not number", Position::new(1, 1)), 
        Ok((
            " not number", 
            Lex::new(
                "number", 
                "32768", 
                (Position::new(1, 1), Position::new(1, 6))
            )
        ))
    );
}

#[test]
fn test_string_lexer() {
    use rwscript_core::lexer::literal::StringLexer;

    let mut lexer = StringLexer;
    assert_eq!(
        lexer.lex("\"abc\"", Position::new(1, 1)), 
        Ok((
            "", 
            Lex::new(
                "string", 
                "\"abc\"", 
                (Position::new(1, 1), Position::new(1, 6))
            )
        ))
    );
    assert_eq!(
        lexer.lex("\"abc\\\"\"", Position::new(1, 1)), 
        Ok((
            "", 
            Lex::new(
                "string", 
                "\"abc\\\"\"", 
                (Position::new(1, 1), Position::new(1, 8))
            )
        ))
    );
    assert_eq!(
        lexer.lex("\"a\\\"bc\\\"\"", Position::new(1, 1)), 
        Ok((
            "", 
            Lex::new(
                "string", 
                "\"a\\\"bc\\\"\"", 
                (Position::new(1, 1), Position::new(1, 10))
            )
        ))
    );
    assert_eq!(
        lexer.lex("'a\\'bc\\''", Position::new(1, 1)), 
        Ok((
            "", 
            Lex::new(
                "string", 
                "'a\\'bc\\''", 
                (Position::new(1, 1), Position::new(1, 10))
            )
        ))
    );
}

#[test]
fn test_boolean_lexer() {
    use rwscript_core::lexer::literal::BooleanLexer;

    let mut lexer = BooleanLexer;
    assert_eq!(
        lexer.lex("true", Position::new(1, 1)), 
        Ok((
            "", 
            Lex::new(
                "boolean", 
                "true", 
                (Position::new(1, 1), Position::new(1, 5))
            )
        ))
    );
    assert_eq!(
        lexer.lex("false", Position::new(1, 1)), 
        Ok((
            "", 
            Lex::new(
                "boolean", 
                "false", 
                (Position::new(1, 1), Position::new(1, 6))
            )
        ))
    );
}
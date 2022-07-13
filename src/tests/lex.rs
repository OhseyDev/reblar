use crate::lex::{self, LexerTokens};

#[test]
pub fn lex_test() {
    let act_res = lex::LexerTokens::parse("hello14641", false);
    let exp_res = LexerTokens { tokens: vec![
        lex::Token::Identifior("hello14641".to_string())
    ]};
    assert_eq!(Some(exp_res), act_res);
}

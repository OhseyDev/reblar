use crate::lex::{self, Indent};

#[test]
fn loose() {
    let mode = lex::Mode::loose();
    let act_res = lex::Token::parse(&"hello14641\"hey friend\"".to_string(), mode);
    let exp_res =  vec![
        lex::Token::Identifior("hello14641".to_string()),
        lex::Token::StrLiteral("hey friend".to_string())
    ];
    assert_eq!(Some(exp_res), act_res);
}

#[test]
fn strict() {
    let mode = lex::Mode::strict();
    let act_res = lex::Token::parse(&"hello14641\n\"hey\tfriend\"'c'".to_string(), mode);
    let exp_res = vec![
        lex::Token::Identifior("hello14641".to_string()),
        lex::Token::Indent(Indent::NewLine),
        lex::Token::StrLiteral("hey\tfriend".to_string()),
        lex::Token::CharLiteral('c')
    ];
    assert_eq!(Some(exp_res), act_res);
}

use std::collections::HashMap;


pub type Variables = HashMap<String, Vec<crate::lex::Token>>;

pub fn process(var: &Variables, mut toks: Vec<crate::lex::Token>) -> Vec<crate::lex::Token> {
    let mut index = 0 as usize;
    while index < toks.len() {
        let tok = &toks[index];
        match tok {
            crate::lex::Token::Identifior(n) => {
                let val = var.get(n);
                if n.starts_with("$") && val.is_some() {
                    toks.remove(index);
                    index -= 1;
                    for item in val.unwrap() {
                        toks.insert(index, item.clone());
                        index += 1;
                    }
                }
            }
            _ => { }
        }
        index+=1;
    }
    toks
}
/*
impl ToString for crate::lex::Tokens  {
}
*/
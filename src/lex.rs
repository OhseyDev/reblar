
pub enum Token {
    Identifior(String),
    Integer(i64),
    Floater(f64),
    StrLiteral(String),
    CharLiteral(char),
    Other(String),
}

pub struct LexerTokens {
    tokens: Vec<Token>
}

impl LexerTokens {
    pub fn from(src: &'static str) -> LexerTokens {
        let mut tokens = Vec::new();
        let mut str = String::new();
        let mut state = 0 as u8;
        for c in src.chars() {
            match c {
                '0'..='9' => { push(proc_digit(c, state, &mut str), &mut state, &mut tokens, &mut str); }
                'a'..='z' | 'A'..='Z' => { push(proc_letter(c, state, &mut str), &mut state, &mut tokens, &mut str); }
                _ => {}
            }
        }
        LexerTokens { tokens }
    }
    pub fn tokens(&self) -> &Vec<Token> { &self.tokens }
}

#[inline]
fn proc_digit(c: char, state: u8, str: &mut String) -> u8 {
    match state {
        1|2 => { str.push(c); 0 }
        _ => { 1 }
    }
}

#[inline]
fn proc_letter(c: char, state: u8, str: &mut String) -> u8 {
    match state {
        0 => { str.push(c); 2 }
        2 => { str.push(c); 0 }
        _ => { 2 }
    }
}

#[inline]
fn pop(state: u8, str: &String) -> Option<Token> {
    match state {
        0 => { None }
        1 => { Some(Token::Integer(str.parse().unwrap())) }
        2 => { Some(Token::Identifior(str.clone())) }
        _ => { Some(Token::Other(str.clone())) }
    }
}

#[inline]
fn push(change: u8, state: &mut u8, tokens: &mut Vec<Token>, str: &mut String) {
    if change > 0 {
        let res = pop(*state, &str);
        if res.is_some() { tokens.push(res.unwrap()); }
        *state = change;
        str.clear();
    }
}

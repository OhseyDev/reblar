
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
    pub fn parse(src: &'static str) -> LexerTokens {
        let mut tokens = Vec::new();
        let mut str = String::new();
        /* 
            0 => Undecided
            1 => Number
            2 => Literal
            3 => Decimal Number
        */
        let mut state = 0 as u8;
        for c in src.chars() {
            match c {
                '0'..='9' => { push(c, proc_digit(c, state, &mut str), &mut state, &mut tokens, &mut str); }
                'a'..='z' | 'A'..='Z' => { push(c, proc_letter(c, state, &mut str), &mut state, &mut tokens, &mut str); }
                '.' => {  }
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
        0 => { 2 }
        2 => { str.push(c); 0 }
        _ => { 2 }
    }
}

#[inline]
fn proc_dot(c: char, state: u8, str: &mut String) -> u8 {
    match state {
        0 => { 4 }
        1 => { 3 }
        2 => { 4 }
        _ => { 4 }
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
fn push(c: char, change: u8, state: &mut u8, tokens: &mut Vec<Token>, str: &mut String) {
    if change > 0 {
        let res = pop(*state, &str);
        if res.is_some() { tokens.push(res.unwrap()); }
        *state = change;
        str.clear();
        str.push(c);
    }
}


#[derive(Debug)]
pub enum Token {
    Identifior(String),
    Integer(i64),
    Floater(f64),
    StrLiteral(String),
    CharLiteral(char),
    Indent,
    Other(String),
}

#[derive(Debug, Copy, Clone)]
pub struct Mode {
    pub strict_literals: bool,
    pub indent_sensitive: bool
}

impl Mode {
    pub fn loose() -> Mode { Mode { strict_literals: false, indent_sensitive: false } }
    pub fn strict() -> Mode { Mode { strict_literals: true, indent_sensitive: true } }
}

impl Token {
    pub fn identifier(&self) -> Option<String> {
        match self {
            Self::Identifior(str) => Some(str.clone()),
            _ => None
        }
    }
    pub fn indent(&self) -> bool { *self == Self::Indent }
    pub fn char_lit(&self) -> Option<char> {
        match self {
            Self::CharLiteral(c) => Some(*c),
            _ => None
        }
    }
    pub fn str_lit(&self) -> Option<String> {
        match self {
            Self::StrLiteral(str) => Some(str.clone()),
            _ => None
        }
    }
    pub fn floater(&self) -> Option<f64> {
        match self {
            Self::Floater(f) => Some(*f),
            _ => None
        }
    }
    pub fn integer(&self) -> Option<i64> {
        match self {
            Self::Integer(i) => Some(*i),
            _ => None
        }
    }
    pub fn other(&self) -> Option<String> {
        match self {
            Self::Other(c) => Some(c.clone()),
            _ => None
        }
    }
    pub fn parse(src: &'static str, mode: Mode) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();
        let mut str = String::new();
        /* 
            0 => Undecided
            1 => Number
            2 => Identifier
            3 => Decimal Number
            4 => Undecided Dot
            5 => Character Literal
            6 => String Literal
            7 => Reserved
            8 => Indent
        */
        let mut state = 0 as u8;
        let mut string_slash = false;
        for c in src.chars() {
            let not_ok;
            if string_slash {
                str.push(c);
                string_slash = false;
                continue;
            }
            match c {
                '\\' => {
                    if state > 6 && state < 5 { continue; }
                    string_slash = true;
                    not_ok = false;
                }
                '0'..='9' => { not_ok = push(c, proc_digit(state), &mut state, &mut tokens, &mut str, mode); }
                'a'..='z' | 'A'..='Z' => { not_ok = push(c, proc_letter(state), &mut state, &mut tokens, &mut str, mode); }
                '.' => { not_ok = push(c, proc_dot(state), &mut state, &mut tokens, &mut str, mode); }
                '\'' => {
                    let change = {
                        let s = mode.strict_literals as u8;
                        let l = 6 * (!mode.strict_literals) as u8;
                        l + proc_quote(state, true) * s
                    };
                    not_ok = push(c, change, &mut state, &mut tokens, &mut str, mode);
                }
                '\n'|'\t' => {
                    not_ok = push(c, 8, &mut state, &mut tokens, &mut str, mode);
                }
                '\"' => { not_ok = push(c, 6, &mut state, &mut tokens, &mut str, mode); }
                _ => { not_ok = push(c, 255, &mut state, &mut tokens, &mut str, mode); }
            };
            if not_ok { return None; }
        }
        Some(tokens)
    }
}

impl PartialEq for Token {
    fn eq(&self, other: &Self) -> bool {
        match other {
            Self::Identifior(n) => {
                let ident = self.identifier();
                if ident.is_none() { return false; }
                ident.unwrap() == *n
            }
            Self::CharLiteral(c) => {
                let c_lit = self.char_lit();
                if c_lit.is_none() { return false; }
                c_lit.unwrap() == *c
            }
            Self::Floater(f) => {
                let floater = self.floater();
                if floater.is_none() { return false }
                floater.unwrap() == *f
            }
            Self::Integer(i) => {
                let integer = self.integer();
                if integer.is_none() { return false }
                integer.unwrap() == *i
            }
            Self::StrLiteral(str) => {
                let str_lit = self.str_lit();
                if str_lit.is_none() { return false }
                str_lit.unwrap() == *str
            }
            Self::Other(c) => {
                let other = self.other();
                if other.is_none() { return false }
                other.unwrap() == *c
            }
            Self::Indent => {
                self.indent()
            }
        }
    }
}

#[inline]
fn proc_digit(state: u8) -> u8 {
    match state {
        1..=3|5..=6 => { 0 }
        4 => { 3 }
        _ => { 1 }
    }
}

#[inline]
fn proc_quote(state: u8, single: bool) -> u8 {
    match state {
        5 => { 7 }
        _ => {
            match single {
                false => { 6 }
                true => { 5 }
            }
        }
    }
}

#[inline]
fn proc_letter(state: u8) -> u8 {
    match state {
        2|5|6 => { 0 }
        _ => { 2 }
    }
}

#[inline]
fn proc_dot(state: u8) -> u8 {
    match state {
        1 => { 3 }
        _ => { 4 }
    }
}

#[inline]
fn pop(state: u8, str: &String) -> (bool, Option<Token>) {
    match state {
        0 => { (true, None) }
        1 => { (true, Some(Token::Integer(str.parse().unwrap()))) }
        2 => { (true, Some(Token::Identifior(str.clone()))) }
        3 => { (true, Some(Token::Floater(str.parse().unwrap()))) }
        5 => {
            if str.len() > 1 { return (false, None); }
            (true, Some(Token::CharLiteral(str.chars().next().unwrap())))
        }
        6 => { (true, Some(Token::StrLiteral(str.clone()))) }
        8 => { (true, Some(Token::Indent)) }
        _ => { (true, Some(Token::Other(str.clone()))) }
    }
}

#[inline]
fn push(c: char, change: u8, state: &mut u8, tokens: &mut Vec<Token>, str: &mut String, mode: Mode) -> bool {
    if change > 0 && change < 255 {
        let (ok, res) = pop(*state, &str);
        if !ok { return true; }
        *state = change;
        if res.is_some() {
            tokens.push(res.unwrap());
            str.clear();
        }
    }
    if change > 6 || change < 5 {
        str.push(c);
    }
    mode.strict_literals && str.len() >= 2 && *state == 5
}

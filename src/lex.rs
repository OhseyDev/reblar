use std::ops::{Index, IndexMut};

#[derive(Debug,Clone,PartialEq,Eq)]
pub struct Tokens { vec: Vec<Token> }
impl Tokens {
    pub fn new() -> Self { Self { vec: vec![] } }
    pub fn push(&mut self, val: Token) { self.vec.push(val); }
    pub fn len(&self) -> usize { self.vec.len() }
}
impl From<Vec<Token>> for Tokens { fn from(vec: Vec<Token>) -> Self { Self { vec } } }
impl Into<Vec<Token>> for Tokens { fn into(self) -> Vec<Token> { self.vec } }
impl IndexMut<usize> for Tokens { fn index_mut(&mut self, index: usize) -> &mut Self::Output { self.vec.index_mut(index) } }
impl Index<usize> for Tokens {
    type Output = Token;
    fn index(&self, index: usize) -> &Self::Output { self.vec.index(index) }
}
impl IntoIterator for Tokens {
    type Item = Token;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        return self.vec.into_iter()
    }
}
impl ToString for Literal {
    fn to_string(&self) -> String {
        match self {
            Literal::Character(c) => format!("{}", c),
            Literal::Floater(f) => format!("{}", f),
            Literal::Integer(i) => format!("{}", i),
            Literal::String(s) => s.clone()
        }
    }
}
impl ToString for Tokens {
    fn to_string(&self) -> String {
        let mut str = String::new();
        for token in self.vec.clone() {
            match token {
                Token::Literal(lit) => { str += format!("\"{}\"", lit.to_string()).as_str(); }
                Token::Identifior(ident) => { str += ident.as_str() }
                Token::Indent(indent) => {
                    match indent {
                        Indent::NewLine => { str.insert(str.chars().count(), '\n'); }
                        Indent::Tab => { str.insert(str.chars().count(), '\t'); }
                        Indent::Space => { str.insert(str.chars().count(), ' '); }
                    }
                }
                Token::Other(s) => { str += s.as_str(); }
                Token::OpenBracket => { str.insert(str.chars().count(), '('); }
                Token::CloseBracket => { str.insert(str.chars().count(), ')'); }
                Token::None => { continue }
            }
        }
        str
    }
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Indent { NewLine, Tab, Space }
#[derive(Debug, Clone, PartialEq)]
pub enum Literal { String(String),Integer(i64),Floater(f64),Character(char) }
#[derive(Debug, Clone)]
pub enum Token {
    Identifior(String),
    Literal(Literal),
    Indent(Indent),Other(String),
    OpenBracket,CloseBracket,
    None,
}
impl Eq for Token {}
#[derive(Debug, Copy, Clone)]
pub struct IndentMode {
    pub spaces: bool,
    pub tabs: bool,
    pub newlines: bool,
}

#[derive(Debug, Copy, Clone)]
pub struct Mode {
    pub strict_literals: bool,
    pub indents: IndentMode,
    pub schar_identp: bool,
    pub schar_vals: Option<&'static [char]>
}
 
impl Mode {
    pub fn loose() -> Mode { Mode { strict_literals: false, indents: IndentMode::weak(), schar_identp: true, schar_vals: None } }
    pub fn strict() -> Mode { Mode { strict_literals: true, indents: IndentMode::strong(), schar_identp: false, schar_vals: None } }
}

impl IndentMode {
    pub const fn strong() -> IndentMode { Self { spaces: false, tabs: true, newlines: true } }
    pub const fn weak() -> IndentMode { Self { spaces: false, tabs: false, newlines: true } }
    pub const fn max() -> IndentMode { Self { spaces: true, tabs: true, newlines: true } }
    pub const fn min() -> IndentMode { Self { spaces: false, tabs: false, newlines: false } }
}

impl Token {
    pub fn identifier(&self) -> Option<String> {
        match self {
            Self::Identifior(str) => Some(str.clone()),
            _ => None
        }
    }
    pub fn indent(&self) -> Option<Indent> {
        match self {
            Self::Indent(i) => Some(i.clone()),
            _ => None
        }
    }
    pub fn literal(&self) -> Option<Literal> {
        match self {
            Self::Literal(lit) => Some(lit.clone()),
            _ => None
        }
    }
    pub fn other(&self) -> Option<String> {
        match self {
            Self::Other(c) => Some(c.clone()),
            _ => None
        }
    }
    pub fn none(&self) -> bool {
        match self {
            Self::None => true,
            _ => false
        }
    }
    pub fn cbracket(&self) -> bool {
        match self {
            Self::CloseBracket => true,
            _ => false
        }
    }
    pub fn obracket(&self) -> bool {
        match self {
            Self::OpenBracket => true,
            _ => false
        }
    }
    pub fn parse(src: &String, mode: Mode) -> Option<Tokens> {
        let mut tokens = Tokens::new();
        let mut str = String::new();
        let mut state = 0 as u8;
        let mut last_c = '\0';
        for c in src.chars() {
            let not_ok;
            if last_c == '\\' && (state == 6 && state == 5) {
                str.push(c);
                last_c = '\0';
                continue;
            }
            match c {
                '\\' => {
                    if state <= 6 && state >= 5 { not_ok = false; }
                    else { not_ok = push(c, 255, &mut state, &mut tokens, &mut str, mode); }
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
                '\n' => {
                    let chng = 8 * (state != 5 && state != 6) as u8;
                    not_ok = push(c, chng as u8, &mut state, &mut tokens, &mut str, mode);
                }
                '\t' => {
                    let chng = mode.indents.tabs as u8 * (state != 5 && state != 6) as u8;
                    not_ok = push(c, chng as u8, &mut state, &mut tokens, &mut str, mode);
                }
                ' ' => {
                    let chng = (mode.indents.spaces as u8 * 2) * (state != 5 && state != 6) as u8;
                    not_ok = push(c, chng, &mut state, &mut tokens, &mut str, mode);
                }
                '\"' => { not_ok = push(c, 6, &mut state, &mut tokens, &mut str, mode); }
                _ => {
                    let mut cstate = 255;
                    if mode.schar_vals.is_some() && state == 2 {
                        let schar_vals = mode.schar_vals.unwrap();
                        let cond = {
                            let cond = schar_vals.contains(&c);
                            cond ^ mode.schar_identp
                        };
                        cstate = 255 * cond as u8;
                    }
                    not_ok = push(c, cstate, &mut state, &mut tokens, &mut str, mode);
                }
            };
            if not_ok { return None; }
            last_c = c;
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
            Self::Literal(literal) => {
                let o_lit = self.literal();
                if o_lit.is_none() { return false; }
                o_lit.unwrap() == *literal
            }
            Self::Other(c) => {
                let other = self.other();
                if other.is_none() { return false }
                other.unwrap() == *c
            }
            Self::Indent(i) => {
                let other = self.indent();
                if other.is_none() { return false }
                other.unwrap() == *i
            }
            Self::None => { self.none() }
            Self::CloseBracket => self.cbracket(),
            Self::OpenBracket => self.obracket()
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
fn pop(state: u8, str: &String) -> (bool, Token) {
    if str.is_empty() { return (true, Token::None); }
    match state {
        1 => { (true, Token::Literal(Literal::Integer(str.parse().unwrap()))) }
        2 => { (true, Token::Identifior(str.clone())) }
        3 => { (true, Token::Literal(Literal::Floater(str.parse().unwrap()))) }
        5 => {
            if str.len() > 1 { return (false, Token::None); }
            (true, Token::Literal(Literal::Character(str.chars().next().unwrap())))
        }
        6 => { (true, Token::Literal(Literal::String(str.clone()))) }
        8 => { (true, Token::Indent(Indent::NewLine)) }
        9 => { (true, Token::Indent(Indent::Tab)) }
        10 => { (true, Token::Indent(Indent::Space)) }
        _ => { (true, Token::Other(str.clone())) }
    }
}

#[inline]
fn push(c: char, change: u8, state: &mut u8, tokens: &mut Tokens, str: &mut String, mode: Mode) -> bool {
    if change > 0 && change < 255 {
        let (ok, res) = pop(*state, &str);
        if !ok { return true; }
        *state = change;
        if !res.none() {
            tokens.push(res);
            str.clear();
        }
    }
    if change > 6 || change < 5 {
        str.push(c);
    }
    mode.strict_literals && str.len() >= 2 && *state == 5
}

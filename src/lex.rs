
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum Indent { NewLine, Tab, Space }

#[derive(Debug, Clone)]
pub enum Token {
    Identifior(String),
    Integer(i64),
    Floater(f64),
    StrLiteral(String),
    CharLiteral(char),
    Indent(Indent),
    Other(String),
    None,
}

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
    pub fn none(&self) -> bool {
        match self {
            Self::None => { true }
            _ => { false }
        }
    }
    pub fn parse(src: &String, mode: Mode) -> Option<Vec<Token>> {
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
            Self::Indent(i) => {
                let other = self.indent();
                if other.is_none() { return false }
                other.unwrap() == *i
            }
            Self::None => { self.none() }
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
        1 => { (true, Token::Integer(str.parse().unwrap())) }
        2 => { (true, Token::Identifior(str.clone())) }
        3 => { (true, Token::Floater(str.parse().unwrap())) }
        5 => {
            if str.len() > 1 { return (false, Token::None); }
            (true, Token::CharLiteral(str.chars().next().unwrap()))
        }
        6 => { (true, Token::StrLiteral(str.clone())) }
        8 => { (true, Token::Indent(Indent::NewLine)) }
        9 => { (true, Token::Indent(Indent::Tab)) }
        10 => { (true, Token::Indent(Indent::Space)) }
        _ => { (true, Token::Other(str.clone())) }
    }
}

#[inline]
fn push(c: char, change: u8, state: &mut u8, tokens: &mut Vec<Token>, str: &mut String, mode: Mode) -> bool {
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

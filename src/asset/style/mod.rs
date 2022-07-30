pub mod bg;

use crate::{lex, traits::Resource};
use std::{collections::BTreeMap, io::Read, str::FromStr};

#[derive(Clone, Debug)]
pub(crate) struct ParseState {
    /*
        0x00 => NULL
        0xxf => Done
        0x1x => Selector
        0x2x => Property
        0x3x => Value
        0xx1 => Append
    */
    pub status: u8,
    pub ok: bool,
    pub last_tok: lex::Token
}
pub(crate) type InnerParseOutput = (ParseState, Option<Value>);
#[derive(Debug, Clone)]
pub enum Property {
    Background(Option<bg::BackgroundProperty>),
}
#[derive(Debug, Clone)]
pub enum Value {
    Background(bg::BackgroundValue),
}
#[derive(Debug, Clone)]
pub struct Rule {
    property: Property,
    value: Value,
}
impl Default for ParseState {
    fn default() -> Self {
        ParseState { status: 0x01, ok: true, last_tok: lex::Token::None }
    }
}
impl crate::traits::PropertyValue for Rule {
    type PropertyType = Property;
    type ValueType = Value;
    fn property(&self) -> &Property { &self.property }
    fn value(&self) -> &Value { &self.value }
}
#[derive(Debug)]
pub enum Error {
    IOError(std::io::Error),
    SyntaxError,
    InvalidValue,
    InvalidProperty,
    Unknown,
}
#[derive(Debug, Clone)]
pub struct Asset {
    name: String,
    rules: BTreeMap<Selector, Vec<Rule>>,
}
#[derive(Debug, Clone)]
pub struct Selector {
    pub(crate) name: String,
    pub(crate) parent: Option<Box<Selector>>,
}
impl Selector {
    pub fn empty() -> Self {
        Self {
            name: String::new(),
            parent: None,
        }
    }
}
impl crate::traits::Name for Asset { fn name(&self) -> &String { &self.name } }
impl crate::traits::Name for Selector { fn name(&self) -> &String { &self.name } }
impl ToString for Selector {
    fn to_string(&self) -> String {
        let mut str = self.name.clone();
        let mut parent = &self.parent;
        while self.parent.is_some() {
            let inuse = parent.as_ref().unwrap();
            str = inuse.name.clone() + " " + str.as_str();
            parent = &inuse.parent;
        }
        return str;
    }
}
impl FromStr for Property {
    type Err = Error;
    fn from_str(s: &str) -> Result<Property, Self::Err> {
        match s {
            "background" => Ok(Self::Background(None)),
            "background-attachment" => Ok(Self::Background(Some(bg::BackgroundProperty::Attachment))),
            "background-blend-mode" => Ok(Self::Background(Some(bg::BackgroundProperty::BlendMode))),
            "background-clip" => Ok(Self::Background(Some(bg::BackgroundProperty::Clip))),
            "background-color" => Ok(Self::Background(Some(bg::BackgroundProperty::Color))),
            "background-image" => Ok(Self::Background(Some(bg::BackgroundProperty::Image))),
            "background-origin" => Ok(Self::Background(Some(bg::BackgroundProperty::Origin))),
            "background-repeat" => Ok(Self::Background(Some(bg::BackgroundProperty::Repeat))),
            "background-size" => Ok(Self::Background(Some(bg::BackgroundProperty::Size))),
            _ => Err(Error::InvalidProperty),
        }
    }
}
impl Asset {
    const VALS: [char; 2] = ['$', '-'];
    const MODE_NORM:crate::lex::Mode=crate::lex::Mode{strict_literals:false,schar_identp:true,indents:lex::IndentMode::min(),schar_vals:Some(&Self::VALS)};
    const MODE_SASSY:crate::lex::Mode=crate::lex::Mode{strict_literals:false,schar_identp:true,indents:lex::IndentMode::strong(),schar_vals:Some(&Self::VALS)};
    pub fn rules(&self) -> &BTreeMap<Selector, Vec<Rule>> { &self.rules }
}
impl Resource for Asset {
    type Error = Error;
    type Options = bool;
    fn file(path: &std::path::Path, sassy: bool) -> Result<Self, Self::Error> {
        let mut file = {
            let f = std::fs::File::open(path);
            if f.is_err() { return Err(Error::IOError(f.err().unwrap())); }
            f.unwrap()
        };
        let (src, _len) = {
            let mut src = String::new();
            let res = file.read_to_string(&mut src);
            if res.is_err() { return Err(Error::IOError(res.err().unwrap())); }
            (src, res.unwrap())
        };
        let tokens = {
            let mode;
            if sassy { mode = Self::MODE_SASSY; }
            else { mode = Self::MODE_NORM; }
            let res = crate::lex::Token::parse(&src, mode);
            if res.is_none() { return Err(Error::Unknown); }
            res.unwrap()
        };
        let rules = BTreeMap::new();
        let mut selector = Selector::empty();
        let mut block_indent = 0 as u8;
        let mut property = None;
        let mut state = ParseState::default();
        
        let mut index = 0 as usize;

        while index < tokens.len() {
            let token = &tokens[index];
            index += 1;
            if property.is_some() {
                let (new_state, _val) = parse(&state, &token, property.as_ref().unwrap());
                state = new_state;
                if !state.ok { return Err(Error::InvalidValue); }
                if state.status == 0x01 { state.status = 0x31; }
                continue;
            }
            match token.clone() {
                crate::lex::Token::Identifior(n) => {
                    let i = state.last_tok.identifier();
                    let n = {
                        if state.status == 2 { ":".to_owned() + n.as_str() }
                        else { n.clone() }
                    };
                    if i.is_some() {
                        let i = i.unwrap();
                        if selector.name.is_empty() {
                            selector.name = i.clone();
                        } else {
                            selector = Selector {
                                name: n.clone(),
                                parent: Some(Box::from(selector)),
                            };
                        }
                    }
                }
                crate::lex::Token::Indent(i) => {
                    if i == crate::lex::Indent::NewLine {
                        if state.status == 0x31 { return Err(Error::SyntaxError); }
                        state.status = 0x11;
                    }
                    if !sassy {
                        state.status = 0x0f;
                        continue;
                    }
                }
                crate::lex::Token::Other(c) => match c.as_str() {
                    "{" => {
                        if sassy { return Err(Error::SyntaxError); }
                        block_indent += 1
                    }
                    "}" => {
                        if block_indent == 0 || sassy { return Err(Error::SyntaxError); }
                        block_indent -= 1;
                    }
                    ":" => {
                        if state.status == 0x11 { state.status = 2; continue; }
                        let ident = state.last_tok.identifier();
                        if ident.is_some() {
                            let prop = Property::from_str(ident.unwrap().as_str());
                            if prop.is_err() { return Err(prop.err().unwrap()); }
                            property = Some(prop.unwrap());
                        }
                    }
                    _ => {}
                },
                _ => {}
            }
            state.last_tok = token.clone();
        }
        Ok(Self { name: String::from(path.file_name().unwrap().to_string_lossy()), rules })
    }
}
#[inline]
fn parse(state: &ParseState, token: &lex::Token, property: &Property) -> InnerParseOutput {
    match property {
        Property::Background(b) => bg::parse(state, b, token),
    }
}
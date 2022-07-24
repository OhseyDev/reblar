pub mod bg;

use std::{collections::BTreeMap, io::Read};
use crate::{lex, traits::{Resource, Parse}};

#[derive(Debug,Clone)]
pub enum Property {
    Background(bg::BackgroundProperty)
}
#[derive(Debug,Clone)]
pub enum Value {}
#[derive(Debug,Clone)]
pub struct Rule {
    property: Property,
    value: Value
}
impl crate::traits::PropertyValue for Rule {
    type PropertyType = Property;
    type ValueType = Value;
    fn property(&self) -> &Property { &self.property }
    fn value(&self) -> &Value { &self.value }
}
#[derive(Debug)]
pub enum Error { IOError(std::io::Error),SyntaxError,InvalidValue,InvalidProperty,Unknown }
#[derive(Debug,Clone)]
pub struct Asset {
    name: String,
    rules: BTreeMap<Selector, Vec<Rule>>,
}
#[derive(Debug,Clone)]
pub struct Selector {
    pub(crate) name: String,
    pub(crate) parent: Option<Box<Selector>>
}
impl Selector { pub fn empty() -> Self { Self { name: String::new(), parent: None } } }
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
impl crate::traits::Parse<String> for Property {
    type Error = Error;
    fn parse(src: String) -> Result<Box<Self>, Self::Error> {
        match src {
            _ => Err(Error::InvalidProperty)
        }
    }
}
impl Asset {
    const VALS: [char; 2] = ['$', '-'];
    const MODE_NORM: crate::lex::Mode = crate::lex::Mode { strict_literals: false, schar_identp: true, indents: lex::IndentMode::min(), schar_vals: Some(&Self::VALS) };
    const MODE_SASSY: crate::lex::Mode = crate::lex::Mode { strict_literals: false, schar_identp: true, indents: lex::IndentMode::strong(), schar_vals: Some(&Self::VALS) };
    pub fn rules(&self) -> &BTreeMap<Selector, Vec<Rule>> { &self.rules }
}
impl Resource for Asset {
    type Error = Error;
    type Options = bool;
    fn file(path: &std::path::Path, sassy: bool) -> Result<Self, Self::Error> {
        let mut file = {
            let f = std::fs::File::open(path);
            if f.is_err() { return Err(Error::IOError(f.err().unwrap()))}
            f.unwrap()
        };
        let (src, _len) = {
            let mut src  = String::new();
            let res = file.read_to_string(&mut src);
            if res.is_err() { return Err(Error::IOError(res.err().unwrap()));}
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
        let mut is_selector = false;
        let mut is_value = false;
        let mut append_selector = false;
        let mut property = None;
        let mut last_tok = lex::Token::None;
        for token in tokens {
            if property.is_some() {
                continue;
            }
            match token.clone() {
                crate::lex::Token::Identifior(n) => {
                    let i = last_tok.identifier();
                    let n = {
                        if append_selector { ":".to_owned() + n.as_str() }
                        else { n.clone() }
                    };
                    if i.is_some() {
                        let i = i.unwrap();
                        if selector.name.is_empty() { selector.name = i.clone(); }
                        else { selector = Selector { name: n.clone(), parent: Some(Box::from(selector)) }; }
                    }
                }
                crate::lex::Token::Indent(i) => {
                    if i == crate::lex::Indent::NewLine && !is_value { is_selector = true; }
                    if !sassy { is_value = false; continue; }
                }
                crate::lex::Token::Other(c) => {
                    match c.as_str() {
                        "{" => {
                            if sassy { return Err(Error::SyntaxError); }
                            block_indent+=1
                        }
                        "}" => {
                            if block_indent == 0 || sassy { return Err(Error::SyntaxError); }
                            block_indent-=1;
                        }
                        ":" => {
                            if is_selector {
                                append_selector = true;
                                is_selector = false;
                                continue;
                            }
                            let ident = last_tok.identifier();
                            if ident.is_some() {
                                let prop = Property::parse(ident.unwrap());
                                if prop.is_err() { return Err(prop.err().unwrap()) }
                                property = Some(prop.unwrap());
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            last_tok = token.clone();
        }
        Ok(Self { name: String::from(path.file_name().unwrap().to_string_lossy()), rules })
    }
}
impl crate::traits::Name for Asset { fn name(&self) -> &String { &self.name } }

use std::collections::BTreeMap;

use crate::lex::{IndentMode, Token};

/*
    Style-rule data structures
*/
pub enum Property {
    Background()
}

pub enum Value {

}

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

/*
    Style Selector
*/

pub struct Selector {
    pub(crate) name: String,
    pub(crate) parent: Option<Box<Selector>>
}

impl Selector {
    pub fn empty() -> Self {
        Self { name: String::new(), parent: None }
    }
    pub fn construct(&self) -> String {
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

impl crate::traits::ParentChild for Selector {
    type ParentType = Option<Box<Self>>;
    type ChildType = Option<&'static Self>;
    fn child(&self) -> &Self::ChildType { &None }
    fn parent(&self) -> &Self::ParentType { &self.parent }
}

impl crate::traits::Name for Selector {
    fn name(&self) -> &String { &self.name }
}

/*
    Style struct
*/
pub struct Asset {
    name: String,
    rules: BTreeMap<Selector, Vec<Rule>>,
}

impl Asset {
    const VALS: [char; 2] = ['$', '-'];
    const MODE_NORM: crate::lex::Mode = crate::lex::Mode { strict_literals: false, schar_identp: true, indents: IndentMode::min(), schar_vals: Some(&Self::VALS) };
    const MODE_SASSY: crate::lex::Mode = crate::lex::Mode { strict_literals: false, schar_identp: true, indents: IndentMode::strong(), schar_vals: Some(&Self::VALS) };
    
    pub fn parse(name: &'static str, src: &'static str, sassy: bool) -> Option<Self> {
        let tokens = {
            let mode;
            if sassy { mode = Self::MODE_SASSY; }
            else { mode = Self::MODE_NORM; }
            let res = crate::lex::Token::parse(src, mode);
            if res.is_none() { return None; }
            res.unwrap()
        };
        let rules = BTreeMap::new();
        let mut selector = Selector::empty();
        let mut ident: Option<String> = None;
        let mut block_indent = 0 as u8;
        let mut is_selector = false;
        let mut is_value = false;
        let mut append_selector = false;
        let mut last_tok = Token::None;
        for token in tokens {
            match token.clone() {
                crate::lex::Token::Identifior(n) => {
                    let i = ident.as_ref();
                    let n = {
                        if append_selector { ":".to_owned() + n.as_str() }
                        else { n.clone() }
                    };
                    if i.is_some() {
                        let i = i.unwrap();
                        if selector.name.is_empty() {
                            selector.name = i.clone();
                        } else {
                            let new_selector = Selector { name: n.clone(), parent: Some(Box::from(selector)) };
                            selector = new_selector;
                        }
                    }
                    ident = Some(n.clone());
                }
                crate::lex::Token::Indent(i) => {
                    if i == crate::lex::Indent::NewLine && !is_value { is_selector = true; }
                    if !sassy {
                        is_value = false;
                        continue;
                    }
                }
                crate::lex::Token::Other(c) => {
                    if sassy { return None; }
                    match c.as_str() {
                        "{" => {
                            block_indent+=1;
                        }
                        "}" => {
                            if block_indent == 0 { return None; }
                            block_indent-=1;
                        }
                        ":" => {
                            if is_selector {
                                append_selector = true;
                                is_selector = false;
                            }
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
            last_tok = token.clone();
        }
        Some(Self { name: String::from(name), rules })
    }
    pub fn rules(&self) -> &BTreeMap<Selector, Vec<Rule>> { &self.rules }
}

impl crate::traits::Name for Asset {
    fn name(&self) -> &String { &self.name }
}

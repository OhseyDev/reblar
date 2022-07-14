use std::collections::BTreeMap;

use crate::lex;

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
    pub(crate) parent: Option<&'static Selector>,
    pub(crate) child: Option<Box<Selector>>
}

impl Selector {
    pub fn empty() -> Self {
        Self { name: String::new(), parent: None, child: None }
    }
}

impl crate::traits::ParentChild for Selector {
    type ChildType = Option<Box<Self>>;
    type ParentType = Option<&'static Self>;
    fn child(&self) -> &Self::ChildType { &self.child }
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
    pub fn parse(name: &'static str, tokens: &Vec<lex::Token>, sass_syntax: bool) -> Option<Self> {
        let mut rules = BTreeMap::new();
        let mut selector = Selector::empty();
        let mut sel_child = None;
        for token in tokens {
            match token {
                lex::Token::Identifior(n) => {
                    if !selector.name.is_empty() {
                        selector.child = Some(Box::new(Selector::empty()));
                        sel_child = Some(&selector.child);
                    }
                }
                _ => {}
            }
        }
        Some(Self { name: String::from(name), rules })
    }
    pub fn rules(&self) -> &BTreeMap<Selector, Vec<Rule>> { &self.rules }
}

impl crate::traits::Name for Asset {
    fn name(&self) -> &String { &self.name }
}

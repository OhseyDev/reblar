use std::collections::BTreeMap;

/*
    Style-rule
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
    name: &'static str,
    child: Option<&'static Selector>,
    parent: Option<&'static Selector>
}

impl crate::traits::ParentChild for Selector {
    fn child(&self) -> Option<&'static Self> { self.child }
    fn parent(&self) -> Option<&'static Self> { self.parent }
}

impl crate::traits::Name for Selector {
    fn name(&self) -> &'static str { self.name }
}

/*
    Style struct
*/
pub struct Asset {
    name: &'static str,
    rules: BTreeMap<Selector, Vec<Rule>>,
}

impl crate::traits::Name for Asset {
    fn name(&self) -> &'static str { self.name }
}

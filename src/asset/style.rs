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
    Style struct
*/
pub struct Style {
    name: &'static str,
    rules: BTreeMap<String, Vec<Rule>>,
}

impl crate::traits::Name for Style {
    fn name(&self) -> &'static str { self.name }
}

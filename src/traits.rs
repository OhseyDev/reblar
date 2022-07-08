pub trait Name { fn name(&self) -> &'static str; }
pub trait PropertyValue {
    type PropertyType;
    type ValueType;

    fn property(&self) -> &Self::PropertyType;
    fn value(&self) -> &Self::ValueType;
}


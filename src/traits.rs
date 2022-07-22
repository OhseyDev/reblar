pub trait Name { fn name(&self) -> &String; }
pub trait PropertyValue {
    type PropertyType;
    type ValueType;
    fn property(&self) -> &Self::PropertyType;
    fn value(&self) -> &Self::ValueType;
}
pub trait Parse {
    type Error;
    fn parse(src: crate::lex::Tokens) -> Result<Box<Self>, Self::Error>;
}
pub trait Resource: Sized {
    type Error;
    type Options;
    fn file(path: &std::path::Path, options: Self::Options) -> Result<Self, Self::Error>;
}
pub trait Asset: Resource + Sized {
    fn src(src: &String, options: Self::Options) -> Result<Self, Self::Error>;
}
pub trait ResourceBuilder {
    type Resource: Resource;
    type Error;
    fn build(&self) -> Result<Self::Resource, Self::Error>;
}

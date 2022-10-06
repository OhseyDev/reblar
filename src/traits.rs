pub trait Name { fn name(&self) -> &String; }
pub trait PropertyValue {
    type PropertyType;
    type ValueType;
    fn property(&self) -> &Self::PropertyType;
    fn value(&self) -> &Self::ValueType;
}
pub trait Resource: Name + Sized {
    type Error;
    type Options;
    type Source;
    fn file(path: &std::path::Path, options: Self::Options) -> Result<Self, Self::Error>;
    fn src(src: &Self::Source, options: Self::Options) -> Result<Self, Self::Error>;
}
pub trait Asset: Resource {}
pub trait Document: Resource {
    fn src(src: &String, options: Self::Options) -> Result<Self, Self::Error>;
}
pub trait Builder {
    type Resource: Resource;
    type Error;
    fn build(&self) -> Result<Self::Resource, Self::Error>;
}
pub trait Compliant {
    type Suggestions: Iterator;
    fn compliant(&self) -> Option<Self::Suggestions>;
}
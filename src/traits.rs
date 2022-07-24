pub trait Name { fn name(&self) -> &String; }
pub trait PropertyValue {
    type PropertyType;
    type ValueType;
    fn property(&self) -> &Self::PropertyType;
    fn value(&self) -> &Self::ValueType;
}
pub trait Parse<T: Sized> {
    type Error;
    fn parse(src: T) -> Result<Box<Self>, Self::Error>;
}
pub trait Resource: Sized {
    type Error;
    type Options;
    fn file(path: &std::path::Path, options: Self::Options) -> Result<Self, Self::Error>;
}
pub trait Asset: Resource + Sized {}
pub trait Document: Resource + Sized {
    fn src(src: &String, options: Self::Options) -> Result<Self, Self::Error>;
}
pub trait Builder {
    type Resource: Resource;
    type Error;
    fn build(&self) -> Result<Self::Resource, Self::Error>;
}

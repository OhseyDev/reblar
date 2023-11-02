pub trait Named {
    fn name() -> String;
}
pub trait PropertyValuePair {
    type PropertyType;
    type ValueType;
    fn property(&self) -> &Self::PropertyType;
    fn value(&self) -> &Self::ValueType;
}
pub trait Resource: Named + Sized {
    type Error;
    type Options;
    type Source;
    fn file(path: &std::path::Path, options: Self::Options) -> Result<Self, Self::Error>;
    fn src(src: &Self::Source, options: Self::Options) -> Result<Self, Self::Error>;
}
pub trait Builder {
    type Resource: Resource;
    type Error;
    fn build(&self) -> Result<Self::Resource, Self::Error>;
}

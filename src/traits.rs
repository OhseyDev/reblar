pub enum Source {
    File(Box<std::path::Path>),
    Memory(*mut u8)
}
pub trait Named {
    fn name() -> String;
}
pub trait PropertyValuePair {
    type PropertyType;
    type ValueType;
    fn property(&self) -> &Self::PropertyType;
    fn value(&self) -> &Self::ValueType;
}
pub trait Resource {
    type Data;
    fn data(&mut self) -> &mut Self::Data;
}
pub trait Builder {
    type Resource: Resource;
    type Error;
    type Source;
    fn build(&self, src: &Self::Source) -> Result<Self::Resource, Self::Error>;
}
pub trait FramedResource: Resource {
    fn dimensions(&self) -> (usize, usize);
    fn width(&self) -> usize;
    fn height(&self) -> usize;
}

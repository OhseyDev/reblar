pub trait Name { fn name(&self) -> &String; }
pub trait PropertyValue {
    type PropertyType;
    type ValueType;

    fn property(&self) -> &Self::PropertyType;
    fn value(&self) -> &Self::ValueType;
}

pub trait ParentChild {
    type ParentType;
    type ChildType;
    fn child(&self) -> &Self::ChildType;
    fn parent(&self) -> &Self::ParentType;
}

pub trait ParseCompile {
    type Error;
    fn parse(src: Vec<crate::lex::Token>) -> Result<Box<Self>, Self::Error>;
    fn compile(&self) -> Result<&'static str, Self::Error>;
}

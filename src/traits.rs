pub trait Name { fn name(&self) -> &'static str; }
pub trait PropertyValue {
    type PropertyType;
    type ValueType;

    fn property(&self) -> &Self::PropertyType;
    fn value(&self) -> &Self::ValueType;
}

pub trait ParentChild {
    fn child(&self) -> Option<&'static Self>;
    fn parent(&self) -> Option<&'static Self>;
}

pub trait ParseCompile {
    type Error;
    fn parse(src: crate::lex::LexerTokens) -> Result<Box<Self>, Self::Error>;
    fn compile(&self) -> Result<&'static str, Self::Error>;
}

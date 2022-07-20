
pub trait Resource: Sized {
    type Error;
    type Options;
    fn file(path: &std::path::Path, options: Self::Options) -> Result<Self, Self::Error>;
}

pub trait Builder {
    type Resource: Resource;
}

pub mod asset;
pub mod code;
pub mod doc;
pub mod traits;
pub mod lex;
pub mod preproc;
#[cfg(test)]
mod tests;

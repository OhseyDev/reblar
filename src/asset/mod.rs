pub mod audio;
pub mod image;
pub mod style;
pub mod video;

#[derive(Debug, Clone, PartialEq)]
pub enum AssetName {
    Empty,
    Str(String),
    FileName
}

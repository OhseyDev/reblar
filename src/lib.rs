pub mod media;
pub mod traits;

pub enum Source {
    File(std::path::PathBuf),
    Memory(Vec<u8>),
}

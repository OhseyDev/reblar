#[cfg(feature = "jpeg")]
pub mod jpeg;
#[cfg(feature = "png")]
pub mod png;

pub enum Error {
    Internal(),
    IO(std::io::Error),
    Format(String),
    #[cfg(feature = "png")]
    ParameterPNG(png::ParameterError),
    #[cfg(feature = "png")]
    LimitsExceededPNG(),
    #[cfg(feature = "jpeg")]
    UnsupportedJPEGFeature()
}

pub struct RawData();

pub struct Frame {
    data: Vec<u8>,
    dim: (u32, u32),
}

pub struct Image {
    frame: Frame,
}

impl<'a> crate::traits::Resource for Image {
    type Data = Frame;
    fn data(&mut self) -> &mut Self::Data {
        &mut self.frame
    }
}

impl<'a> crate::traits::FramedResource for Image {
    fn dimensions(&self) -> (u32, u32) {
        self.frame.dimensions()
    }
    fn width(&self) -> u32 {
        self.frame.width()
    }
    fn height(&self) -> u32 {
        self.frame.height()
    }
}

impl crate::traits::Resource for Frame {
    type Data = Vec<u8>;
    fn data(&mut self) -> &mut Self::Data {
        &mut self.data
    }
}

impl crate::traits::FramedResource for Frame {
    fn dimensions(&self) -> (u32, u32) {
        self.dim
    }
    fn width(&self) -> u32 {
        self.dim.0
    }
    fn height(&self) -> u32 {
        self.dim.1
    }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(val: std::io::Error) -> Self {
        Error::IO(val)
    }
}


#[cfg(feature = "png")]
pub mod png;
// #[cfg(feature = "jpeg")]
// pub mod jpeg;

pub enum Error {
    IO(std::io::Error),
    Format(),
    #[cfg(feature = "png")]
    ParameterPNG(png::ParameterError),
    #[cfg(feature = "png")]
    LimitsExceededPNG(),
}

pub struct RawData();

pub struct Frame {
    data: Vec<u8>,
    dim: (usize,usize),
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
    fn dimensions(&self) -> (usize, usize) { self.frame.dimensions() }
    fn width(&self) -> usize { self.frame.width() }
    fn height(&self) -> usize { self.frame.height() }
}

impl crate::traits::Resource for Frame {
    type Data = Vec<u8>;
    fn data(&mut self) -> &mut Self::Data { &mut self.data }
}

impl crate::traits::FramedResource for Frame {
    fn dimensions(&self) -> (usize, usize) { self.dim }
    fn width(&self) -> usize { self.dim.0 }
    fn height(&self) -> usize { self.dim.1 }
}

impl std::convert::From<std::io::Error> for Error {
    fn from(val: std::io::Error) -> Self {
        Error::IO(val)
    }
}


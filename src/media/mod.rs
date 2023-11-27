
// pub mod png;
// pub mod jpeg;
//

pub enum Error {
    IO(std::io::Error)
}

pub struct RawData(*mut u8);

pub struct Frame<'a> {
    data: RawData,
    dim: &'a (usize,usize),
}

pub struct Image<'a> {
    frame: Frame<'a>,
    dim: (usize,usize),
}

impl<'a> crate::traits::Resource for Image<'a> {
    type Data = Frame<'a>;
    fn data(&mut self) -> &mut Self::Data {
        &mut self.frame
    }
}

impl<'a> crate::traits::FramedResource for Image<'a> {
    fn dimensions(&self) -> (usize, usize) { self.dim }
    fn width(&self) -> usize { self.dim.0 }
    fn height(&self) -> usize { self.dim.1 }
}

impl<'a> crate::traits::Resource for Frame<'a> {
    type Data = RawData;
    fn data(&mut self) -> &mut Self::Data { &mut self.data }
}

impl<'a> crate::traits::FramedResource for Frame<'a> {
    fn dimensions(&self) -> (usize, usize) { (self.dim.0, self.dim.1) }
    fn width(&self) -> usize { self.dim.0 }
    fn height(&self) -> usize { self.dim.1 }
}


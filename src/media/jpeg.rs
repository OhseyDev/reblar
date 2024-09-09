extern crate jpeg_encoder as encoder;
extern crate jpeg_decoder as decoder;

use self::decoder::Decoder;

use crate::traits::{FramedResource, Resource};

use std::fs::File;
use std::io::{BufReader, BufWriter};

pub fn load(path: &std::path::Path) -> Result<super::Image, super::Error> {
    let mut decoder = Decoder::new(BufReader::new(File::open(path)?));
    let data = decoder.decode()?;
    let metadata = decoder.info().unwrap();
    todo!()
}

pub fn save(mut img: super::Image, path: &std::path::Path) -> Result<(), super::Error> {
    todo!()
}

impl std::convert::From<decoder::Error> for super::Error {
    fn from(e: decoder::Error) -> Self {
        match e {
            decoder::Error::Io(io) => Self::IO(io),
            decoder::Error::Format(s) => Self::Format(s),
            decoder::Error::Unsupported(_) => Self::UnsupportedJPEGFeature(),
            decoder::Error::Internal(_) => Self::Internal()
        }
    }
}
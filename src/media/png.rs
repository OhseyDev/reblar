extern crate png as ogpng;

use self::ogpng::{Decoder, Encoder, DecodingError, EncodingError};
use ::return_err;

use crate::traits::{FramedResource, Resource};

use std::fs::File;
use std::io::BufWriter;

pub use self::ogpng::ParameterError;

pub fn load(path: &std::path::Path) -> Result<super::Image, super::Error> {
    let file = return_err!(File::open(path));
    let decoder = Decoder::new(file);
    let mut reader = return_err!(decoder.read_info());
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = return_err!(reader.next_frame(&mut buf));
    buf.shrink_to_fit();
    let _l = info.buffer_size();
    let frame = super::Frame { data: buf, dim: (info.width, info.height) };
    Ok(super::Image { frame: frame})
}

pub fn save(mut img: super::Image, path: &std::path::Path) -> Result<(), super::Error> {
    let file = return_err!(File::create(path));
    let ref mut w = BufWriter::new(file);
    let mut encoder = Encoder::new(w, img.width(), img.height());
    let mut writer = return_err!(encoder.write_header());
    return_err!(writer.write_image_data(img.data().data()));
    Ok(())
}

impl std::convert::From<DecodingError> for super::Error {
    fn from(val: DecodingError) -> Self {
       return match val {
            DecodingError::IoError(e) => super::Error::IO(e),
            DecodingError::Format(..) => super::Error::Format(),
            DecodingError::Parameter(e) => super::Error::ParameterPNG(e),
            DecodingError::LimitsExceeded => super::Error::LimitsExceededPNG()
        } 
    }
}

impl std::convert::From<EncodingError> for super::Error {
    fn from(val: EncodingError) -> Self {
        return match val {
            EncodingError::IoError(e) => super::Error::IO(e),
            EncodingError::Format(..) => super::Error::Format(),
            EncodingError::Parameter(e) => super::Error::ParameterPNG(e),
            EncodingError::LimitsExceeded => super::Error::LimitsExceededPNG()
        }
    }
}

impl std::convert::From<self::ParameterError> for super::Error {
    fn from(val: self::ParameterError) -> Self {
        super::Error::ParameterPNG(val)
    }
}


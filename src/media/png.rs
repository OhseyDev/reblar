extern crate png as ogpng;

use self::ogpng::{Decoder, Encoder, DecodingError};
use ::return_err;

use std::fs::File;

pub use self::ogpng::ParameterError;


pub fn load(path: &std::path::Path) -> Result<super::Image, super::Error> {
    let file = return_err!(File::open(path));
    let decoder = Decoder::new(file);
    let mut reader = return_err!(decoder.read_info());
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = return_err!(reader.next_frame(&mut buf));
    buf.shrink_to_fit();
    let _l = info.buffer_size();
    let frame = super::Frame { data: buf, dim: (info.width as usize, info.height as usize) };
    Ok(super::Image { frame: frame})
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

impl std::convert::From<self::ParameterError> for super::Error {
    fn from(val: self::ParameterError) -> Self {
        super::Error::ParameterPNG(val)
    }
}



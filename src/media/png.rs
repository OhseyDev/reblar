extern crate png as ogpng;

use self::ogpng::{Decoder, DecodingError, Encoder, EncodingError};

use crate::traits::{FramedResource, Resource};

use std::fs::File;
use std::io::BufWriter;

pub use self::ogpng::ParameterError;

pub fn load(path: &std::path::Path) -> Result<super::Image, super::Error> {
    let file = File::open(path)?;
    let decoder = Decoder::new(file);
    let mut reader = decoder.read_info()?;
    let mut buf = vec![0; reader.output_buffer_size()];
    let info = reader.next_frame(&mut buf)?;
    buf.shrink_to_fit();
    let _l = info.buffer_size();
    let frame = super::Frame {
        data: buf,
        dim: (info.width, info.height),
    };
    Ok(super::Image { frame: frame })
}

pub fn save(mut img: super::Image, path: &std::path::Path) -> Result<(), super::Error> {
    let file = File::create(path)?;
    let ref mut w = BufWriter::new(file);
    let encoder = Encoder::new(w, img.width(), img.height());
    let mut writer = encoder.write_header()?;
    writer.write_image_data(img.data().data())?;
    Ok(())
}

impl std::convert::From<DecodingError> for super::Error {
    fn from(val: DecodingError) -> Self {
        return match val {
            DecodingError::IoError(e) => super::Error::IO(e),
            DecodingError::Format(f) => super::Error::Format(format!("{}", f)),
            DecodingError::Parameter(e) => super::Error::ParameterPNG(e),
            DecodingError::LimitsExceeded => super::Error::LimitsExceededPNG(),
        };
    }
}

impl std::convert::From<EncodingError> for super::Error {
    fn from(val: EncodingError) -> Self {
        return match val {
            EncodingError::IoError(e) => super::Error::IO(e),
            EncodingError::Format(f) => super::Error::Format(format!("{}", f)),
            EncodingError::Parameter(e) => super::Error::ParameterPNG(e),
            EncodingError::LimitsExceeded => super::Error::LimitsExceededPNG(),
        };
    }
}

impl std::convert::From<self::ParameterError> for super::Error {
    fn from(val: self::ParameterError) -> Self {
        super::Error::ParameterPNG(val)
    }
}

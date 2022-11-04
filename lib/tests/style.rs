use crate::{asset::style, traits::Resource};
use std::path::Path;

#[test]
fn parse_sass() {
    let _asset = style::Asset::file(Path::new("src/tests/style.sass"), style::Options::sassy()).expect("Error parsing SASS file");
}
#[test]
fn parse_scss() {
    let _asset = style::Asset::file(Path::new("src/tests/style.scss"), style::Options::default()).expect("Error parsing SCSS file");
}

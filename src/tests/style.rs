use crate::{asset::style, traits::Resource};
use std::path::Path;

#[test]
fn parse_sass() {
    let asset = style::Asset::file(Path::new("src/tests/styles/background.sass"), style::Options::sassy()).expect("Error parsing SASS file");
}
#[test]
fn parse_scss() {
    let asset = style::Asset::file(Path::new("src/tests/styles/background.scss"), style::Options::default()).expect("Error parsing SCSS file");
}

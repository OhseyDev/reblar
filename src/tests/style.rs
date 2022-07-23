use crate::{asset::style, traits::Resource};
use std::path::Path;

#[test]
fn parse_sass() {
    let _asset = style::Asset::file(Path::new("src/tests/style.sass"), true).expect("Error parsing SASS file");
}
#[test]
fn parse_scss() {
    let _asset = style::Asset::file(Path::new("src/tests/style.scss"), false).expect("Error parsing SCSS file");
}

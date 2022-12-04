extern crate derive_builder;
extern crate parse_display;

pub mod asset;
pub mod code;
pub mod doc;
pub mod traits;
pub mod lex;
pub mod preproc;
#[cfg(test)]
mod tests;

fn main() {
    println!("Hello World!");
}
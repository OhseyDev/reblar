use std::fmt::Debug;

use parse_display::Display;

pub mod bg;

#[derive(Display, Clone, Debug, PartialEq)]
pub enum Value<T: std::fmt::Display + Clone + Debug> {
    #[display("inherit")]
    Inherit,
    #[display("auto")]
    Auto,
    #[display("none")]
    None,
    #[display("{}")]
    Specified(T)
}

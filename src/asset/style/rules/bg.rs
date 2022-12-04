use std::{
    fmt::Debug
};

use parse_display::{Display, FromStr, ParseError};

use crate::traits::IsValid;

use super::super::{InnerParseOutput, ParseState, Value};

#[derive(Debug, Clone, PartialEq, Display, FromStr)]
pub enum Alpha {
    #[display("{0}%")]
    Percentage(f32),
}

#[derive(Debug, Clone, PartialEq, Display, FromStr)]
pub enum Color {
    #[display("{0}, {1}, {2}")]
    RGB(f64, f64, f64),
    #[display("{0}, {1}%, {2}% {3}")]
    RGBA(f64, f64, f64, Alpha),
    #[display("{0}, {1}%, {2}%")]
    HSL(Hue, Saturation, Lightness),
    #[display("{0}, {1}%, {2}%, {3}")]
    HSLA(Hue, Saturation, Lightness, Alpha),
    #[display("{0}")]
    Name(String),
    Transparent,
}
#[derive(Debug, Clone, PartialEq, Display, FromStr)]
pub enum Direction {
    #[display("left")]
    Left,
    #[display("right")]
    Right,
    #[display("bottom")]
    Bottom,
    #[display("top")]
    Top
}
#[derive(Debug, Clone, PartialEq, Display, FromStr)]
#[display("{x} {y}")]
pub struct BackgroundPosition {
    pub x: PositionValue,
    pub y: PositionValue,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ConicGradient {
    pub from_angle: Option<f64>,
    pub at_position: Option<PositionValue>,
    pub colors: Vec<Color>,
}
impl std::fmt::Display for ConicGradient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("conic-gradient(");
        let mut first = true;
        if self.from_angle.is_some() {
            f.write_str(self.from_angle.unwrap().to_string().as_str());
            first = false;
        }
        if self.at_position.is_some() {
            if !first {
                f.write_str(", ");
            }
            f.write_str(self.at_position.unwrap().to_string().as_str());
            first = false;
        }
        f.write_str("}")
    }
}
impl std::str::FromStr for ConicGradient {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !s.starts_with("conic-gradient(") {
            return Err(ParseError::with_message("expected keyword 'conic-gradient'"));
        }
        let mut list = s.replace("conic-gradient(", "").split(",");
        for item in list {
            match item {
                "" => {},
                _ => {}
            }
        }
        return Err(ParseError::with_message("unknown error"));
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct LinearGradient {
    pub directions: (Direction, Option<Direction>),
    pub colors: Vec<Color>,
}
impl std::fmt::Display for LinearGradient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (direction1, direction2) = &self.directions;
        let mut list = direction1.to_string();
        if direction2.is_some() {
            list = format!("{} {}", list, direction2.as_ref().unwrap());
        }
        for color in &self.colors {
            list = format!("{}, {}", list, color);
        }
        f.write_str(format!("linear-gradient({})", list).as_str())
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RadialGradientSize {
    FarthestCorner,
    ClosestSide,
    ClosestCorner,
    FarthestSide,
}
#[derive(Debug, Clone, PartialEq)]
pub struct RadialGradient {
    pub elipsis: bool,
    pub repeats: bool,
    pub position: PositionValue,
    pub size: RadialGradientSize,
    pub colors: Vec<Color>,
}
impl std::fmt::Display for RadialGradient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fn_name = {
            if self.repeats {
                "repeating-radial-gradient"
            } else {
                "radial-gradient"
            }
        };
        let mut list = String::new();
        for color in &self.colors {
            if list.is_empty() {
                list += color.to_string().as_str();
                continue;
            }
            list = format!("{}, {}", list, color);
        }
        f.write_str(format!("{}({})", fn_name, list).as_str())
    }
}
impl std::str::FromStr for RadialGradient {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        
    }
}
#[derive(Debug, Clone, PartialEq, Copy, Display, FromStr)]
pub enum PositionValue {
    #[display("left")] Left,
    #[display("right")] Right,
    #[display("center")] Center,
    #[display("top")] Top,
    #[display("bottom")] Bottom,
    #[display("{0}px")] Pixels(f64),
    #[display("{0}%")] Percentage(f64),
    #[display("{0}em")] Em(f64),
    #[display("{0}in")] Inch(f64),
    #[display("{0}pt")] Point(f64),
    #[display("{0}cm")] Centimetre(f64),
    #[display("{0}mm")] Millimetre(f64),
    #[display("{0}pc")] Pica(f64),
    #[display("{0}ex")] Ex(f64),
}
#[derive(Debug, Clone, PartialEq, Copy, Display, FromStr)]
#[display("{0}")]
pub struct Hue(u16);
impl IsValid for Hue {
    fn valid(&self) -> bool {
        self.0 <= 360
    }
}
#[derive(Debug, Clone, PartialEq, Copy, Display, FromStr)]
#[display("{val}")]
pub struct Saturation {
    pub val: f64,
}
impl IsValid for Saturation {
    fn valid(&self) -> bool {
        self.val <= 100.0
    }
}
#[derive(Debug, Clone, PartialEq, Copy, Display, FromStr)]
#[display("{val}")]
pub struct Lightness {
    pub val: f64,
}
impl IsValid for Lightness {
    fn valid(&self) -> bool {
        self.val <= 100.0
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct ColorHex {
    pub raw: u8,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackgroundAttachment {
    Scroll,
    Fixed,
    Local,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackgroundBlendMode {
    Normal,
    Multiply,
    Screen,
    Overlay,
    Darken,
    Lighten,
    ColorDodge,
    Saturation,
    Color,
    Luminosity,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackgroundClip {
    BorderBox,
    PaddingBox,
    ContentBox,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackgroundOrigin {
    BorderBox,
    PaddingBox,
    ContentBox,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackgroundRepeat {
    Repeat,
    RepeatX,
    RepeatY,
    NoRepeat,
    Space,
    Round,
}
#[derive(Debug, Clone, PartialEq, Display, FromStr)]
pub enum BackgroundSize {
    #[display("cover")]
    Cover,
    #[display("contain")]
    Contain,
    #[display("{0} {1}")]
    Specified(PositionValue, PositionValue)
}
#[derive(Debug, Clone, PartialEq, Display, FromStr)]
pub enum BackgroundImage {
    #[display("url({0})")]
    Url(String),
    #[display("{0}")]
    ConicGradient(ConicGradient),
    #[display("{0}")]
    RadialGradient(RadialGradient),
    #[display("{0}")]
    LinearGradient(LinearGradient),
}
#[derive(Debug, Clone, PartialEq)]
pub struct Background {
    pub color: Color,
    pub image: Vec<BackgroundImage>,
    pub position: BackgroundPosition,
    pub attachment: BackgroundAttachment,
    pub blend_mode: BackgroundBlendMode,
    pub clip: BackgroundClip,
    pub origin: BackgroundOrigin,
    pub repeat: BackgroundRepeat,
    pub size: BackgroundSize,
}
#[derive(Debug, Clone, PartialEq, Eq, Display, FromStr)]
pub enum BackgroundProperty {
    #[display("-color")]
    Color,
    #[display("-image")]
    Image,
    #[display("attachment")]
    Attachment,
    #[display("blend-mode")]
    BlendMode,
    #[display("clip")]
    Clip,
    #[display("origin")]
    Origin,
    #[display("repeat")]
    Repeat,
    #[display("size")]
    Size
}
#[derive(Debug, Clone, PartialEq, Display, FromStr)]
pub enum BackgroundValue {
    #[display("{0}")]
    Color(Color),
    #[display("{0}")]
    Image(BackgroundImage),
}
impl Into<Value> for BackgroundValue {
    fn into(self) -> Value {
        Value::Background(self)
    }
}
#[inline]
pub(crate) fn parse(
    state: &ParseState,
    property: &BackgroundProperty,
    token: &crate::lex::Token,
) -> InnerParseOutput {
    match property {
        BackgroundProperty::Attachment => attachment(state, token),
        BackgroundProperty::BlendMode => blend_mode(state, token),
        BackgroundProperty::Image => image(state, token),
        _ => root(state, token),
    }
}

#[inline]
fn root(state: &ParseState, token: &crate::lex::Token) -> InnerParseOutput {
    let new_state = state.to_owned();
    match token {
        _ => return (new_state, None),
    }
}

#[inline]
fn attachment(state: &ParseState, token: &crate::lex::Token) -> InnerParseOutput {
    let new_state = state.to_owned();
    match token {
        _ => return (new_state, None),
    }
}

#[inline]
fn blend_mode(state: &ParseState, token: &crate::lex::Token) -> InnerParseOutput {
    let new_state = state.to_owned();
    match token {
        _ => (new_state, None),
    }
}

#[inline]
fn image(state: &ParseState, token: &crate::lex::Token) -> InnerParseOutput {
    let new_state = state.to_owned();
    match token {
        _ => (new_state, None),
    }
}

use std::{
    fmt::{Debug, Display},
    num::{ParseFloatError, ParseIntError},
    str::FromStr,
};

use crate::traits::IsValid;

use super::{InnerParseOutput, ParseState, Value};

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    RGB(f64, f64, f64),
    RGBA(f64, f64, f64, f64),
    HSL(Hue, Saturation, Lightness),
    HSLA(Hue, Saturation, Lightness, f64),
    Name(String),
    Transparent,
}
impl Display for Color {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::HSL(h, s, l) => f.write_str(format!("hsl({}, {}%, {}%)", h, s, l).as_str()),
            Self::HSLA(h, s, l, a) => f.write_str(format!("hsla({}, {}%, {}%, {}%)", h, s, l, a).as_str()),
            Self::RGB(r, g, b) => f.write_str(format!("rgb({}, {}, {})", r, g, b).as_str()),
            Self::RGBA(r, g, b, a) => f.write_str(format!("rgba({}, {}, {}, {})", r, g, b, a).as_str()),
            Self::Name(s) => f.write_str(s),
            Self::Transparent => f.write_str(format!("transparent").as_str()),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Bottom,
    Top
}
impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => f.write_str("left"),
            Self::Right => f.write_str("right"),
            Self::Top => f.write_str("top"),
            Self::Bottom => f.write_str("bottom"),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct BackgroundPosition {
    pub x: PositionValue,
    pub y: PositionValue,
}
#[derive(Debug, Clone, PartialEq)]
pub struct ConicGradientValue {
    pub from_angle: Option<f64>,
    pub at_position: Option<PositionValue>,
    pub color: Color,
}
impl Display for ConicGradientValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut string = self.color.to_string();
        if self.from_angle.is_some() {
            string = format!("{} {}", string, self.from_angle.unwrap());
        }
        if self.at_position.is_some() {
            let (ty, valf) = self.at_position.unwrap();
            let mut val = String::new();
            match ty {
                PositionValueType::Em
                | PositionValueType::Inch
                | PositionValueType::Percentage
                | PositionValueType::Pixels
                | PositionValueType::Point
                | PositionValueType::Centimetre
                | PositionValueType::Millimetre => val = valf.to_string(),
                _ => {}
            }
            string = format!("{} {}{}", string, val, ty);
        }
        f.write_str(string.as_str())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct ConicGradient {
    pub repeats: bool,
    pub values: Vec<ConicGradientValue>,
}
impl Display for ConicGradient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fn_name = {
            if self.repeats {
                "repeating-conic-gradient"
            } else {
                "conic-gradient"
            }
        };
        let mut list = String::new();
        for value in &self.values {
            if list.is_empty() {
                list += value.to_string().as_str();
                continue;
            }
            list = format!("{}, {}", list, value.to_string());
        }
        f.write_str(format!("{}({})", fn_name, list).as_str())
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct LinearGradient {
    pub directions: (Direction, Option<Direction>),
    pub colors: Vec<Color>,
}
impl Display for LinearGradient {
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
impl Display for RadialGradient {
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
type PositionValue = (PositionValueType, f64);
#[derive(Debug, Clone, PartialEq, Copy)]
pub enum PositionValueType {
    Left,
    Right,
    Center,
    Top,
    Bottom,
    Pixels,
    Percentage,
    Em,
    Inch,
    Point,
    Centimetre,
    Millimetre,
    Pica,
    Ex,
}
impl Display for PositionValueType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Top => f.write_str("top"),
            Self::Bottom => f.write_str("bottom"),
            Self::Left => f.write_str("left"),
            Self::Right => f.write_str("right"),
            Self::Center => f.write_str("center"),
            Self::Percentage => f.write_str("%"),
            Self::Pixels => f.write_str("px"),
            Self::Centimetre => f.write_str("cm"),
            Self::Millimetre => f.write_str("mm"),
            Self::Inch => f.write_str("in"),
            Self::Point => f.write_str("pt"),
            Self::Em => f.write_str("em"),
            Self::Pica => f.write_str("pc"),
            Self::Ex => f.write_str("ex")
        }
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Hue {
    pub val: u16,
}
impl IsValid for Hue {
    fn valid(&self) -> bool {
        self.val <= 360
    }
}
impl FromStr for Hue {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = u16::from_str(s);
        if val.is_err() {
            return Err(val.err().unwrap());
        }
        Ok(Self { val: val.unwrap() })
    }
}
impl std::fmt::Display for Hue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.val.to_string().as_str())
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Saturation {
    pub val: f64,
}
impl IsValid for Saturation {
    fn valid(&self) -> bool {
        self.val <= 100.0
    }
}
impl FromStr for Saturation {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = f64::from_str(s);
        if val.is_err() {
            return Err(val.err().unwrap());
        }
        Ok(Self { val: val.unwrap() })
    }
}
impl std::fmt::Display for Saturation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.val.to_string().as_str())
    }
}
#[derive(Debug, Clone, PartialEq, Copy)]
pub struct Lightness {
    pub val: f64,
}
impl IsValid for Lightness {
    fn valid(&self) -> bool {
        self.val <= 100.0
    }
}
impl FromStr for Lightness {
    type Err = ParseFloatError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let val = f64::from_str(s);
        if val.is_err() {
            return Err(val.err().unwrap());
        }
        Ok(Self { val: val.unwrap() })
    }
}
impl std::fmt::Display for Lightness {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.val.to_string().as_str())
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
#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundSize {
    Auto,
    Cover,
    Contain,
    Percentage(f64),
    Length(f64, Option<f64>),
}
#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundImage {
    Url(String),
    None,
    ConicGradient(ConicGradient),
    RadialGradient(RadialGradient),
    LinearGradient(LinearGradient),
}
impl Display for BackgroundImage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Url(url) => f.write_str(format!("url({})", url).as_str()),
            Self::ConicGradient(gradient) => f.write_str(gradient.to_string().as_str()),
            Self::LinearGradient(gradient) => f.write_str(gradient.to_string().as_str()),
            Self::RadialGradient(gradient) => f.write_str(gradient.to_string().as_str()),
            Self::None => f.write_str("none"),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundColor {
    Color(Color),
    CurrentColor,
}
impl ToString for BackgroundColor {
    fn to_string(&self) -> String {
        match self {
            Self::Color(c) => c.to_string(),
            Self::CurrentColor => String::from(""),
        }
    }
}
#[derive(Debug, Clone, PartialEq)]
pub struct Background {
    pub color: BackgroundColor,
    pub image: Vec<BackgroundImage>,
    pub position: BackgroundPosition,
    pub attachment: BackgroundAttachment,
    pub blend_mode: BackgroundBlendMode,
    pub clip: BackgroundClip,
    pub origin: BackgroundOrigin,
    pub repeat: BackgroundRepeat,
    pub size: BackgroundSize,
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BackgroundProperty {
    Color,
    Image,
    Attachment,
    BlendMode,
    Clip,
    Origin,
    Repeat,
    Size,
}
#[derive(Debug, Clone, PartialEq)]
pub enum BackgroundValue {
    Color(BackgroundColor),
    Image(BackgroundImage),
}
impl Into<Value> for BackgroundValue {
    fn into(self) -> Value {
        Value::Background(self)
    }
}
impl ToString for BackgroundValue {
    fn to_string(&self) -> String {
        match &self {
            Self::Color(c) => c.to_string(),
            Self::Image(i) => i.to_string(),
        }
    }
}

#[inline]
pub(crate) fn parse(
    state: &ParseState,
    property: &Option<BackgroundProperty>,
    token: &crate::lex::Token,
) -> InnerParseOutput {
    match property {
        &Some(BackgroundProperty::Attachment) => attachment(state, token),
        &Some(BackgroundProperty::BlendMode) => blend_mode(state, token),
        &Some(BackgroundProperty::Image) => image(state, token),
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

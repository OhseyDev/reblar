use super::{ParseState, InnerParseOutput, Value};

#[derive(Debug,Clone,PartialEq)]
pub enum Color {
    RGB(f64,f64,f64),RGBA(f64,f64,f64,f64),
    HSL(f64,f64,f64),HSLA(f64,f64,f64,f64),
    Name(String),Transparent
}
#[derive(Debug,Clone,PartialEq)]
pub enum Direction { Left,Right,Bottom,Top,None }
#[derive(Debug,Clone,PartialEq)]
pub struct BackgroundPosition {
    pub x: PositionValue,
    pub y: PositionValue
}
#[derive(Debug,Clone,PartialEq)]
pub struct ConicGradientValue {
    pub from_angle: Option<f64>,
    pub at_position: Option<PositionValue>,
    pub color: Color
}
#[derive(Debug,Clone,PartialEq)]
pub struct ConicGradient { pub repeats: bool, pub values: Vec<ConicGradientValue> }
#[derive(Debug,Clone,PartialEq)]
pub struct LinearGradient {
    pub directions: [Option<Direction>; 2],
    pub colors: Vec<Color>
}
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum RadialGradientSize { FarthestCorner,ClosestSide,ClosestCorner,FarthestSide }
#[derive(Debug,Clone,PartialEq)]
pub struct RadialGradientValue {
    pub size: RadialGradientSize,
    pub position: PositionValue
}
#[derive(Debug,Clone,PartialEq)]
pub struct RadialGradient {
    pub elipsis: bool,
    pub repeats: bool,
    pub position: PositionValue,
    pub values: Vec<ConicGradientValue>
}
#[derive(Debug,Clone,PartialEq,Copy)]
pub enum PositionValue {
    Left,Right,Center,Top,Bottom,
    Percentage(f64),Pixels(i64)
}
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum BackgroundAttachment { Scroll,Fixed,Local }
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum BackgroundBlendMode { Normal,Multiply,Screen,Overlay,Darken,Lighten,ColorDodge,Saturation,Color,Luminosity }
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum BackgroundClip { BorderBox,PaddingBox,ContentBox }
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum BackgroundOrigin { BorderBox,PaddingBox,ContentBox }
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum BackgroundRepeat { Repeat,RepeatX,RepeatY,NoRepeat,Space,Round }
#[derive(Debug,Clone,PartialEq)]
pub enum BackgroundSize { Auto,Cover,Contain,Percentage(f64),Length(f64,Option<f64>) }
#[derive(Debug,Clone,PartialEq)]
pub enum BackgroundImage {
    Url(String),None,
    ConicGradient(ConicGradient),
    LinearGradient(LinearGradient),
}
#[derive(Debug,Clone,PartialEq)]
pub enum BackgroundColor { Color(Color),CurrentColor }
#[derive(Debug,Clone,PartialEq)]
pub struct Background {
    pub color: BackgroundColor,
    pub image: Vec<BackgroundImage>,
    pub position: BackgroundPosition,
    pub attachment: BackgroundAttachment,
    pub blend_mode: BackgroundBlendMode,
    pub clip: BackgroundClip,
    pub origin: BackgroundOrigin,
    pub repeat: BackgroundRepeat,
    pub size: BackgroundSize
}
#[derive(Debug,Clone,PartialEq,Eq)]
pub enum BackgroundProperty { Color,Image,Attachment,BlendMode,Clip,Origin,Repeat,Size }
#[derive(Debug,Clone,PartialEq)]
pub enum BackgroundValue { Color(BackgroundColor),Image(BackgroundImage), }
impl Into<Value> for BackgroundValue { fn into(self) -> Value { Value::Background(self) } }

fn root(state: &ParseState, token: &crate::lex::Token) -> InnerParseOutput {
    let new_state = state.to_owned();
    match token {
        _ => return (new_state, None)
    }
}

#[inline]
pub(crate) fn parse(state: &ParseState, property: &Option<BackgroundProperty>, token: &crate::lex::Token) -> InnerParseOutput {
    match property {
        &Some(BackgroundProperty::Image) => image(state, token),
        _ => root(state, token)
    }
}
#[inline]
fn image(state: &ParseState, token: &crate::lex::Token) -> InnerParseOutput {
    let new_state = state.to_owned();
    match token {
        _ => (new_state, None)
    }
}
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
pub struct ConicGradientValue {
    pub from_angle: Option<f64>,
    pub at_position: Option<PositionValue>,
    pub color: Color
}
pub struct ConicGradient { pub repeats: bool, pub values: Vec<ConicGradientValue> }
pub struct LinearGradient {
    pub directions: [Option<Direction>; 2],
    pub colors: Vec<Color>
}
pub enum RadialGradientSize { FarthestCorner,ClosestSide,ClosestCorner,FarthestSide }
pub struct RadialGradientValue {
    pub size: RadialGradientSize,
    pub position: PositionValue
}
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
pub enum BackgroundAttachment { Scroll,Fixed,Local }
pub enum BackgroundBlendMode { Normal,Multiply,Screen,Overlay,Darken,Lighten,ColorDodge,Saturation,Color,Luminosity }
pub enum BackgroundClip { BorderBox,PaddingBox,ContentBox }
pub enum BackgroundOrigin { BorderBox,PaddingBox,ContentBox }
pub enum BackgroundRepeat { Repeat, RepeatX, RepeatY, NoRepeat, Space, Round }
pub enum BackgroundSize { Auto, Cover, Contain, Percentage(f64), Length(f64, Option<f64>) }
pub enum BackgroundImage {
    Url(String),None,
    ConicGradient(ConicGradient),
    LinearGradient(LinearGradient),
}
#[derive(Debug,Clone)]
pub enum BackgroundColor { Color(Color),CurrentColor }
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
#[derive(Debug,Clone)]
pub enum BackgroundProperty { Color,Image,Attachment,BlendMode,Clip,Origin,Repeat,Size }

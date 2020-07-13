#[derive(Clone, Copy, Debug)]
pub struct Glyph{
    pub coords: Option<i32,i32>,
    pub symbol: Option<char>,
    pub background: Color,
    pub foreground: Color,
}
impl Glyph{
    pub fn new()->Glyph{
        Glyph{
            coords: None,
            symbol: None,
            background: Color::black,
            foreground: Color::white,
        }
    }
    pub fn from_char(symbol: char) -> Self{
        Glyph{
            coords: None,
            symbol: Some(symbol),
            background: Color::black,
            foreground: Color::white,
        }
    }
    pub fn color_fg(&mut self, color: Color) ->&mut self{
        self.foreground = color;
        self
    }
    pub fn color_bg(&mut self, color: Color) ->&mut self{
        self.background = color;
        self
    }
}
#[derive(Clone, Copy, Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}
impl Color {
    pub fn rgb(r: f32, g: f32, b: f32)->Color{
        Color{
            r,
            g,
            b,
            a: 1.0,
        }
    }
    pub const black: Color = Color::rgb(0.0,0.0,0.0); 
    pub const white: Color = Color::rgb(1.0,1.0,1.0); 
    pub const red: Color = Color::rgb(1.0,0.0,0.0); 
    pub const green: Color = Color::rgb(0.0,1.0,0.0); 
    pub const blue: Color = Color::rgb(0.0,0.0,1.0); 
}
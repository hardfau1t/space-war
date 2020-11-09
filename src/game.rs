use crate::types::Display;
use embedded_graphics::{
    // prelude::*,
    pixelcolor::BinaryColor,
    primitives::Rectangle,
    style::{
        PrimitiveStyle,
        Styled,
    },
    // pixelcolor::PixelColor,
    drawable::Drawable,
};

#[derive(Debug, Copy, Clone)]
pub struct Object {
    x:u8,
    y:u8,
    style:Styled<Rectangle, PrimitiveStyle<BinaryColor>>,
}

impl Object{
    pub fn new(x:u8, y:u8, style:Styled<Rectangle, PrimitiveStyle<BinaryColor>>)->Self{
        Self{ x, y, style}
    }
    pub fn draw(&self, disp:&mut Display){
        self.style.draw(disp).unwrap();
    }
}

impl Position for Object {
    fn get_pos(&self)->(u8, u8){
        (self.x, self.y)
    }

    fn set_pos(&mut self, x:u8, y:u8){
        self.x = x;
        self.y = y;
    }
}

pub trait Position{
    fn get_pos(&self)->(u8,u8);
    fn set_pos(&mut self, x:u8, y:u8);
}


// import Section 
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

// Structs definitions
#[derive(Debug, Copy, Clone)]
pub struct Object {
    x:u8,
    y:u8,
    friendly:bool,
    vel_x:i8,
    vel_y:i8,
    style:Styled<Rectangle, PrimitiveStyle<BinaryColor>>,
}

// Traits Definitions Section
pub trait Position{
    fn get_pos(&self)->(u8,u8);
    fn set_pos(&mut self, x:u8, y:u8);
}

pub trait Shooter{
    // Creates a bullet which is also an Object
    fn shoot(&self)->Self;
}
// implementation Section
impl Object{
    pub fn new(x:u8, y:u8, friendly:bool, style:Styled<Rectangle, PrimitiveStyle<BinaryColor>>)->Self{
        Self{ x, y, friendly, vel_x:0, vel_y:0, style}
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

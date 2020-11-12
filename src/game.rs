// import Section 
use crate::{
    types::Display,
    objects::Sprite,
};
use embedded_graphics::{
    prelude::*,
    pixelcolor::BinaryColor,
    image::{Image, ImageRaw},
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
    raw_image: ImageRaw<'static, BinaryColor>,
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
    pub fn new(x:u8, y:u8, friendly:bool, sprite: &Sprite)->Self{
        let raw_image = ImageRaw::new(sprite.data, sprite.width as u32, sprite.height as u32);
        Self{ x, y, friendly, vel_x:0, vel_y:0, raw_image}
    }
    pub fn draw(&self, disp:&mut Display){
        let image = Image::new( &self.raw_image, Point::new(self.x as i32, self.y as i32) );
        image.draw(disp).unwrap();
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

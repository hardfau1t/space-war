// import Section 
use crate::{
    types::Display,
    objects::{Sprite, BULLET},
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
    x:i8,
    y:i8,
    friendly:bool,
    pub vel_x:i8,
    pub vel_y:i8,
    sprite_width:u8,
    sprite_height:u8,
    raw_image: ImageRaw<'static, BinaryColor>,
}

// Traits Definitions Section
pub trait Position{
    fn get_pos(&self)->(i8,i8);
    fn set_pos(&mut self, x:i8, y:i8);
}

pub trait Shooter{
    // Creates a bullet which is also an Object
    fn shoot(&self)->Self;
}
// implementation Section
impl Object{
    pub fn new(x:i8, y:i8, friendly:bool, sprite: &Sprite)->Self{
        let raw_image = ImageRaw::new(sprite.data, sprite.width as u32, sprite.height as u32);
        Self{ x, y, friendly, vel_x:0, vel_y:0,sprite_width: sprite.width, sprite_height:sprite.height, raw_image}
    }
    pub fn draw(&self, disp:&mut Display){
        let image = Image::new( &self.raw_image, Point::new(self.x as i32, self.y as i32) );
        image.draw(disp).unwrap();
    }
}

impl Position for Object {
    fn get_pos(&self)->(i8, i8){
        (self.x, self.y)
    }

    fn set_pos(&mut self, x:i8, y:i8){
        self.x = x;
        self.y = y;
    }
}

impl Shooter for Object{
    fn shoot(&self)->Self{
        let raw_image = ImageRaw::new(BULLET.data, BULLET.width as u32, BULLET.height as u32);
        Self{
            x:self.x + self.sprite_width as i8/2 - BULLET.width as i8/2,
            // if object is friendly then y = y - bullet height else y = y+bullet height;
            y: self.y - ((self.friendly as i8)*2 - 1)*BULLET.height as i8, // workaround for branchless if
            friendly: self.friendly,
            sprite_height: BULLET.height,
            sprite_width : BULLET.width,
            vel_x: 0,                   // velocity in x is 0 for every bullet
            vel_y: (1 - (self.friendly as i32)*2 ) as i8, // if friendly then vel_y is -ve
            raw_image
        }
    }
}

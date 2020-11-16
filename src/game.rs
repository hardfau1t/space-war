// import Section 
use crate::{
    types::Display,
    objects::{Sprite, BULLET_SPRITE},
};
use embedded_graphics::{
    prelude::*,
    pixelcolor::BinaryColor,
    image::{Image, ImageRaw},
    drawable::Drawable,
};

// Structs definitions
#[derive(Debug, Copy, Clone)]
pub struct Player {
    x:i8,
    y:i8,
    pub vel_x:i8,
    pub vel_y:i8,
    sprite_width:u8,
    sprite_height:u8,
    raw_image: ImageRaw<'static, BinaryColor>,
}

#[derive(Debug, Copy, Clone)]
pub struct Enemy {
    x:i8,
    y:i8,
    pub vel_x:i8,
    pub vel_y:i8,
    sprite_width:u8,
    sprite_height:u8,
    raw_image: ImageRaw<'static, BinaryColor>,
}

#[derive(Debug, Copy, Clone)]
pub struct Bullet {
    x:i8,
    y:i8,
    friendly:bool,
    pub vel_y:i8,       // no need of x vel, they will always move in straight line
    sprite_width:u8,
    sprite_height:u8,
    raw_image: ImageRaw<'static, BinaryColor>,
}

#[derive(Debug, Copy, Clone)]
pub struct Asteroids {
    x:i8,
    y:i8,
    pub vel_x:i8,
    pub vel_y:i8,
    sprite_width:u8,
    sprite_height:u8,
    raw_image: ImageRaw<'static, BinaryColor>,
}
// Object struct is to group all objects like player bullets, enemy

pub trait Object{
    fn get_pos(&self)->(i8,i8);
    fn set_pos(&mut self, x:i8, y:i8);
    fn update(&mut self);
    fn draw(&self, disp:&mut Display);
}

pub trait Shooter{
    // Creates a bullet which is also an Player
    fn shoot(&self)->Bullet;
}
// implementation Section
impl Player{
    pub fn new(x:i8, y:i8, sprite: &Sprite)->Self{
        let raw_image = ImageRaw::new(sprite.data, sprite.width as u32, sprite.height as u32);
        Self{ x, y, vel_x:0, vel_y:0,sprite_width: sprite.width, sprite_height:sprite.height, raw_image}
    }
}

impl Enemy{
    pub fn new(x:i8, y:i8, sprite: &Sprite)->Self{
        let raw_image = ImageRaw::new(sprite.data, sprite.width as u32, sprite.height as u32);
        Self{ x, y, vel_x:0, vel_y:0,sprite_width: sprite.width, sprite_height:sprite.height, raw_image}
    }
}

// no new function for bullet. 
// it should be created using shoot

impl Asteroids{
    pub fn new(x:i8, y:i8, sprite: &Sprite)->Self{
        let raw_image = ImageRaw::new(sprite.data, sprite.width as u32, sprite.height as u32);
        Self{ x, y, vel_x:0, vel_y:0,sprite_width: sprite.width, sprite_height:sprite.height, raw_image}
    }
}

// these implementations are later changed into macros, currently for simplicity it is implimented
// for every object
impl Object for Player {
    fn get_pos(&self)->(i8, i8){
        (self.x, self.y)
    }

    fn set_pos(&mut self, x:i8, y:i8){
        self.x = x;
        self.y = y;
    }
    fn draw(&self, disp:&mut Display) {
        let image = Image::new( &self.raw_image, Point::new(self.x as i32, self.y as i32) );
        image.draw(disp).unwrap();
    }

    fn update(&mut self) {
        self.set_pos(self.get_pos().0 +self.vel_x, self.get_pos().1 + self.vel_y);
    }
}

impl Object for Enemy {
    fn get_pos(&self)->(i8, i8){
        (self.x, self.y)
    }

    fn set_pos(&mut self, x:i8, y:i8){
        self.x = x;
        self.y = y;
    }
    fn draw(&self, disp:&mut Display) {
        let image = Image::new( &self.raw_image, Point::new(self.x as i32, self.y as i32) );
        image.draw(disp).unwrap();
    }

    fn update(&mut self) {
        self.set_pos(self.get_pos().0 +self.vel_x, self.get_pos().1 + self.vel_y);
    }
}

impl Object for Bullet {
    fn get_pos(&self)->(i8, i8){
        (self.x, self.y)
    }

    fn set_pos(&mut self, x:i8, y:i8){
        self.x = x;
        self.y = y;
    }
    fn draw(&self, disp:&mut Display) {
        let image = Image::new( &self.raw_image, Point::new(self.x as i32, self.y as i32) );
        image.draw(disp).unwrap();
    }

    fn update(&mut self) {
        self.set_pos(self.x, self.get_pos().1 + self.vel_y);
    }
}

impl Object for Asteroids {
    fn get_pos(&self)->(i8, i8){
        (self.x, self.y)
    }

    fn set_pos(&mut self, x:i8, y:i8){
        self.x = x;
        self.y = y;
    }
    fn draw(&self, disp:&mut Display) {
        let image = Image::new( &self.raw_image, Point::new(self.x as i32, self.y as i32) );
        image.draw(disp).unwrap();
    }

    fn update(&mut self) {
        self.set_pos(self.get_pos().0 +self.vel_x, self.get_pos().1 + self.vel_y);
    }
}


impl Shooter for Player{
    fn shoot(&self)->Bullet{
        let raw_image = ImageRaw::new(BULLET_SPRITE.data, BULLET_SPRITE.width as u32, BULLET_SPRITE.height as u32);
        Bullet{
            x:self.x + self.sprite_width as i8/2 - BULLET_SPRITE.width as i8/2,
            // if object is friendly then y = y - bullet height else y = y+bullet height;
            y: self.y - BULLET_SPRITE.height as i8, 
            friendly: true,
            sprite_height: BULLET_SPRITE.height,
            sprite_width : BULLET_SPRITE.width,
            vel_y: -1, // if friendly then vel_y is -ve
            raw_image
        }
    }
}

impl Shooter for Enemy{
    fn shoot(&self)->Bullet{
        let raw_image = ImageRaw::new(BULLET_SPRITE.data, BULLET_SPRITE.width as u32, BULLET_SPRITE.height as u32);
        Bullet{
            x:self.x + self.sprite_width as i8/2 - BULLET_SPRITE.width as i8/2,
            // if object is friendly then y = y - bullet height else y = y+bullet height;
            y: self.y + BULLET_SPRITE.height as i8, 
            friendly: false,
            sprite_height: BULLET_SPRITE.height,
            sprite_width : BULLET_SPRITE.width,
            vel_y: 1, // if friendly then vel_y is -ve
            raw_image
        }
    }
}


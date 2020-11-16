// import Section 
use crate::{
    types::{Left, Right, Display},
    objects::*,
};
use embedded_graphics::{
    prelude::*,
    pixelcolor::BinaryColor,
    image::{Image, ImageRaw},
    drawable::Drawable,
};

use stm32f7xx_hal::prelude::*;

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

    /// check if that object crosses the boundary
    fn boundary_check(&mut self);
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
    pub fn mov(&mut self, dir:&(Left, Right)){
        // check if left button is pressed,
        if let Ok(left) = dir.0.is_low(){
            if left{
                self.vel_x = -1;
                // when left is pressed but right is not
                // then velocity will be 0. to bypass that we return early
                return;
            } else {
                self.vel_x = 0;
            }
        }
        if let Ok(right) = dir.1.is_low(){
            if right{
                self.vel_x = 1;
            } else {
                self.vel_x = 0;
            }
        }
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
    fn boundary_check(&mut self){
        // check on both right upper corner and left lower corner if anyone of them crosses the
        // boundary then take an action on them
        
        // since player moves only in x axis, checking on x axis is sufficient
        // SAFETY: pos in i16, to avoid boundary overflow bugs 
        // extra 1 pixel removed for the border
        //
        // we are taking new position, if not player will get stuck on hitting border
        let new_pos:i16  = (self.x + self.vel_x) as i16;
        if new_pos <= 1  || new_pos + self.sprite_width as i16 >= DISPLAY_WIDTH as i16 - 2{
            self.vel_x = 0;
        }
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
    fn boundary_check(&mut self){
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
    fn boundary_check(&mut self){
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
    fn boundary_check(&mut self){
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


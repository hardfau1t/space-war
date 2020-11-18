// import Section 
#![derive(Debug)]
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

use heapless::{
    Vec,
    consts::*,
};

use stm32f7xx_hal::prelude::*;
use defmt::*;

// Structs definitions
pub struct Player {
    x:i16,
    y:i16,
    vel_x:i16,
    vel_y:i16,
    active: bool,
    bullets:Vec<Bullet, U20>,
    max_shots:i8,
    raw_image: ImageRaw<'static, BinaryColor>,
}

pub struct Enemy {
    x:i16,
    y:i16,
    // number bullets are created by enemy and number of active
    raw_image: ImageRaw<'static, BinaryColor>,
    active:bool,
    bullet:Option<Bullet>,
}

pub struct Bullet {
    x:i16,
    y:i16,
    friendly:bool,
    vel_x:i8,       // no need of x vel, they will always move in straight line
    vel_y:i8,       // no need of x vel, they will always move in straight line
    raw_image: ImageRaw<'static, BinaryColor>,
    active: bool,
}

pub struct Asteroid {
    x:i16,
    y:i16,
    vel_x:i8,
    vel_y:i8,
    raw_image: ImageRaw<'static, BinaryColor>,
    active:bool,
}

pub struct Sprite{
    data:&'static [u8],
    width:u8,
    height:u8,
}

pub struct Screen{
    width:u8,
    height:u8,
}
pub struct GamePool{
    bullets:Vec<Bullet, U100>
}

/// this is used for indicating boundary condition
pub enum Boundary{
    Fine,
    Breach,
}
// Object struct is to group all objects like player bullets, enemy

pub trait Object{
    fn update(&mut self);
    /// if objects is it will return false;
    fn is_active(&self)->bool;
    /// drops the object
    fn die(self, pool:&mut GamePool);
}

/// Creates a bullet which is also an Player
pub trait Shooter{
    fn shoot(&mut self);
}

/// enables movement for an object
pub trait Movable{
    fn set_pos(&mut self, x:i16, y:i16);
    fn get_pos(&self)->(i16, i16);
}

/// Boundary cross conditions are checked
pub trait HasBoundary{
    fn boundary_check(&self)->Boundary;
    fn breach_action(&mut self);
}

pub trait CanDraw{
    fn draw(&self, disp:Display);
}
// implementation Section
impl Player{
    pub fn new(x:i16, y:i16, sprite: &Sprite)->Self{
        let raw_image = ImageRaw::new(sprite.data, sprite.width as u32, sprite.height as u32);
        let bullets = Vec::new();
        Self{ x, y, vel_x:0, vel_y:0, raw_image,
            active:true, max_shots:10, bullets
        }
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
    pub fn new(x:i16, y:i16, sprite: &Sprite)->Self{
        let raw_image = ImageRaw::new(sprite.data, sprite.width as u32, sprite.height as u32);
        Self{ x, y, raw_image, active:true, bullet: None, }
    }
}

impl Bullet{
// no new function for bullet. 
// it should be created using shoot
}

impl Asteroid{
    pub fn new(x:i16, y:i16, sprite: &Sprite)->Self{
        let raw_image = ImageRaw::new(sprite.data, sprite.width as u32, sprite.height as u32);
        Self{ x, y, vel_x:-1, vel_y:1, raw_image, active:true}
    }
    pub fn kill(&mut self){
        self.active = false;
    }
}

impl Sprite{
    pub fn height(&self)->u8{
        self.height
    }
    pub fn width(&self)->u8{
        self.width
    }
}

// these implementations are later changed into macros, currently for simplicity it is implimented
// for every object
impl Object for Player {

    // fn draw(&self, disp:&mut Display) {
    //     let image = Image::new( &self.raw_image, Point::new(self.x as i32, self.y as i32) );
    //     image.draw(disp).unwrap();
    // }

    fn update(&mut self) {
        self.x += self.vel_x;
        self.y += self.vel_y;
    }

    fn is_active(&self) ->bool {
        self.active
    }
    fn die(self, _:&mut GamePool){
    }


    // fn boundary_check(&mut self){
    //     // check on both right upper corner and left lower corner if anyone of them crosses the
    //     // boundary then take an action on them
    //
    //     // since player moves only in x axis, checking on x axis is sufficient
    //     // SAFETY: pos in i16, to avoid boundary overflow bugs
    //     // extra 1 pixel removed for the border
    //     //
    //     // we are taking new position, if not player will get stuck on hitting border
    //     let new_pos:i16  = (self.x + self.vel_x) as i16;
    //     if new_pos <= 1  || new_pos + self.sprite_width as i16 >= DISPLAY_WIDTH as i16 - 2{
    //         self.vel_x = 0;
    //     }
    // }
}

impl Object for Enemy {
    // fn draw(&self, disp:&mut Display) {
    //     let image = Image::new( &self.raw_image, Point::new(self.x as i32, self.y as i32) );
    //     image.draw(disp).unwrap();
    // }

    fn update(&mut self) {
    }
    fn is_active(&self) ->bool {
        self.active
    }
    fn die(self, pool:&mut GamePool){
        if let Some(bullet) = self.bullet{
            pool.bullets.push(bullet);
        }
    }
}

impl Object for Bullet {
    // fn draw(&self, disp:&mut Display) {
    //     let image = Image::new( &self.raw_image, Point::new(self.x as i32, self.y as i32) );
    //     image.draw(disp).unwrap();
    // }

    fn update(&mut self) {
        self.x += self.vel_x as i16;
        self.y += self.vel_y as i16;
    }
    fn is_active(&self) ->bool {
        self.active
    }

    fn die(self, pool:&mut GamePool) {}
    // fn boundary_check(&mut self){
    //     // check for players boundary check
    //     let new_pos:i16  = (self.y + self.vel_y) as i16;
    //     if self.friendly {
    //         if new_pos < 1 {
    //             self.active = false;
    //         }
    //     } else {
    //         if new_pos > (DISPLAY_HEIGHT - self.sprite_height - 2) as i16{
    //             self.active = false
    //         }
    //     }
    // }
}

impl Object for Asteroid {
    // fn draw(&self, disp:&mut Display) {
    //     let image = Image::new( &self.raw_image, Point::new(self.x as i32, self.y as i32) );
    //     image.draw(disp).unwrap();
    // }

    fn update(&mut self) {
        self.x += self.vel_x as i16;
        self.y += self.vel_y as i16;
    }
    fn is_active(&self) ->bool {
        self.active
    }

    fn die(self, pool:&mut GamePool) {}
    // fn boundary_check(&mut self){
    //     // check for players boundary check
    //     let new_pos:i16  = (self.x + self.vel_x) as i16;
    //     if new_pos <= 1  || new_pos + self.sprite_width as i16 >= DISPLAY_WIDTH as i16 - 2{
    //         self.vel_x = -self.vel_x ;
    //     }
    //     if self.y as i16 + self.vel_y as i16 + self.sprite_height as i16 >= (DISPLAY_HEIGHT as i16 ){
    //         self.active = false;
    //     }
    // }
}


impl Shooter for Player{
    fn shoot(&mut self){
        let raw_image = ImageRaw::new(BULLET_SPRITE.data, BULLET_SPRITE.width as u32, BULLET_SPRITE.height as u32);
        if self.bullets.len() < self.max_shots as usize{
            self.bullets.push(Bullet{
                x:self.x + self.raw_image.width() as i16/2 - BULLET_SPRITE.width as i16/2,
                // if object is friendly then y = y - bullet height else y = y+bullet height;
                y: self.y - BULLET_SPRITE.height as i16, 
                friendly: true,
                vel_y: -3, // if friendly then vel_y is -ve
                vel_x:0,
                raw_image,
                active:true,
            });
        }
    }
}

impl Shooter for Enemy{
    fn shoot(&mut self){
        let raw_image = ImageRaw::new(BULLET_SPRITE.data, BULLET_SPRITE.width as u32, BULLET_SPRITE.height as u32);
        if self.bullet.is_none(){
            self.bullet = Some(Bullet{
                x:self.x + self.raw_image.width() as i16/2 - BULLET_SPRITE.width as i16/2,
                // if object is friendly then y = y - bullet height else y = y+bullet height;
                y: self.y + BULLET_SPRITE.height as i16, 
                friendly: false,
                vel_y: 2, // if friendly then vel_y is -ve
                vel_x:0,
                raw_image,
                active:true,
            });
        } 
    }
}

impl Movable for Player{
    fn get_pos(&self) ->(i16, i16) {
        (self.x, self.y)
    }
    fn set_pos(&mut self, x:i16, y:i16) {
        self.x = x;
        self.y = y;
    }
}
// although enemy cant move, it can be replaced
impl Movable for Enemy{
    fn get_pos(&self) ->(i16, i16) {
        (self.x, self.y)
    }
    fn set_pos(&mut self, x:i16, y:i16) {
        self.x = x;
        self.y = y;
    }
}
impl Movable for Bullet{
    fn get_pos(&self) ->(i16, i16) {
        (self.x, self.y)
    }
    fn set_pos(&mut self, x:i16, y:i16) {
        self.x = x;
        self.y = y;
    }
}
impl Movable for Asteroid{
    fn get_pos(&self) ->(i16, i16) {
        (self.x, self.y)
    }
    fn set_pos(&mut self, x:i16, y:i16) {
        self.x = x;
        self.y = y;
    }
}

impl HasBoundary for Player{
    fn boundary_check(&self) ->Boundary {
        todo!()
    }
    fn breach_action(&mut self) {
    }
}
impl HasBoundary for Bullet{
    fn boundary_check(&self) ->Boundary {
        todo!()
    }
    fn breach_action(&mut self) {
    }
}
impl HasBoundary for Asteroid{
    fn boundary_check(&self) ->Boundary {
        todo!()
    }
    fn breach_action(&mut self) {
    }
}

impl CanDraw for Player{
    fn draw(&self, disp:Display) {
    }
}
impl CanDraw for Bullet{
    fn draw(&self, disp:Display) {
    }
}
impl CanDraw for Enemy{
    fn draw(&self, disp:Display) {
    }
}
impl CanDraw for Asteroid{
    fn draw(&self, disp:Display) {
    }
}

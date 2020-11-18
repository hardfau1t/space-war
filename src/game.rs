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
use defmt::*;

// Structs definitions
#[derive(Debug, Copy, Clone)]
pub struct Player {
    x:i16,
    y:i16,
    pub vel_x:i16,
    pub vel_y:i16,
    pub bullets_cnt:i16,
    max_shots:i16,
    sprite_width:u8,
    sprite_height:u8,
    raw_image: ImageRaw<'static, BinaryColor>,
    active: bool
}

#[derive(Debug,Clone)]
pub struct Enemy {
    x:i16,
    y:i16,
    pub vel_x:i16,
    pub vel_y:i16,
    sprite_width:u8,
    sprite_height:u8,
    // number bullets are created by enemy and number of active
    raw_image: ImageRaw<'static, BinaryColor>,
    active:bool,
    pub bullet:*mut Bullet,
}

#[derive(Debug, Clone)]
pub struct Bullet {
    x:i16,
    y:i16,
    pub friendly:bool,
    pub vel_y:i16,       // no need of x vel, they will always move in straight line
    sprite_width:u8,
    sprite_height:u8,
    raw_image: ImageRaw<'static, BinaryColor>,
    active: bool,
    parent:*mut Enemy,
}

#[derive(Debug, Copy, Clone)]
pub struct Asteroids {
    x:i16,
    y:i16,
    pub vel_x:i16,
    pub vel_y:i16,
    sprite_width:u8,
    sprite_height:u8,
    raw_image: ImageRaw<'static, BinaryColor>,
    active:bool,
}
// Object struct is to group all objects like player bullets, enemy

pub trait Object{
    fn get_pos(&self)->(i16,i16);
    fn set_pos(&mut self, x:i16, y:i16);
    fn update(&mut self);
    fn draw(&self, disp:&mut Display);
    fn get_corner(&self)->(i16, i16);

    /// if objects is it will return false;
    fn is_active(&self)->bool;

    /// check if that object crosses the boundary
    fn boundary_check(&mut self);
}

pub trait Shooter{
    // Creates a bullet which is also an Player
    // type Counter;
    fn shoot(&mut self)->Result<Bullet, ()>;
}
// implementation Section
impl Player{
    pub fn new(x:i16, y:i16, sprite: &Sprite)->Self{
        let raw_image = ImageRaw::new(sprite.data, sprite.width as u32, sprite.height as u32);
        Self{ x, y, vel_x:0, vel_y:0,sprite_width: sprite.width, sprite_height:sprite.height, raw_image, active:true, bullets_cnt:0, max_shots:10}
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
    pub fn kill(&mut self){
        self.active = false;
    }
    pub fn max_shots(&self)->i16{
        self.max_shots
    }

}


impl Enemy{
    pub fn new(x:i16, y:i16, sprite: &Sprite)->Self{
        let raw_image = ImageRaw::new(sprite.data, sprite.width as u32, sprite.height as u32);
        Self{ 
            x, y, vel_x:0, vel_y:0,sprite_width: sprite.width, 
            sprite_height:sprite.height, raw_image, active:true, 
            bullet: core::ptr::null_mut(),
        }
    }
    pub fn kill(&mut self){
        self.active = false;
        let bullet = self.bullet as *mut Bullet;
        if !bullet.is_null(){
            unsafe{
                (*bullet).parent = core::ptr::null_mut();
            }
        }
    }
}

impl Bullet{
// no new function for bullet. 
// it should be created using shoot
    // pub fn is_hit(&mut self, enemies:&Vec<Enemy, _>, asteroids:&Vec<Asteroids, _>){
    // }
    pub fn kill(&mut self){
        self.active = false;
        let parent = self.parent as *mut Enemy;
        if !parent.is_null(){
            debug!("setting child to null");
            unsafe{
                 (*parent).bullet = core::ptr::null_mut();
            }
        }
    }
}

impl Asteroids{
    pub fn new(x:i16, y:i16, sprite: &Sprite)->Self{
        let raw_image = ImageRaw::new(sprite.data, sprite.width as u32, sprite.height as u32);
        Self{ x, y, vel_x:-1, vel_y:1,sprite_width: sprite.width, sprite_height:sprite.height, raw_image, active:true}
    }
    pub fn kill(&mut self){
        self.active = false;
    }
}

// these implementations are later changed into macros, currently for simplicity it is implimented
// for every object
impl Object for Player {
    fn get_pos(&self)->(i16, i16){
        (self.x, self.y)
    }

    fn set_pos(&mut self, x:i16, y:i16){
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

    fn is_active(&self) ->bool {
        self.active
    }

    fn get_corner(&self)->(i16, i16){
        let (x, y)= self.get_pos();
        (x+ self.sprite_width as i16, y + self.sprite_height as i16)
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
    fn get_pos(&self)->(i16, i16){
        (self.x, self.y)
    }

    fn set_pos(&mut self, x:i16, y:i16){
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
    fn is_active(&self) ->bool {
        self.active
    }
    fn get_corner(&self)->(i16, i16){
        let (x, y)= self.get_pos();
        (x+ self.sprite_width as i16, y + self.sprite_height as i16)
    }
    fn boundary_check(&mut self){
    }
}

impl Object for Bullet {
    fn get_pos(&self)->(i16, i16){
        (self.x, self.y)
    }

    fn set_pos(&mut self, x:i16, y:i16){
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
    fn is_active(&self) ->bool {
        self.active
    }
    fn get_corner(&self)->(i16, i16){
        let (x, y)= self.get_pos();
        (x+ self.sprite_width as i16, y + self.sprite_height as i16)
    }
    fn boundary_check(&mut self){
        // check for players boundary check
        let new_pos:i16  = (self.y + self.vel_y) as i16;
        if self.friendly {
            if new_pos < 1 {
                self.active = false;
            }
        } else {
            if new_pos > (DISPLAY_HEIGHT - self.sprite_height - 2) as i16{
                self.active = false
            }
        }
    }
}

impl Object for Asteroids {
    fn get_pos(&self)->(i16, i16){
        (self.x, self.y)
    }

    fn set_pos(&mut self, x:i16, y:i16){
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
    fn is_active(&self) ->bool {
        self.active
    }
    fn get_corner(&self)->(i16, i16){
        let (x, y)= self.get_pos();
        (x+ self.sprite_width as i16, y + self.sprite_height as i16)
    }
    fn boundary_check(&mut self){
        // check for players boundary check
        let new_pos:i16  = (self.x + self.vel_x) as i16;
        if new_pos <= 1  || new_pos + self.sprite_width as i16 >= DISPLAY_WIDTH as i16 - 2{
            self.vel_x = -self.vel_x ;
        }
        if self.y as i16 + self.vel_y as i16 + self.sprite_height as i16 >= (DISPLAY_HEIGHT as i16 ){
            self.active = false;
        }
    }
}


impl Shooter for Player{
    fn shoot(&mut self)->Result<Bullet, ()>{
        let raw_image = ImageRaw::new(BULLET_SPRITE.data, BULLET_SPRITE.width as u32, BULLET_SPRITE.height as u32);
        if self.bullets_cnt < self.max_shots{
            self.bullets_cnt +=1;
            Ok(Bullet{
                x:self.x + self.sprite_width as i16/2 - BULLET_SPRITE.width as i16/2,
                // if object is friendly then y = y - bullet height else y = y+bullet height;
                y: self.y - BULLET_SPRITE.height as i16, 
                friendly: true,
                sprite_height: BULLET_SPRITE.height,
                sprite_width : BULLET_SPRITE.width,
                vel_y: -3, // if friendly then vel_y is -ve
                raw_image,
                active:true,
                parent: core::ptr::null_mut(),
            })
        } else{
            Err(())
        }
    }
}

impl Shooter for Enemy{
    fn shoot(&mut self)->Result<Bullet, ()>{
        let raw_image = ImageRaw::new(BULLET_SPRITE.data, BULLET_SPRITE.width as u32, BULLET_SPRITE.height as u32);
        if self.bullet.is_null(){
            let mut bullet = Bullet{
                x:self.x + self.sprite_width as i16/2 - BULLET_SPRITE.width as i16/2,
                // if object is friendly then y = y - bullet height else y = y+bullet height;
                y: self.y + BULLET_SPRITE.height as i16, 
                friendly: false,
                sprite_height: BULLET_SPRITE.height,
                sprite_width : BULLET_SPRITE.width,
                vel_y: 2, // if friendly then vel_y is -ve
                raw_image,
                active:true,
                parent:self,
            };
            self.bullet = &mut bullet;
            Ok(bullet)
        } else{
            Err(())
        }
    }
}

// SAFETY: TODO: add a state to check weather parent is alive or not
unsafe impl Send for Bullet{
}
unsafe impl Send for Enemy{}
// while dropping bullet
impl Drop for Bullet{
    fn drop(&mut self) {
        // bullet spawned by player then do nothing
        // if the bullet is spawned by enemy and the parent is alive then
        if !(self.friendly || self.parent.is_null()){
        }
    }
}

impl Drop for Enemy{
    fn drop(&mut self) {
    }
}

// import Section 
use crate::{
    types::{Left, Right, Display},
    objects::*,
};
use embedded_graphics::{
    prelude::*,
    pixelcolor::BinaryColor,
    image::{Image, ImageRaw},
    fonts::{ Font6x8, Text },
    drawable::Drawable,
    style::Styled,
    style::{PrimitiveStyle, TextStyle},
    primitives::Rectangle,
};

use heapless::{
    Vec,
    consts::*,
};

use stm32f7xx_hal::prelude::*;

// Structs definitions
#[derive(Debug)]
pub struct Player {
    x:i16,
    y:i16,
    vel_x:i16,
    vel_y:i16,
    pub active: bool,
    pub bullets:Vec<Bullet, U4>,
    raw_image: ImageRaw<'static, BinaryColor>,
    pub player_score:i16,
    cool_down: u8,
}

#[derive(Debug)]
pub struct Enemy {
    x:i16,
    y:i16,
    // number bullets are created by enemy and number of active
    raw_image: ImageRaw<'static, BinaryColor>,
    pub active:bool,
    pub bullet_cool_down: u16,
    cool_down:u16,
}

#[derive(Debug)]
pub struct Bullet {
    x:i16,
    y:i16,
    friendly:bool,
    vel_x:i8,       // no need of x vel, they will always move in straight line
    vel_y:i8,       // no need of x vel, they will always move in straight line
    raw_image: ImageRaw<'static, BinaryColor>,
    pub active: bool,
}

#[derive(Debug)]
pub struct Asteroid {
    x:i16,
    y:i16,
    vel_x:i8,
    vel_y:i8,
    raw_image: ImageRaw<'static, BinaryColor>,
    pub active:bool,
}

#[derive(Debug)]
pub struct Sprite{
    pub data:&'static [u8],
    pub width:u8,
    pub height:u8,
}

#[derive(Debug)]
pub struct Screen{
    width:u8,
    height:u8,
    border: Styled<Rectangle, PrimitiveStyle<BinaryColor>>,
}

#[derive(Debug)]
pub struct Stats{
    pub border: Styled<Rectangle, PrimitiveStyle<BinaryColor>>,
    pub score:ImageRaw<'static, BinaryColor>,
    pub ammo:ImageRaw<'static, BinaryColor>,
}

/// this is used for indicating boundary condition
pub enum Boundary{
    Fine,
    Breach,
}
// Object struct is to group all objects like player bullets, enemy

pub trait Object{
    /// if objects is it will return false;
    fn is_active(&self)->bool;
    /// drops the object
    fn bury(self, score:&mut i16);
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
    fn draw(&self, disp:&mut Display);
}
// implementation Section
impl Screen{
    pub fn new(width:u8, height:u8, border:Styled<Rectangle, PrimitiveStyle<BinaryColor>>)->Self{
        Self{ height, width, border }
    }
    pub fn height(&self)->u8{
        self.height
    }
    pub fn width(&self)->u8{
        self.width
    }
}

impl Player{
    pub fn new(x:i16, y:i16, sprite: &Sprite)->Self{
        let raw_image = ImageRaw::new(sprite.data, sprite.width as u32, sprite.height as u32);
        let bullets = Vec::new();
        Self{ x, y, vel_x:0, vel_y:0, raw_image,
            active:true, bullets,
            player_score:0,
            cool_down:0,
        }
    }
    pub fn update(&mut self, dir:&(Left, Right), screen:&Screen) {
        if self.cool_down !=0{
            self.cool_down -=1;
        }
        self.mov(dir);
        self.boundary_check(screen);
        self.x += self.vel_x;
        self.y += self.vel_y;
    }
    pub fn mov(&mut self, dir:&(Left, Right)){
        // check if left button is pressed,
        if let Ok(left) = dir.0.is_low(){
            if left{
                self.vel_x = -2;
                // when left is pressed but right is not
                // then velocity will be 0. to bypass that we return early
                return;
            } else {
                self.vel_x = 0;
            }
        }
        if let Ok(right) = dir.1.is_low(){
            if right{
                self.vel_x = 2;
            } else {
                self.vel_x = 0;
            }
        }
    }
    pub fn get_corner_pos(&self)->(i16, i16){
        let (x, y) = self.get_pos();
        (x + self.raw_image.width() as i16, y + self.raw_image.height()as i16)
    }
    pub fn shoot(&mut self){
        let raw_image = ImageRaw::new(BULLET_SPRITE.data, BULLET_SPRITE.width as u32, BULLET_SPRITE.height as u32);
        let x = self.x + self.raw_image.width() as i16/2 - BULLET_SPRITE.width as i16/2;
        // if object is friendly then y = y - bullet height else y = y+bullet height;
        let y =  self.y - BULLET_SPRITE.height as i16; 
        defmt::debug!("spawning friendly bullet at ({:?}, {:?})", x, y);
        self.cool_down = match self.bullets.push(Bullet{
            x,y,
            friendly: true,
            vel_y: -3, // if friendly then vel_y is -ve
            vel_x:0,
            raw_image,
            active:true,
        }){
            // Ok(_r=>{},
            Ok(_)=> PLAYER_COOL_DOWN,
            Err(_)=>0,
        };
    }
    fn boundary_check(&mut self, screen:&Screen) {
        // check on both right upper corner and left lower corner if anyone of them crosses the
        // boundary then take an action on them

        // since player moves only in x axis, checking on x axis is sufficient
        //
        // we are taking new position, if not player will get stuck on hitting border
        let new_pos = self.x + self.vel_x;
        if new_pos <= 1 - self.raw_image.width()as i16/2  || new_pos + self.raw_image.width()as i16/2   >= screen.width as i16 {
            self.vel_x = 0;
        }
    }
    pub fn can_shoot(&self)->bool{
        self.cool_down == 0
    }
}

impl Enemy{
    pub fn new(x:i16, y:i16, sprite: &Sprite, cool_down:u16)->Self{
        let raw_image = ImageRaw::new(sprite.data, sprite.width as u32, sprite.height as u32);
        defmt::debug!("spawn: Enemy at ({:?}, {:?})", x,y);
        Self{ x, y, raw_image, active:true, bullet_cool_down:40, cool_down:cool_down*2+50}
    }
    pub fn update(&mut self) {
        // no need of boundary check for enemy
        // count down counter
        if self.bullet_cool_down > 0{
            self.bullet_cool_down -=1;
        }
    }
    pub fn get_corner_pos(&self)->(i16, i16){
        let (x, y) = self.get_pos();
        (x + self.raw_image.width() as i16, y + self.raw_image.height()as i16)
    }
    pub fn shoot(&mut self)-> Option<Bullet>{
        let raw_image = ImageRaw::new(BULLET_SPRITE.data, BULLET_SPRITE.width as u32, BULLET_SPRITE.height as u32);
        if self.bullet_cool_down == 0{
            self.bullet_cool_down = self.cool_down;
            let x = self.x + self.raw_image.width() as i16/2 - BULLET_SPRITE.width as i16/2;
            // if object is friendly then y = y - bullet height else y = y+bullet height;
            let y = self.y + BULLET_SPRITE.height as i16; 
            defmt::debug!("spawning foes bullet at ({:?}, {:?})", x, y);
            Some(Bullet{
                x,y,
                friendly: false,
                vel_y: 2, // if friendly then vel_y is -ve
                vel_x:0,
                raw_image,
                active:true,
            })
        } else {
            None
        }
    }
}

impl Bullet{
// no new function for bullet. 
// it should be created using shoot
    pub fn update(&mut self, screen:&Screen) {
        self.boundary_check(&screen);
        self.x += self.vel_x as i16;
        self.y += self.vel_y as i16;
    }
    pub fn is_friendly(&self)->bool{
        self.friendly
    }
    pub fn get_corner_pos(&self)->(i16, i16){
        let (x, y) = self.get_pos();
        (x + self.raw_image.width() as i16, y + self.raw_image.height()as i16)
    }
    pub fn boundary_check(&mut self, screen:&Screen){
        // check for players boundary check
        let new_pos:i16  = self.y + self.vel_y as i16 ;
        if self.friendly {
            if new_pos < 1 {
                self.active = false;
            }
        } else {
            if new_pos > screen.height() as i16{
                self.active = false
            }
        }
    }
}

impl Asteroid{
    pub fn new(x:i16, y:i16, sprite: &Sprite, random_val:u32)->Self{
        let raw_image = ImageRaw::new(sprite.data, sprite.width as u32, sprite.height as u32);

        let vel_x = (random_val % 3)as i8 - 1;
        defmt::debug!("spawn: asteroid at ({:?}, {:?})", x,y);
        Self{ x, y, vel_x , vel_y:1, raw_image, active:true}
    }
    pub fn update(&mut self, screen:&Screen) {
        self.boundary_check(screen);
        self.x += self.vel_x as i16;
        self.y += self.vel_y as i16;
    }
    pub fn get_corner_pos(&self)->(i16, i16){
        let (x, y) = self.get_pos();
        (x + self.raw_image.width() as i16, y + self.raw_image.height()as i16)
    }
    fn boundary_check(&mut self, screen:&Screen){
        // check for players boundary check
        let new_pos:i16  = self.x + self.vel_x as i16;
        if new_pos <= 1  || new_pos + self.raw_image.width() as i16 >= screen.width as i16 {
            self.vel_x = -self.vel_x ;
        }
        if self.y + self.vel_y as i16 >= screen.height() as i16 {
            self.active = false;
        }
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

impl Stats{
    pub fn new(screen: &Screen)->Self{
        let border = Rectangle::new(
            Point::new(0,screen.height() as i32 +1), Point::new(screen.width as i32 + 1, screen.height() as i32 + 12)
            )
            .into_styled(
                PrimitiveStyle::with_stroke(BinaryColor::On, 1)
                );
        let score = ImageRaw::new(GUN.data, GUN.width.into(), GUN.height.into());
        let ammo = ImageRaw::new(AMMO.data, AMMO.width.into(), AMMO.height.into());
        Self{
            border,
            score,
            ammo,
        }
    }
}

// these implementations are later changed into macros, currently for simplicity it is implimented
// for every object
impl Object for Player {



    fn is_active(&self) ->bool {
        self.active
    }
    fn bury(self, _:&mut i16){
        todo!()
    }


}

impl Object for Enemy {
    fn is_active(&self) ->bool {
        self.active
    }
    fn bury(self, score: &mut i16){
        defmt::debug!("player score: {:?}", *score);
        *score +=1;
    }
}

impl Object for Bullet {
    fn is_active(&self) ->bool {
        self.active
    }
    fn bury(self, _:&mut i16) {
    }
}

impl Object for Asteroid {
    fn is_active(&self) ->bool {
        self.active
    }

    fn bury(self, score:&mut i16) {
        *score +=1;
        defmt::debug!("player score: {:?}", *score);
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

impl CanDraw for Player{
    fn draw(&self, disp:&mut Display) {
        let image = Image::new( &self.raw_image, Point::new(self.x as i32, self.y as i32) );
        image.draw(disp).unwrap();
    }
}
impl CanDraw for Bullet{
    fn draw(&self, disp:&mut Display) {
        let image = Image::new( &self.raw_image, Point::new(self.x as i32, self.y as i32) );
        image.draw(disp).unwrap();
    }
}
impl CanDraw for Enemy{
    fn draw(&self, disp:&mut Display) {
        let image = Image::new( &self.raw_image, Point::new(self.x as i32, self.y as i32) );
        image.draw(disp).unwrap();
    }
}
impl CanDraw for Asteroid{
    fn draw(&self, disp:&mut Display) {
        let image = Image::new( &self.raw_image, Point::new(self.x as i32, self.y as i32) );
        image.draw(disp).unwrap();
    }
}
impl CanDraw for Screen{
    fn draw(&self, disp:&mut Display) {
        self.border.draw(disp).unwrap();
    }
}

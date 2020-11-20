#![no_std]
#![feature(const_in_array_repeat_expressions)]

// import Section 
pub mod types;
pub mod game;
pub mod objects;

use heapless::{
    Vec,
    String,
    consts::*,
};

use core:: sync::atomic::{AtomicUsize, Ordering};
// use core::panic::PanicInfo;

use game::* ;
use objects::*;
use types::*;

use defmt_rtt as _; // global logger
use panic_probe as _;

use embedded_graphics::{
    prelude::*,
    fonts::{ Font6x8, Text, Font8x16},
    image::{Image, ImageRaw},
    pixelcolor::BinaryColor,
    primitives::Rectangle,
    style::{PrimitiveStyle, TextStyle},
};

use stm32f7xx_hal::prelude::*;

// structs 

#[derive(Debug)]
pub struct GamePool{
    pub player: Player,
    enemies: Vec<Enemy, U10>,
    bullets:Vec<Bullet, U100>,
    asteroids: Vec<Asteroid, U20>,
    screen: Screen,
    stats: Stats,
    status: bool,
}
impl GamePool{
    // This will return all necessory game objects
    pub fn init(disp:&Display)->Self{
        let (disp_width, disp_height )= disp.get_dimensions();
        let disp_height = disp_height - 12;
        let border = Rectangle::new(
            Point::zero(), Point::new( (disp_width - 1 ) as i32, (disp_height - 1) as i32))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));
        // Get the screen
        let screen = Screen::new(
            // border takes 1 pixel so -2
            disp_width - 2,
            disp_height -2,
            border,
        );
        // start the player in center 
        let player = Player::new(
            (screen.width()/2 - PLAYER_1_SPRITE.width()/2 +1)as i16, 
            (screen.height() - PLAYER_1_SPRITE.height() - 1) as i16, // -1 for border
            &PLAYER_1_SPRITE
        );
        // enemies
        let enemies:Vec<Enemy, U10> = Vec::new();
        let bullets:Vec<Bullet, U100> = Vec::new();
        let asteroids:Vec<Asteroid, U20> = Vec::new();
        let stats = Stats::new(&screen);
        Self{player, enemies, bullets, asteroids, screen, stats, status:true}
    }

    /// spawns objects like enemies and asteroids, but not bullets
    pub fn spawn(&mut self, rng:&mut stm32f7xx_hal::rng::Rng) {
        // spawn asteroids
        while self.asteroids.len() as i16 <= self.player.player_score/LEVEL_SCORE as i16{
            // get random value for spawn position
            let random_val = match rng.get_rand(){
                Ok(val) => val,
                Err(_) => {
                    defmt::warn!("couldn't generate random value for asteroid. spawning will be corner");
                    0
                },
            };
            let x_pos = random_val% (self.screen.width() - &ASTEROID_SPRITE.width) as u32;
            // spawn asteroid
            let asteroid = Asteroid::new(
                x_pos as i16,
                1 - ASTEROID_SPRITE.height as i16,
                &ASTEROID_SPRITE,
                random_val
                );
            self.asteroids.push(
                asteroid
                ).expect("couldn't create enemy");
        }
        // spawn enemies
        while self.enemies.len() as i16 <= self.player.player_score/(LEVEL_SCORE*2) as i16{
            let rand_val:u32 = match rng.get_rand(){
                Ok(val) => val,
                Err(_)=>{
                    // if cant generate random value then spawn in center
                    defmt::warn!("couldn't generate random value for enemies. spawning will be center");
                    (self.screen.width() as u32 / 2)<< 16 | (self.screen.height()as u32 / 2)
                }
            };
            // let xpos:i16 = (self.screen.width()/2 -&ENEMY_SPRITE.width()/2 -1 ) as i16;
            let xpos:i16 = ((rand_val >> 16 ) as u16 % (self.screen.width() - &ENEMY_SPRITE.width) as u16 + 1) as i16;
            let ypos:i16 = (rand_val as u16 % (self.screen.width() - &ENEMY_SPRITE.width) as u16 + 1) as i16;
            let cooldown = LEVEL_SCORE as i16 - self.player.player_score % LEVEL_SCORE  as i16;
            let enemy = Enemy::new(xpos, ypos , &ENEMY_SPRITE, cooldown as u16);
            self.enemies.push(
                enemy
                ).expect("couldn't create enemy");
        }
        // spawn enemy bullets
        for i in 0..self.enemies.len(){
            if self.enemies[i].bullet_cool_down == 0{
                if let Some(bullet) = self.enemies[i].shoot(){
                    self.bullets.push(bullet).unwrap();
                }
            }
        }
    }
    
    pub fn update(&mut self, direction: &(Left, Right)){
        // update enemy bullet spawn speed
        // update player
        self.player.update(direction, &self.screen);
        
        // update enemies
        for index in 0..self.enemies.len(){
            self.enemies[index].update();
        }
        
        // update bullets position
        for index in 0..self.player.bullets.len(){
            self.player.bullets[index].update(&self.screen);

            // check if it is hitting anyone.
            let (x1, y1) = self.player.bullets[index].get_pos();
            let (x2, y2) = self.player.bullets[index].get_corner_pos();
            let mut killed = false;
            for i in 0..self.enemies.len(){
                let (x3, y3) = self.enemies[i].get_pos();
                let (x4, y4) = self.enemies[i].get_corner_pos();
                // here we are making some assumption
                // *) x1 is always less than x2 because & x3 is always < x4 
                //      because x1 & x3 are used as origin points for objects and we are not
                //      drawing outside of display
                // *) same for y
                // check if object is active because objects that are killed wont go away until
                // next frame
                if !(x3>x2 || x1 > x4 ||  y1 > y4 || y3 > y2) && self.enemies[i].is_active(){
                    // now these are overlapping
                    self.player.bullets[index].active = false;
                    self.enemies[i].active = false;
                    killed = true;
                    break;
                }
            }
            // if none of the enemies are killed by bullet then check for asteroids
            if !killed{
                for i in 0..self.asteroids.len(){
                    let (x3, y3) = self.asteroids[i].get_pos();
                    let (x4, y4) = self.asteroids[i].get_corner_pos();
                    if !(x3>x2 || x1 > x4 || y1 > y4 || y3 > y2)&& self.asteroids[i].is_active(){
                        // now these are overlapping
                        self.player.bullets[index].active = false;
                        self.asteroids[i].active = false;
                        break;
                    }
                }
            }
        }
        // enemy bullets action
        for index in 0..self.bullets.len(){
            self.bullets[index].update(&self.screen);

            // check if it is hitting anyone.
            let (x1, y1) = self.bullets[index].get_pos();
            let (x2, y2) = self.bullets[index].get_corner_pos();
            // if the bullet is from opponent
            let (x3, y3) = self.player.get_pos();
            // CHEAT: so that player hit box is reduced
            let x3 = x3 + 1;
            let y3 = y3 + 1;
            let (x4, y4) = self.player.get_corner_pos();
            let x4 = x4 - 1;
            if !(x3>x2 || x1 > x4 || y1 > y4 || y3 > y2){
                self.status = false;
            }
        }
        // update asteroids position
        for index in 0..self.asteroids.len(){
            self.asteroids[index].update(&self.screen);
            
            // check for asteroids hit
            let (x1, y1) = self.asteroids[index].get_pos();
            let (x2, y2) = self.asteroids[index].get_corner_pos();
            let (x3, y3) = self.player.get_pos();
            let (x4, y4) = self.player.get_corner_pos();
            if !(x3>x2 || x1 > x4 || y1 > y4 || y3 > y2){
                // TODO: set the player to inactive and latery bury
                self.status = false;
            }
        }
    }

    // collects all the elements that are dead and calls burry on them
    pub fn collect(&mut self){
        if !self.player.is_active(){
            todo!()
        }
        let mut removed = 0;
        for mut index in 0..self.player.bullets.len(){
            index -= removed;
            if !self.player.bullets[index].is_active(){
                self.player.bullets.swap_remove(index);
                removed +=1;
            }
        }
        removed =0;
        for mut index in 0..self.enemies.len(){
            index -= removed;
            if !self.enemies[index].is_active(){
                self.enemies.swap_remove(index).bury(&mut self.player.player_score);
                removed +=1;
            }
        }
        removed =0;
        for mut index in 0..self.bullets.len(){
            index -= removed;
            if !self.bullets[index].is_active(){
                self.bullets.swap_remove(index);
                removed +=1;
            }
        }
        removed =0;
        for mut index in 0..self.asteroids.len(){
            index -= removed;
            if !self.asteroids[index].is_active(){
                self.asteroids.swap_remove(index).bury(&mut self.player.player_score);
                removed +=1;
            }
        }
    }

    /// draw all objects in the game
    pub fn draw(&self, disp:&mut Display){
        // update player
        self.player.draw(disp);
        self.screen.draw(disp);
        for index in 0..self.enemies.len(){
            self.enemies[index].draw(disp);
        }
        for index in 0..self.bullets.len(){
            self.bullets[index].draw(disp);
        }
        for index in 0..self.player.bullets.len(){
            self.player.bullets[index].draw(disp);
        }
        for index in 0..self.asteroids.len(){
            self.asteroids[index].draw(disp);
        }
    }
    pub fn draw_stats(&self, disp:&mut Display){
        self.stats.border.draw(disp).unwrap();
        Image::new( 
            &self.stats.score,
            Point::new(3, self.screen.height() as i32 +3) )
            .draw(disp)
            .unwrap();
        Image::new( 
            &self.stats.ammo,
            Point::new(42, self.screen.height() as i32 + 5) )
            .draw(disp)
            .unwrap();

        // player score
        let a:String<U6> = String::from(self.player.player_score);
        Text::new(
            a.as_str(),
            Point::new(22, self.screen.height() as i32 + 4)
            )
            .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
            .draw(disp).unwrap();

        // player ammo
        let a:String<U6> = String::from((self.player.bullets.capacity() - self.player.bullets.len()) as i16);
        Text::new(
            a.as_str(),
            Point::new(56, self.screen.height() as i32 + 4)
            )
            .into_styled(TextStyle::new(Font6x8, BinaryColor::On))
            .draw(disp).unwrap();
    }
    
    pub fn is_ok(&self)->bool{
        !self.status
    }
}

#[defmt::timestamp]
fn timestamp() -> u64 {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    let n = COUNT.load(Ordering::Relaxed);
    COUNT.store(n + 1, Ordering::Relaxed);
    n as u64
}

/// Terminates the application and makes `probe-run` exit with exit-code = 0
pub fn exit() -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}


pub fn final_screen(score:i16, disp:&mut Display, delay:&mut Delay)->!{
    let image = ImageRaw::new( GUN.data, GUN.width() as u32, GUN.height() as u32);
    let gun: Image<ImageRaw<BinaryColor>, BinaryColor> = Image::new(
        &image,
        Point::new(3, 118)
        );
    let score_txt :String<U6> = String::from(score);
    let score_disp = Text::new(
            score_txt.as_str(),
            Point::new(22, 118)
            )
            .into_styled(TextStyle::new(Font6x8, BinaryColor::On)) ;
    let score_info = Text::new(
            score_txt.as_str(),
            Point::new(
                31 - (score_txt.len()as i32 *8)/2 ,
                 93)
            )
            .into_styled(TextStyle::new(Font8x16, BinaryColor::On)) ;
    let game = Text::new("Game", Point::new(5,20))
        .into_styled(TextStyle::new(Font8x16, BinaryColor::On));
    let over = Text::new("Over", Point::new(28, 50))
        .into_styled(TextStyle::new(Font8x16, BinaryColor::On));

    let border = Rectangle::new(
        Point::zero(), Point::new(63, 127))
        .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));

    let sub = Text::new("you score", Point::new(7, 80))
        .into_styled(TextStyle::new(Font6x8, BinaryColor::On));
    loop{
        disp.clear();
        border.draw(disp).unwrap();
        game.draw(disp).unwrap();
        gun.draw(disp).unwrap();
        score_disp.draw(disp).unwrap();
        score_info.draw(disp).unwrap();
        sub.draw(disp).unwrap();
        disp.flush().unwrap();
        delay.delay_ms(700u16);

        disp.clear();
        border.draw(disp).unwrap();
        over.draw(disp).unwrap();
        gun.draw(disp).unwrap();
        score_disp.draw(disp).unwrap();
        score_info.draw(disp).unwrap();
        sub.draw(disp).unwrap();
        disp.flush().unwrap();
        delay.delay_ms(700u16);
    }
}

// #[panic_handler]
// fn panic_handle(info: &PanicInfo)->!{
//     defmt::debug!("{:?}", info;
//     exit();
// }

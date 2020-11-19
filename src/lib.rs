#![no_std]
#![feature(const_in_array_repeat_expressions)]

// import Section 
pub mod types;
pub mod game;
pub mod objects;

use heapless::{
    Vec,
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
    pixelcolor::BinaryColor,
    primitives::Rectangle,
    style::PrimitiveStyle,
    // drawable::Drawable,
};


// structs 

#[derive(Debug)]
pub struct GamePool{
    pub player: Player,
    enemies: Vec<Enemy, U10>,
    bullets:Vec<Bullet, U100>,
    asteroids: Vec<Asteroid, U20>,
    screen: Screen,
}
impl GamePool{
    // This will return all necessory game objects
    pub fn init(disp:&Display)->Self{
        let (disp_width, disp_height )= disp.get_dimensions();
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
        Self{player, enemies, bullets, asteroids, screen}
    }

    /// spawns objects like enemies and asteroids, but not bullets
    pub fn spawn(&mut self) {
        // spawn asteroids
        // while self.asteroids.len() == 0{
        //     let asteroid = Asteroids::new((DISPLAY_WIDTH-&ASTEROID_SPRITE.width -2 ) as i16, 1 - ASTEROID_SPRITE.height as i16, &ASTEROID_SPRITE);
        //     self.asteroids.push(
        //         asteroid
        //         ).expect("couldn't create enemy");
        // }
        // spawn enemies
        // while self.enemies.len() == 0{
        //     let enemy = Enemy::new((self.screen.width()/2 -&ENEMY_SPRITE.width()/2 -1 ) as i16, 1 , &ENEMY_SPRITE);
        //     self.enemies.push(
        //         enemy
        //         ).expect("couldn't create enemy");
        // }
        // // spawn enemy bullets
        // for i in 0..self.enemies.len(){
        //     if self.enemies[i].bullet_cool_down == 0{
        //         if let Some(bullet) = self.enemies[i].shoot(){
        //             self.bullets.push(bullet).unwrap();
        //         }
        //     }
        // }
    }
    
    pub fn update(&mut self, direction: &(Left, Right)){
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
            let (x1, y1) = self.bullets[index].get_pos();
            let (x2, y2) = self.bullets[index].get_corner_pos();
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
                    self.bullets[index].active = false;
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
                        self.bullets[index].active = false;
                        self.asteroids[i].active = false;
                        break;
                    }
                }
            }
        }
        for index in 0..self.bullets.len(){
            defmt::debug!("open");
            self.player.bullets[index].update(&self.screen);
            defmt::debug!("close");

            // check if it is hitting anyone.
            let (x1, y1) = self.bullets[index].get_pos();
            let (x2, y2) = self.bullets[index].get_corner_pos();
            // if the bullet is from opponent
            let (x3, y3) = self.player.get_pos();
            let (x4, y4) = self.player.get_corner_pos();
            if !(x3>x2 || x1 > x4 || y1 > y4 || y3 > y2){
                panic!("game over");
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
                defmt::debug!("game over");
                panic!("game over");
            }
        }
    }

    // collects all the elements that are dead and calls burry on them
    pub fn collect(&mut self){
        if !self.player.is_active(){
            todo!()
        }
        for index in 0..self.player.bullets.len(){
            if !self.player.bullets[index].is_active(){
                self.player.bullets.swap_remove(index).bury();
            }
        }
        for index in 0..self.enemies.len(){
            if !self.enemies[index].is_active(){
                self.enemies.swap_remove(index).bury();
            }
        }
        for index in 0..self.bullets.len(){
            if !self.bullets[index].is_active(){
                self.bullets.swap_remove(index).bury();
            }
        }
        for index in 0..self.asteroids.len(){
            if !self.asteroids[index].is_active(){
                self.asteroids.swap_remove(index).bury();
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
        for index in 0..self.asteroids.len(){
            self.asteroids[index].draw(disp);
        }
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

// #[panic_handler]
// fn panic_handle(info: &PanicInfo)->!{
//     defmt::debug!("{:?}", info;
//     exit();
// }

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
    style::Styled,
    // drawable::Drawable,
};


// structs 
/// GameObject will contain all types of objects available in game to manage them from
/// one place
pub struct GameObject{
    pub player:Player,
    pub enemies:Vec<Enemy, U10>,
    pub bullets:Vec<Bullet, U100>,
    pub asteroids:Vec<Asteroids, U20>,
    border: Styled<Rectangle, PrimitiveStyle<BinaryColor>>,
}

impl GameObject{
    pub fn init()->Self{
        // start the player in center 
        let player = Player::new(
            (DISPLAY_WIDTH/2 - PLAYER_1_SPRITE.width/2 +1)as i8, 
            (DISPLAY_HEIGHT - PLAYER_1_SPRITE.height - 1) as i8, // -1 for border
            &PLAYER_1_SPRITE
        );
        // enemies
        let enemies:Vec<Enemy, _> = Vec::new();
        let bullets:Vec<Bullet, _> = Vec::new();
        let asteroids:Vec<Asteroids, _> = Vec::new();
        let border = Rectangle::new(
            Point::zero(), Point::new(
                (DISPLAY_WIDTH - 1 ) as i32,
                (DISPLAY_HEIGHT - 1) as i32
            ))
            .into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));
        Self{
            player, enemies, bullets, asteroids, border
        }
    }
    
    pub fn update(&mut self, direction: &(Left, Right)){
        while self.asteroids.len() == 0{
            let asteroid = Asteroids::new((DISPLAY_WIDTH-&ASTEROID_SPRITE.width -2 ) as i8, 1 - ASTEROID_SPRITE.height as i8, &ASTEROID_SPRITE);
            self.asteroids.push(
                asteroid
                ).expect("couldn't create enemy");
        }
        // update player
        self.player.mov(direction);
        self.player.boundary_check();
        self.player.update();
        
        // update enemies
        let mut removed = 0;
        for index in 0..self.enemies.len(){
            let index = index - removed;
            self.enemies[index].boundary_check();
            self.enemies[index].update();
            if !self.enemies[index].is_active(){
                // dont know how this works but it works.
                self.enemies.swap_remove(index);
                removed +=1;
            }
        }
        
        // update bullets position
        removed = 0;
        for index in 0..self.bullets.len(){
            let index = index - removed;
            self.bullets[index].boundary_check();
            self.bullets[index].update();
            if !self.bullets[index].is_active(){
                self.bullets.swap_remove(index);
                removed +=1;
            }
        }
        // update asteroids position
        removed = 0;
        for index in 0..self.asteroids.len(){
            let index = index - removed;
            self.asteroids[index].boundary_check();
            self.asteroids[index].update();
            if !self.asteroids[index].is_active(){
                self.asteroids.swap_remove(index);
                removed +=1;
            }
        }
    }
    pub fn draw(&self, disp:&mut Display){
        // update player
        self.player.draw(disp);
        self.border.draw(disp).expect("cant draw border");
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

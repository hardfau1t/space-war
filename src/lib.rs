#![no_std]
#![feature(const_in_array_repeat_expressions)]

pub mod types;
pub mod game;

use game::Object;
use core::sync::atomic::{AtomicUsize, Ordering};
use game::* ;

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

pub fn game_init()->[Object;2] {
    let player_style:Styled<Rectangle, PrimitiveStyle<BinaryColor>> = Rectangle::new(Point::new(0, 28), Point::new(3, 36) ).into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));
    let opponent_style:Styled<Rectangle, PrimitiveStyle<BinaryColor>> = Rectangle::new(Point::new(124, 28), Point::new(127, 36) ).into_styled(PrimitiveStyle::with_stroke(BinaryColor::On, 1));
    let player = Object::new(123u8, 3u8, player_style);
    let opponent = Object::new(3u8, 3u8, opponent_style);
    [player, opponent]
}

pub fn game_update(player: &mut Object, opponent:&mut Object) {
    player.set_pos(player.get_pos().0 +1, player.get_pos().1);
    opponent.set_pos(opponent.get_pos().0 +1, opponent.get_pos().1);
}

pub fn game_draw(player: &Object, opponent:&Object, disp: &mut types::Display) {
    player.draw(disp);
    opponent.draw(disp);
    disp.flush().unwrap();
}

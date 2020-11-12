#![no_std]
#![feature(const_in_array_repeat_expressions)]

// import Section 
pub mod types;
pub mod game;
pub mod objects;


use core::{
    sync::atomic::{AtomicUsize, Ordering},
    iter::Iterator,
};
use game::* ;
use objects::*;

use defmt_rtt as _; // global logger
use panic_probe as _;

// use embedded_graphics::{
//     prelude::*,
//     pixelcolor::BinaryColor,
//     primitives::Rectangle,
//     style::PrimitiveStyle,
//     // style::Styled,
//     // drawable::Drawable,
// };

// Constants
pub const DISPLAY_WIDTH:u8 = 64;
pub const DISPLAY_HEIGHT:u8 = 128;
pub const FPS_LIMIT:u16      = 10;

// structs or enums Section
// a linked list for objects.
// idle loop will have all objects in its stack.
/// Concept of Linked list here is that while pushing a new node to linked list,
/// instead of detaching old node and attaching new node as head and attaching old
/// node as next, we simply attach linked list to the new_node as next.
/// problem with this approch is we will get different linked list each time we push a node.
pub struct Node<'a>{
    object: Object,
    next: Option<&'a Node<'a>>,
}

impl<'a> Node<'a>{
    // instead of pushing new node to linked list we are linking old head to new node
    /// this will link the list to new node. use new node as head
    pub fn link(& mut self, node:&'a Node<'a>){
        self.next = Some(node)
    }
    /// return reference to object for read
    pub fn peek(&self)->&Object{
        &self.object
    }
    /// returns a mutable reference to object for modifieng
    pub fn as_mut(&mut self)->&mut Object{
        &mut self.object
    }
}

impl<'a> Iterator for Node<'a>{
    type Item = Object;
    fn next(&mut self) -> Option<Self::Item> {
        todo!()
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

pub fn game_init()->Object{
    // start the player in center 
    let player = Object::new(
        DISPLAY_WIDTH/2 - PLAYER_1.width/2 +1, 
        DISPLAY_HEIGHT - PLAYER_1.height - 1, // -1 for border
        true,
        &PLAYER_1);
    player
}

pub fn game_update(object: &mut Object) {
    object.set_pos(object.get_pos().0 +1, object.get_pos().1);
}

pub fn game_draw(object: &Object, disp: &mut types::Display) {
    object.draw(disp);
}

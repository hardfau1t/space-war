#![no_std]
#![feature(const_in_array_repeat_expressions)]

// import Section 
pub mod types;
pub mod game;
pub mod objects;


use core::{
    sync::atomic::{AtomicUsize, Ordering},
    cell::{RefCell, Ref, RefMut},
};

use game::* ;
use objects::*;
use types::*;

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
#[derive(Clone, Debug)]
pub struct Node<'a,T: Object>{
    object: RefCell<T>,
    next: Option<&'a Node<'a, T>>,
}

impl<'a, T:Object> Node<'a, T>{
    pub fn new(obj:T, next:Option<&'a Node<'a, T>>)->Self{
        Node{
            object:RefCell::new(obj),
            next
        }
    }
    pub fn peek(&self)->Ref<T>{
        self.object.borrow()
    }

    pub fn next(&self)->Option<&Node<'a, T>>{
        self.next
    }

    pub fn as_mut(&self)->RefMut<T>{
        self.object.borrow_mut()
    }
}

pub struct List<'a, T:Object>{
    pub head:Option<&'a Node<'a, T>>
}
impl<'a, T:Object> List<'a, T>{
    // instead of pushing new node to linked list we are linking old head to new node
    /// this will link the list to new node. use new node as head
    pub fn new()->Self{
        Self{
            head:None,
        }
    }

    pub fn push(&mut self, node: &'a Node<'a, T>){
        self.head = Some(node)
    }
    // / return reference to object for read
    
    pub fn update(&mut self){
        let mut head = self.head;
        while let Some(node) = head{
            node.as_mut().update();
            head = node.next;
        }
    }
    pub fn draw(&mut self, disp:&mut Display){
        let mut head = self.head;
        while let Some(node) = head{
            node.as_mut().draw(disp);
            head = node.next;
        }
    }

}
// impl<'a, T: Object> Iterator for Node<'a, T>{
//     type Item = &'a mut T;
//     fn next(&mut self) -> Option<Self::Item> {
//         match self.next{
//             Some(&node)=> Some(&mut node.object),
//             None => None,
//         }
//     }
// }

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

pub fn game_init()->Player{
    // start the player in center 
    Player::new(
        (DISPLAY_WIDTH/2 - PLAYER_1.width/2 +1)as i8, 
        (DISPLAY_HEIGHT - PLAYER_1.height - 1) as i8, // -1 for border
        &PLAYER_1
    )
}


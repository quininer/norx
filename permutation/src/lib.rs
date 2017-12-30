extern crate core;


pub mod portable;

pub use config::*;


#[cfg(feature = "config_6461")]
mod config {
    use core::mem;

    pub type W = u64;
    pub const L: usize = 6;
    pub const P: usize = 1;
    pub const T: usize = mem::size_of::<W>() * 4;
}

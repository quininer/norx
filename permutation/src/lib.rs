extern crate core;

pub mod portable;


use core::mem;
pub use config::*;
pub const T: usize = mem::size_of::<U>() * 4;


#[cfg(feature = "config_0841")]
mod config {
    pub type U = u8;
    pub const L: usize = 4;
    pub const P: usize = 1;
}

#[cfg(feature = "config_1641")]
mod config {
    pub type U = u16;
    pub const L: usize = 4;
    pub const P: usize = 1;
}

#[cfg(feature = "config_3241")]
mod config {
    pub type U = u32;
    pub const L: usize = 4;
    pub const P: usize = 1;
}

#[cfg(feature = "config_3261")]
mod config {
    pub type U = u32;
    pub const L: usize = 6;
    pub const P: usize = 1;
}

#[cfg(feature = "config_6441")]
mod config {
    pub type U = u64;
    pub const L: usize = 4;
    pub const P: usize = 1;
}

#[cfg(feature = "config_6444")]
mod config {
    pub type U = u64;
    pub const L: usize = 4;
    pub const P: usize = 4;
}

#[cfg(feature = "config_6461")]
mod config {
    pub type U = u64;
    pub const L: usize = 6;
    pub const P: usize = 1;
}


#[cfg(feature = "config_0841")]
mod rot_const {
    pub const R0: u32 = 1;
    pub const R1: u32 = 3;
    pub const R2: u32 = 5;
    pub const R3: u32 = 7;
}

#[cfg(feature = "config_1641")]
mod rot_const {
    pub const R0: u32 =  8;
    pub const R1: u32 = 11;
    pub const R2: u32 = 12;
    pub const R3: u32 = 15;
}

#[cfg(any(feature = "config_3241", feature = "config_3261"))]
mod rot_const {
    pub const R0: u32 =  8;
    pub const R1: u32 = 11;
    pub const R2: u32 = 16;
    pub const R3: u32 = 31;
}

#[cfg(any(
    feature = "config_6441",
    feature = "config_6444",
    feature = "config_6461"
))]
mod rot_const {
    pub const R0: u32 =  8;
    pub const R1: u32 = 19;
    pub const R2: u32 = 40;
    pub const R3: u32 = 63;
}

#![no_std]
#![cfg_attr(feature = "simd", feature(target_feature, cfg_target_feature, stdsimd))]

// FIXME `is_x86_feature_detected!` missing in `core`.
#[cfg(feature = "simd")]
#[macro_use]
extern crate std;

pub mod portable;

#[cfg(feature = "simd")]
#[cfg(feature = "W32")]
#[path = "x32_ssse3.rs"]
pub mod ssse3;

#[cfg(feature = "simd")]
#[cfg(feature = "W64")]
#[path = "x64_ssse3.rs"]
pub mod ssse3;

#[cfg(feature = "simd")]
#[cfg(feature = "W64")]
#[path = "x64_avx2.rs"]
pub mod avx2;

#[cfg(feature = "W8")]  pub type U = u8;
#[cfg(feature = "W16")] pub type U = u16;
#[cfg(feature = "W32")] pub type U = u32;
#[cfg(feature = "W64")] pub type U = u64;

#[cfg(feature = "L4")] pub const L: usize = 4;
#[cfg(feature = "L6")] pub const L: usize = 6;

pub const S: usize = 16;


#[inline]
pub fn norx(state: &mut [U; S]) {
    #[cfg(feature = "simd")]
    #[cfg(feature = "W64")]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        if is_x86_feature_detected!("avx2") {
            return avx2::norx(state);
        }
    }

    #[cfg(feature = "simd")]
    #[cfg(any(feature = "W32", feature = "W64"))]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        if is_x86_feature_detected!("ssse3") {
            return ssse3::norx(state);
        }
    }

    portable::norx(state)
}

#[inline]
pub fn norx_x4(state1: &mut [U; S], state2: &mut [U; S], state3: &mut [U; S], state4: &mut [U; S]) {
    #[cfg(feature = "simd")]
    #[cfg(feature = "W64")]
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    unsafe {
        if is_x86_feature_detected!("avx2") {
            return avx2::norx_x4(state1, state2, state3, state4);
        }
    }

    norx(state1);
    norx(state2);
    norx(state3);
    norx(state4);
}


#[cfg(feature = "W8")]
mod rot_const {
    pub const R0: u32 = 1;
    pub const R1: u32 = 3;
    pub const R2: u32 = 5;
    pub const R3: u32 = 7;
}

#[cfg(feature = "W16")]
mod rot_const {
    pub const R0: u32 =  8;
    pub const R1: u32 = 11;
    pub const R2: u32 = 12;
    pub const R3: u32 = 15;
}

#[cfg(feature = "W32")]
mod rot_const {
    pub const R0: u32 =  8;
    pub const R1: u32 = 11;
    pub const R2: u32 = 16;
    pub const R3: u32 = 31;
}

#[cfg(feature = "W64")]
mod rot_const {
    pub const R0: u32 =  8;
    pub const R1: u32 = 19;
    pub const R2: u32 = 40;
    pub const R3: u32 = 63;
}

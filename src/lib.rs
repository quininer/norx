extern crate core;
extern crate byteorder;
extern crate norx_permutation as permutation;

use core::mem;
use byteorder::LittleEndian;
pub use permutation::{ U, S };


const BLOCK_LENGTH: usize = mem::size_of::<U>() * S;

type State = [u8; BLOCK_LENGTH];

pub struct Norx {
    state: State,
    pos: usize
}

impl Norx {
    fn permutation(state: &mut State) {
        #[inline]
        fn array_as_block(arr: &mut State) -> &mut [U; S] {
            unsafe { mem::transmute(arr) }
        }

        #[inline]
        fn le_from_slice(arr: &mut [U; S]) {
            #[cfg(features = "W16")] LittleEndian::from_slice_u16(state);
            #[cfg(features = "W32")] LittleEndian::from_slice_u32(state);
            #[cfg(features = "W64")] LittleEndian::from_slice_u64(state);
        }

        let state = array_as_block(state);
        le_from_slice(state);
        permutation::norx(state);
        le_from_slice(state);
    }

    // TODO
}

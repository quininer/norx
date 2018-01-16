extern crate core;
extern crate byteorder;
extern crate norx_permutation as permutation;

use std::{ fs, env };
use std::io::Write;
use std::path::PathBuf;
use byteorder::{ LittleEndian, ByteOrder };
use permutation::{ U, S, L };

#[allow(dead_code)]
#[path = "src/constant.rs"]
mod constant;

use constant::*;

fn init() -> [u8; STATE_LENGTH] {
    let mut state_bytes = [0; STATE_LENGTH];
    let mut state = [0; S];

    for i in 0..S {
        state[i] = i as U;
    }

    permutation::portable::f(&mut state);
    permutation::portable::f(&mut state);

    state[12] ^= W as U;
    state[13] ^= L as U;
    state[14] ^= P as U;
    state[15] ^= T as U;

    #[cfg(feature = "W32")] LittleEndian::write_u32_into(&state, &mut state_bytes);
    #[cfg(feature = "W64")] LittleEndian::write_u64_into(&state, &mut state_bytes);

    state_bytes
}

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut p = fs::File::create(PathBuf::from(dir).join("src").join("init_state.rs")).unwrap();
    write!(p, "{:?}", &init()[..]).unwrap();
}

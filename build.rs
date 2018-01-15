extern crate byteorder;
extern crate norx_permutation as permutation;

use std::{ fs, env, mem };
use std::io::Write;
use std::path::PathBuf;
use byteorder::{ LittleEndian, ByteOrder };
use permutation::{ U, S };


const B: usize = mem::size_of::<U>() * S;

fn init() -> [u8; B] {
    let mut state_bytes = [0; B];
    let mut state = [0; S];

    for i in 0..S {
        state[i] = i as U;
    }

    permutation::norx(&mut state);
    permutation::norx(&mut state);

    #[cfg(feature = "W8")]  { state_bytes = state };
    #[cfg(feature = "W16")] LittleEndian::write_u16_into(&state, &mut state_bytes);
    #[cfg(feature = "W32")] LittleEndian::write_u32_into(&state, &mut state_bytes);
    #[cfg(feature = "W64")] LittleEndian::write_u64_into(&state, &mut state_bytes);

    state_bytes
}

fn main() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut p = fs::File::create(PathBuf::from(dir).join("src").join("constant.rs")).unwrap();
    write!(p, "{:?}", &init()[..]).unwrap();
}

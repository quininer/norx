extern crate core;
extern crate norx_permutation as permutation;

use std::{ fs, env };
use std::io::Write;
use std::path::PathBuf;
use permutation::{ U, S, L };

#[allow(dead_code)]
#[path = "src/constant.rs"]
mod constant;

use constant::*;

fn init() -> [U; S] {
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

    state
}

fn main() {
    let dir = env::var("OUT_DIR").unwrap();
    let mut p = fs::File::create(PathBuf::from(dir).join("init_state.rs")).unwrap();
    write!(p, "{:?}", &init()[..]).unwrap();
}

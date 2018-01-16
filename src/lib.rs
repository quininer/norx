#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(needless_range_loop))]

#[macro_use] extern crate arrayref;
extern crate subtle;
extern crate byteorder;
extern crate norx_permutation as permutation;

mod common;

#[cfg(feature = "P1")]
#[path = "p1.rs"]
mod imp;

pub mod constant;

use common::{ Tag, tags, with, absorb };
use constant::{ W, STATE_LENGTH, KEY_LENGTH, NONCE_LENGTH, TAG_LENGTH };
pub use imp::Process;


#[derive(Clone)]
pub struct Norx([u8; STATE_LENGTH]);
pub struct Encrypt;
pub struct Decrypt;

impl Norx {
    pub fn new(key: &[u8; KEY_LENGTH], nonce: &[u8; NONCE_LENGTH]) -> Norx {
        // TODO use CTFE https://github.com/rust-lang/rust/issues/24111
        let mut state = include!("init_state.rs");

        state[..NONCE_LENGTH].copy_from_slice(nonce);
        state[NONCE_LENGTH..][..KEY_LENGTH].copy_from_slice(key);

        with(&mut state, permutation::norx);

        for i in 0..KEY_LENGTH {
            state[(12 * W / 8)..][i] ^= key[i];
        }

        Norx(state)
    }

    fn finalize(self, key: &[u8; KEY_LENGTH], aad: &[u8], tag: &mut [u8; TAG_LENGTH]) {
        let Norx(mut state) = self;

        absorb::<tags::Trailer>(&mut state, aad);

        with(&mut state, |state| {
            state[15] ^= <tags::Final as Tag>::TAG;
            permutation::norx(state);
        });

        for i in 0..KEY_LENGTH {
            state[(12 * W / 8)..][i] ^= key[i];
        }

        with(&mut state, permutation::norx);

        for i in 0..KEY_LENGTH {
            tag[i] = state[(12 * W / 8)..][i] ^ key[i];
        }
    }
}

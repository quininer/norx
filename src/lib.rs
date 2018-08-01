#![no_std]
#![cfg_attr(feature = "cargo-clippy", allow(needless_range_loop))]

#[cfg(feature = "P4")]
#[macro_use] extern crate if_chain;
#[macro_use] extern crate arrayref;
extern crate seckey;
extern crate byteorder;
extern crate norx_permutation as permutation;

#[macro_use]
mod common;

#[cfg(feature = "P1")]
#[path = "p1.rs"]
mod imp;

#[cfg(feature = "P4")]
#[path = "p4.rs"]
mod imp;

pub mod constant;

use common::{ Tag, tags, with, absorb };
use constant::{ W, U, S, KEY_LENGTH, NONCE_LENGTH, TAG_LENGTH };
pub use imp::Process;


#[derive(Clone)]
pub struct Norx([U; S]);
pub struct Encrypt;
pub struct Decrypt;

impl Norx {
    pub fn new(key: &[u8; KEY_LENGTH], nonce: &[u8; NONCE_LENGTH]) -> Norx {
        // TODO use CTFE https://github.com/rust-lang/rust/issues/24111
        #![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
        let mut state = include!(concat!(env!("OUT_DIR"), "/", "init_state.rs"));

        with(&mut state, |state| {
            state[..NONCE_LENGTH].copy_from_slice(nonce);
            state[NONCE_LENGTH..][..KEY_LENGTH].copy_from_slice(key);
        });

        permutation::norx(&mut state);

        with(&mut state, |state| {
            for i in 0..KEY_LENGTH {
                state[(12 * W / 8)..][i] ^= key[i];
            }
        });

        Norx(state)
    }

    fn finalize(mut self, key: &[u8; KEY_LENGTH], aad: &[u8], tag: &mut [u8; TAG_LENGTH]) {
        let Norx(ref mut state) = self;

        absorb::<tags::Trailer>(state, aad);

        state[15] ^= <tags::Final as Tag>::TAG;
        permutation::norx(state);

        with(state, |state| {
            for i in 0..KEY_LENGTH {
                state[(12 * W / 8)..][i] ^= key[i];
            }
        });

        permutation::norx(state);

        with(state, |state| {
            for i in 0..KEY_LENGTH {
                tag[i] = state[(12 * W / 8)..][i] ^ key[i];
            }
        });

        #[cfg(feature = "zero")]
        seckey::zero(state);
    }
}

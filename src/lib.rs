#![allow(dead_code)]

extern crate core;

#[macro_use] extern crate arrayref;
extern crate subtle;
extern crate byteorder;
extern crate norx_permutation as permutation;

mod common;

#[cfg(feature = "P1")]
#[path = "p1.rs"]
mod imp;

use core::mem;
use common::{ Tag, tags, with, absorb };
pub use permutation::{ U, S, L };
pub use imp::Process;


#[cfg(feature = "P1")]
/// Parallelism degree
pub const P: usize = 1;

#[cfg(feature = "P4")]
/// Parallelism degree
pub const P: usize = 4;

/// Word size
pub const W: usize = mem::size_of::<U>();

/// Tag size
pub const T: usize = W * 4;

/// Nonce size
pub const N: usize = W * 4;

/// Key size
pub const K: usize = W * 4;

/// Permutation width
pub const B: usize = W * S;

/// Capacity
pub const C: usize = W * 4;

/// Rate
pub const R: usize = B - C;

#[derive(Clone)]
pub struct Norx([u8; B]);
pub struct Encrypt;
pub struct Decrypt;

impl Norx {
    pub fn new(key: &[u8; K], nonce: &[u8; N]) -> Norx {
        // TODO use CTFE https://github.com/rust-lang/rust/issues/24111
        let mut state = include!("constant.rs");

        state[..N].copy_from_slice(nonce);
        state[N..][..K].copy_from_slice(key);

        with(&mut state, |state| {
            state[12] = W as U;
            state[13] = L as U;
            state[14] = P as U;
            state[15] = T as U;

            permutation::norx(state);
        });

        for i in 0..K {
            state[N..][K..][i] ^= key[i];
        }

        Norx(state)
    }

    fn finalize(self, key: &[u8; K], aad: &[u8], tag: &mut [u8; T]) {
        let Norx(mut state) = self;

        absorb::<tags::Final>(&mut state, aad);

        with(&mut state, |state| {
            state[15] ^= <tags::Final as Tag>::TAG;

            permutation::norx(state);
        });

        for i in 0..K {
            state[N + K..][i] ^= key[i];
        }

        with(&mut state, permutation::norx);

        for i in 0..K {
            tag[i] = state[N + K..][i] ^ key[i];
        }
    }
}

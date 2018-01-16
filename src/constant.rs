use core::mem;
pub use permutation::{ U, S, L };


#[cfg(feature = "P1")]
/// Parallelism degree
pub const P: usize = 1;

#[cfg(feature = "P4")]
/// Parallelism degree
pub const P: usize = 4;

/// Word size
pub const W: usize = mem::size_of::<U>() * 8;

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

/// Key bytes length
pub const KEY_LENGTH: usize = K / 8;

/// Nonce bytes length
pub const NONCE_LENGTH: usize = N / 8;

/// Tag bytes length
pub const TAG_LENGTH: usize = T / 8;

/// State bytes length
pub const STATE_LENGTH: usize = B / 8;

/// Block bytes length (Rate)
pub const BLOCK_LENGTH: usize = R / 8;

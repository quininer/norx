#![cfg_attr(feature = "cargo-clippy", allow(too_many_arguments))]

use core::arch::x86_64::{
    _mm_xor_si128, _mm_add_epi64,
    _mm_and_si128, _mm_or_si128,
    _mm_shuffle_epi8, _mm_set_epi8,
    _mm_srli_epi64, _mm_slli_epi64,
    _mm_alignr_epi8
};
use packed_simd::{ FromBits, IntoBits, u64x2 };
use ::{ U, S, L };
use ::rot_const::*;



macro_rules! xor {
    ( $a:expr, $b:expr ) => {
        u64x2::from_bits(_mm_xor_si128($a.into_bits(), $b.into_bits()));
    }
}

macro_rules! and {
    ( $a:expr, $b:expr ) => {
        u64x2::from_bits(_mm_and_si128($a.into_bits(), $b.into_bits()))
    }
}

macro_rules! add {
    ( $a:expr, $b:expr ) => {
        u64x2::from_bits(_mm_add_epi64($a.into_bits(), $b.into_bits()))
    }
}

macro_rules! rot {
    ( $x:expr, R0 ) => {
        _mm_shuffle_epi8(
            $x.into_bits(),
            _mm_set_epi8( 8, 15, 14, 13, 12, 11, 10,  9, 0, 7, 6, 5, 4, 3, 2, 1).into_bits()
        ).into_bits()
    };
    ( $x:expr, R2 ) => {
        _mm_shuffle_epi8(
            $x.into_bits(),
            _mm_set_epi8(12, 11, 10,  9,  8, 15, 14, 13, 4, 3, 2, 1, 0, 7, 6, 5).into_bits()
        ).into_bits()
    };
    ( $x:expr, R3 ) => {
        _mm_or_si128(
            _mm_add_epi64($x.into_bits(), $x.into_bits()).into_bits(),
            _mm_srli_epi64($x.into_bits(), 63).into_bits()
        ).into_bits()
    };
    ( $x:expr, R1 ) => {
        _mm_or_si128(
            _mm_srli_epi64($x.into_bits(), R1 as i32).into_bits(),
            _mm_slli_epi64($x.into_bits(), 64 - R1 as i32).into_bits()
        ).into_bits()
    }
}


#[target_feature(enable = "ssse3")]
pub unsafe fn norx(state: &mut [U; S]) {
    unsafe fn f(state: &mut [u64x2; 8]) {
        macro_rules! EX {
            ( $f:ident ( $a0:expr, $a1:expr, $b0:expr, $b1:expr, $c0:expr, $c1:expr, $d0:expr, $d1:expr ) ) => {
                let (a0, a1, b0, b1, c0, c1, d0, d1) =
                    $f($a0, $a1, $b0, $b1, $c0, $c1, $d0, $d1);
                $a0 = a0; $a1 = a1; $b0 = b0; $b1 = b1;
                $c0 = c0; $c1 = c1; $d0 = d0; $d1 = d1;
            };
            ( $( $f:ident ( $( $a:expr ),+ ) );+ ; ) => {
                $( EX!( $f( $( $a ),+ ) ); )+
            };
        }

        EX!{
            g(state[0], state[1], state[2], state[3], state[4], state[5], state[6], state[7]);
            diagonalize(state[0], state[1], state[2], state[3], state[4], state[5], state[6], state[7]);
            g(state[0], state[1], state[2], state[3], state[4], state[5], state[6], state[7]);
            undiagonalize(state[0], state[1], state[2], state[3], state[4], state[5], state[6], state[7]);
        }
    }

    let mut s = [
        u64x2::from_slice_unaligned(&state[0..]),
        u64x2::from_slice_unaligned(&state[2..]),
        u64x2::from_slice_unaligned(&state[4..]),
        u64x2::from_slice_unaligned(&state[6..]),
        u64x2::from_slice_unaligned(&state[8..]),
        u64x2::from_slice_unaligned(&state[10..]),
        u64x2::from_slice_unaligned(&state[12..]),
        u64x2::from_slice_unaligned(&state[14..]),
    ];

    for _ in 0..L {
        f(&mut s);
    }

    s[0].write_to_slice_unaligned(&mut state[0..]);
    s[1].write_to_slice_unaligned(&mut state[2..]);
    s[2].write_to_slice_unaligned(&mut state[4..]);
    s[3].write_to_slice_unaligned(&mut state[6..]);
    s[4].write_to_slice_unaligned(&mut state[8..]);
    s[5].write_to_slice_unaligned(&mut state[10..]);
    s[6].write_to_slice_unaligned(&mut state[12..]);
    s[7].write_to_slice_unaligned(&mut state[14..]);
}


unsafe fn g(
    mut a0: u64x2, mut a1: u64x2, mut b0: u64x2, mut b1: u64x2,
    mut c0: u64x2, mut c1: u64x2, mut d0: u64x2, mut d1: u64x2
) -> (u64x2, u64x2, u64x2, u64x2, u64x2, u64x2,u64x2, u64x2) {
    let (mut l0, mut l1, mut r0, mut r1);

    l0 = xor!(a0, b0);    r0 = xor!(a1, b1);
    l1 = and!(a0, b0);    r1 = and!(a1, b1);
    l1 = add!(l1, l1);    r1 = add!(r1, r1);
    a0 = xor!(l0, l1);    a1 = xor!(r0, r1);
    d0 = xor!(d0, a0);    d1 = xor!(d1, a1);
    d0 = rot!(d0, R0);    d1 = rot!(d1, R0);

    l0 = xor!(c0, d0);    r0 = xor!(c1, d1);
    l1 = and!(c0, d0);    r1 = and!(c1, d1);
    l1 = add!(l1, l1);    r1 = add!(r1, r1);
    c0 = xor!(l0, l1);    c1 = xor!(r0, r1);
    b0 = xor!(b0, c0);    b1 = xor!(b1, c1);
    b0 = rot!(b0, R1);    b1 = rot!(b1, R1);

    l0 = xor!(a0, b0);    r0 = xor!(a1, b1);
    l1 = and!(a0, b0);    r1 = and!(a1, b1);
    l1 = add!(l1, l1);    r1 = add!(r1, r1);
    a0 = xor!(l0, l1);    a1 = xor!(r0, r1);
    d0 = xor!(d0, a0);    d1 = xor!(d1, a1);
    d0 = rot!(d0, R2);    d1 = rot!(d1, R2);

    l0 = xor!(c0, d0);    r0 = xor!(c1, d1);
    l1 = and!(c0, d0);    r1 = and!(c1, d1);
    l1 = add!(l1, l1);    r1 = add!(r1, r1);
    c0 = xor!(l0, l1);    c1 = xor!(r0, r1);
    b0 = xor!(b0, c0);    b1 = xor!(b1, c1);
    b0 = rot!(b0, R3);    b1 = rot!(b1, R3);

    (a0, a1, b0, b1, c0, c1, d0, d1)
}


unsafe fn diagonalize(
        a0: u64x2,     a1: u64x2, mut b0: u64x2, mut b1: u64x2,
    mut c0: u64x2, mut c1: u64x2, mut d0: u64x2, mut d1: u64x2
) -> (u64x2, u64x2, u64x2, u64x2, u64x2, u64x2,u64x2, u64x2) {
    let (mut t0, mut t1);

    t0 = _mm_alignr_epi8(b1.into_bits(), b0.into_bits(), 8).into_bits();
    t1 = _mm_alignr_epi8(b0.into_bits(), b1.into_bits(), 8).into_bits();
    b0 = t0;
    b1 = t1;

    t0 = c0;
    c0 = c1;
    c1 = t0;

    t0 = _mm_alignr_epi8(d1.into_bits(), d0.into_bits(), 8).into_bits();
    t1 = _mm_alignr_epi8(d0.into_bits(), d1.into_bits(), 8).into_bits();
    d0 = t1;
    d1 = t0;

    (a0, a1, b0, b1, c0, c1, d0, d1)
}

unsafe fn undiagonalize(
        a0: u64x2,     a1: u64x2, mut b0: u64x2, mut b1: u64x2,
    mut c0: u64x2, mut c1: u64x2, mut d0: u64x2, mut d1: u64x2
) -> (u64x2, u64x2, u64x2, u64x2, u64x2, u64x2,u64x2, u64x2) {
    let (mut t0, mut t1);

    t0 = _mm_alignr_epi8(b0.into_bits(), b1.into_bits(), 8).into_bits();
    t1 = _mm_alignr_epi8(b1.into_bits(), b0.into_bits(), 8).into_bits();
    b0 = t0;
    b1 = t1;

    t0 = c0;
    c0 = c1;
    c1 = t0;

    t0 = _mm_alignr_epi8(d0.into_bits(), d1.into_bits(), 8).into_bits();
    t1 = _mm_alignr_epi8(d1.into_bits(), d0.into_bits(), 8).into_bits();
    d0 = t1;
    d1 = t0;

    (a0, a1, b0, b1, c0, c1, d0, d1)
}

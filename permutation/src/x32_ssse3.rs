use core::arch::x86_64::{
    _mm_xor_si128, _mm_and_si128,
    _mm_add_epi32, _mm_or_si128,
    _mm_shuffle_epi8, _mm_shuffle_epi32,
    _mm_set_epi8,
    _mm_srli_epi32, _mm_slli_epi32
};
use packed_simd::{ FromBits, IntoBits, u32x4 };
use ::{ U, S, L };
use ::rot_const::*;


macro_rules! shuffle {
    ( $fp3:expr, $fp2:expr, $fp1:expr, $fp0:expr ) => {
        ($fp3 << 6) | ($fp2 << 4) | ($fp1 << 2) | $fp0
    }
}

macro_rules! xor {
    ( $a:expr, $b:expr ) => {
        u32x4::from_bits(_mm_xor_si128($a.into_bits(), $b.into_bits()));
    }
}

macro_rules! and {
    ( $a:expr, $b:expr ) => {
        u32x4::from_bits(_mm_and_si128($a.into_bits(), $b.into_bits()))
    }
}

macro_rules! add {
    ( $a:expr, $b:expr ) => {
        u32x4::from_bits(_mm_add_epi32($a.into_bits(), $b.into_bits()))
    }
}

macro_rules! rot {
    ( $x:expr, R0 ) => {
        _mm_shuffle_epi8(
            $x.into_bits(),
            _mm_set_epi8(12,15,14,13, 8,11,10, 9, 4,7,6,5, 0,3,2,1).into_bits()
        ).into_bits()
    };
    ( $x:expr, R2 ) => {
        _mm_shuffle_epi8(
            $x.into_bits(),
            _mm_set_epi8(13,12,15,14, 9, 8,11,10, 5,4,7,6, 1,0,3,2).into_bits()
        ).into_bits()
    };
    ( $x:expr, R3 ) => {
        _mm_or_si128(
            _mm_add_epi32($x.into_bits(), $x.into_bits()).into_bits(),
            _mm_srli_epi32($x.into_bits(), 31).into_bits()
        ).into_bits()
    };
    ( $x:expr, R1 ) => {
        _mm_or_si128(
            _mm_srli_epi32($x.into_bits(), R1 as i32).into_bits(),
            _mm_slli_epi32($x.into_bits(), 32 - R1 as i32).into_bits()
        ).into_bits()
    }
}


pub unsafe fn norx(state: &mut [U; S]) {
    unsafe fn f(state: &mut [u32x4; 4]) {
        macro_rules! EX {
            ( $f:ident ( $a:expr, $b:expr, $c:expr, $d:expr ) ) => {
                let (a, b, c, d) = $f($a, $b, $c, $d);
                $a = a; $b = b; $c = c; $d = d;
            };
            ( $( $f:ident ( $( $a:expr ),+ ) );+ ; ) => {
                $( EX!( $f( $( $a ),+ ) ); )+
            }
        }

        EX!{
            g(state[0], state[1], state[2], state[3]);
            diagonalize(state[0], state[1], state[2], state[3]);
            g(state[0], state[1], state[2], state[3]);
            undiagonalize(state[0], state[1], state[2], state[3]);
        }
    }

    let mut s = [
        u32x4::from_slice_unaligned(&state[0..]),
        u32x4::from_slice_unaligned(&state[4..]),
        u32x4::from_slice_unaligned(&state[8..]),
        u32x4::from_slice_unaligned(&state[12..]),
    ];

    for _ in 0..L {
        f(&mut s);
    }

    s[0].write_to_slice_unaligned(&mut state[0..]);
    s[1].write_to_slice_unaligned(&mut state[4..]);
    s[2].write_to_slice_unaligned(&mut state[8..]);
    s[3].write_to_slice_unaligned(&mut state[12..]);
}

unsafe fn g(mut a: u32x4, mut b: u32x4, mut c: u32x4, mut d: u32x4)
    -> (u32x4, u32x4, u32x4, u32x4)
{
    let (mut t0, mut t1);

    t0 = xor!( a,  b);
    t1 = and!( a,  b);
    t1 = add!(t1, t1);
     a = xor!(t0, t1);
     d = xor!( d, t0);
     d = xor!( d, t1);
     d = rot!( d, R0);

    t0 = xor!( c,  d);
    t1 = and!( c,  d);
    t1 = add!(t1, t1);
     c = xor!(t0, t1);
     b = xor!( b, t0);
     b = xor!( b, t1);
     b = rot!( b, R1);

    t0 = xor!( a,  b);
    t1 = and!( a,  b);
    t1 = add!(t1, t1);
     a = xor!(t0, t1);
     d = xor!( d, t0);
     d = xor!( d, t1);
     d = rot!( d, R2);

    t0 = xor!( c,  d);
    t1 = and!( c,  d);
    t1 = add!(t1, t1);
     c = xor!(t0, t1);
     b = xor!( b, t0);
     b = xor!( b, t1);
     b = rot!( b, R3);

     (a, b, c, d)
}

unsafe fn diagonalize(a: u32x4, mut b: u32x4, mut c: u32x4, mut d: u32x4) -> (u32x4, u32x4, u32x4, u32x4) {
    d = _mm_shuffle_epi32(d.into_bits(), shuffle!(2, 1, 0, 3)).into_bits();
    c = _mm_shuffle_epi32(c.into_bits(), shuffle!(1, 0, 3, 2)).into_bits();
    b = _mm_shuffle_epi32(b.into_bits(), shuffle!(0, 3, 2, 1)).into_bits();
    (a, b, c, d)
}

unsafe fn undiagonalize(a: u32x4, mut b: u32x4, mut c: u32x4, mut d: u32x4) -> (u32x4, u32x4, u32x4, u32x4) {
    d = _mm_shuffle_epi32(d.into_bits(), shuffle!(0, 3, 2, 1)).into_bits();
    c = _mm_shuffle_epi32(c.into_bits(), shuffle!(1, 0, 3, 2)).into_bits();
    b = _mm_shuffle_epi32(b.into_bits(), shuffle!(2, 1, 0, 3)).into_bits();
    (a, b, c, d)
}

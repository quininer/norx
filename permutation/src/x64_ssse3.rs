use coresimd::simd::u64x2;
use coresimd::vendor::{
    _mm_xor_si128, _mm_add_epi64,
    _mm_and_si128, _mm_or_si128,
    _mm_shuffle_epi8, _mm_set_epi8,
    _mm_srli_epi64, _mm_slli_epi64,
    _mm_alignr_epi8
};
use ::{ U, L };
use ::rot_const::*;


pub fn norx(state: &mut [U; 16]) {
    unsafe fn f(state: &mut [u64x2; 8]) {
        macro_rules! EX {
            ( $f:ident ( $a0:expr, $a1:expr, $b0:expr, $b1:expr, $c0:expr, $c1:expr, $d0:expr, $d1:expr ) ) => {
                let (a0, a1, b0, b1, c0, c1, d0, d1) =
                    $f($a0, $a1, $b0, $b1, $c0, $c1, $d0, $d1);
                $a0 = a0; $a1 = a1; $b0 = b0; $b1 = b1;
                $c0 = c0; $c1 = c1; $d0 = d0; $d1 = d1;
            };

            ( $(
                $f:ident ( $a0:expr, $a1:expr, $b0:expr, $b1:expr, $c0:expr, $c1:expr, $d0:expr, $d1:expr )
            );+ ) => {
                $( EX!($f($a0, $a1, $b0, $b1, $c0, $c1, $d0, $d1)); )+
            }
        }

        EX!{
            g(state[0], state[1], state[2], state[3], state[4], state[5], state[6], state[7]);
            diagonalize(state[0], state[1], state[2], state[3], state[4], state[5], state[6], state[7]);
            g(state[0], state[1], state[2], state[3], state[4], state[5], state[6], state[7]);
            undiagonalize(state[0], state[1], state[2], state[3], state[4], state[5], state[6], state[7])
        }
    }

    let mut s = [
        u64x2::load(state, 0),
        u64x2::load(state, 2),
        u64x2::load(state, 4),
        u64x2::load(state, 6),
        u64x2::load(state, 8),
        u64x2::load(state, 10),
        u64x2::load(state, 12),
        u64x2::load(state, 14),
    ];

    for _ in 0..L {
        unsafe { f(&mut s) };
    }

    s[0].store(state, 0);
    s[1].store(state, 2);
    s[2].store(state, 4);
    s[3].store(state, 6);
    s[4].store(state, 8);
    s[5].store(state, 10);
    s[6].store(state, 12);
    s[7].store(state, 14);
}


#[cfg(not(any(feature = "config_6441", feature = "config_6461")))]
unsafe fn g(
    mut a0: u64x2, mut a1: u64x2, mut b0: u64x2, mut b1: u64x2,
    mut c0: u64x2, mut c1: u64x2, mut d0: u64x2, mut d1: u64x2
) -> (u64x2, u64x2, u64x2, u64x2, u64x2, u64x2,u64x2, u64x2) {
    let (mut l0, mut l1, mut r0, mut r1);

    l0 = xor(a0, b0);    r0 = xor(a1, b1);
    l1 = and(a0, b0);    r1 = and(a1, b1);
    l1 = add(l1, l1);    r1 = add(r1, r1);
    a0 = xor(l0, l1);    a1 = xor(r0, r1);
    d0 = xor(d0, l0);    d1 = xor(d1, r0);
    d0 = xor(d0, l1);    d1 = xor(d1, r1);
    d0 = rot(d0, R0);    d1 = rot(d1, R0);

    l0 = xor(c0, d0);    r0 = xor(c1, d1);
    l1 = and(c0, d0);    r1 = and(c1, d1);
    l1 = add(l1, l1);    r1 = add(r1, r1);
    c0 = xor(l0, l1);    c1 = xor(r0, r1);
    b0 = xor(b0, l0);    b1 = xor(b1, r0);
    b0 = xor(b0, l1);    b1 = xor(b1, r1);
    b0 = rot(b0, R1);    b1 = rot(b1, R1);

    l0 = xor(a0, b0);    r0 = xor(a1, b1);
    l1 = and(a0, b0);    r1 = and(a1, b1);
    l1 = add(l1, l1);    r1 = add(r1, r1);
    a0 = xor(l0, l1);    a1 = xor(r0, r1);
    d0 = xor(d0, l0);    d1 = xor(d1, r0);
    d0 = xor(d0, l1);    d1 = xor(d1, r1);
    d0 = rot(d0, R2);    d1 = rot(d1, R2);

    l0 = xor(c0, d0);    r0 = xor(c1, d1);
    l1 = and(c0, d0);    r1 = and(c1, d1);
    l1 = add(l1, l1);    r1 = add(r1, r1);
    c0 = xor(l0, l1);    c1 = xor(r0, r1);
    b0 = xor(b0, l0);    b1 = xor(b1, r0);
    b0 = xor(b0, l1);    b1 = xor(b1, r1);
    b0 = rot(b0, R3);    b1 = rot(b1, R3);

    (a0, a1, b0, b1, c0, c1, d0, d1)
}

#[cfg(any(feature = "config_6441", feature = "config_6461"))]
unsafe fn g(
    mut a0: u64x2, mut a1: u64x2, mut b0: u64x2, mut b1: u64x2,
    mut c0: u64x2, mut c1: u64x2, mut d0: u64x2, mut d1: u64x2
) -> (u64x2, u64x2, u64x2, u64x2, u64x2, u64x2,u64x2, u64x2) {
    let (mut l0, mut l1, mut r0, mut r1);

    l0 = xor(a0, b0);    r0 = xor(a1, b1);
    l1 = and(a0, b0);    r1 = and(a1, b1);
    l1 = add(l1, l1);    r1 = add(r1, r1);
    a0 = xor(l0, l1);    a1 = xor(r0, r1);
    d0 = xor(d0, a0);    d1 = xor(d1, a1);
    d0 = rot(d0, R0);    d1 = rot(d1, R0);

    l0 = xor(c0, d0);    r0 = xor(c1, d1);
    l1 = and(c0, d0);    r1 = and(c1, d1);
    l1 = add(l1, l1);    r1 = add(r1, r1);
    c0 = xor(l0, l1);    c1 = xor(r0, r1);
    b0 = xor(b0, c0);    b1 = xor(b1, c1);
    b0 = rot(b0, R1);    b1 = rot(b1, R1);

    l0 = xor(a0, b0);    r0 = xor(a1, b1);
    l1 = and(a0, b0);    r1 = and(a1, b1);
    l1 = add(l1, l1);    r1 = add(r1, r1);
    a0 = xor(l0, l1);    a1 = xor(r0, r1);
    d0 = xor(d0, a0);    d1 = xor(d1, a1);
    d0 = rot(d0, R2);    d1 = rot(d1, R2);

    l0 = xor(c0, d0);    r0 = xor(c1, d1);
    l1 = and(c0, d0);    r1 = and(c1, d1);
    l1 = add(l1, l1);    r1 = add(r1, r1);
    c0 = xor(l0, l1);    c1 = xor(r0, r1);
    b0 = xor(b0, c0);    b1 = xor(b1, c1);
    b0 = rot(b0, R3);    b1 = rot(b1, R3);

    (a0, a1, b0, b1, c0, c1, d0, d1)
}


unsafe fn diagonalize(
        a0: u64x2,     a1: u64x2, mut b0: u64x2, mut b1: u64x2,
    mut c0: u64x2, mut c1: u64x2, mut d0: u64x2, mut d1: u64x2
) -> (u64x2, u64x2, u64x2, u64x2, u64x2, u64x2,u64x2, u64x2) {
    let (mut t0, mut t1);

    t0 = _mm_alignr_epi8(b1.into(), b0.into(), 8).into();
    t1 = _mm_alignr_epi8(b0.into(), b1.into(), 8).into();
    b0 = t0;
    b1 = t1;

    t0 = c0;
    c0 = c1;
    c1 = t0;

    t0 = _mm_alignr_epi8(d1.into(), d0.into(), 8).into();
    t1 = _mm_alignr_epi8(d0.into(), d1.into(), 8).into();
    d0 = t1;
    d1 = t0;

    (a0, a1, b0, b1, c0, c1, d0, d1)
}

unsafe fn undiagonalize(
        a0: u64x2,     a1: u64x2, mut b0: u64x2, mut b1: u64x2,
    mut c0: u64x2, mut c1: u64x2, mut d0: u64x2, mut d1: u64x2
) -> (u64x2, u64x2, u64x2, u64x2, u64x2, u64x2,u64x2, u64x2) {
    let (mut t0, mut t1);

    t0 = _mm_alignr_epi8(b0.into(), b1.into(), 8).into();
    t1 = _mm_alignr_epi8(b1.into(), b0.into(), 8).into();
    b0 = t0;
    b1 = t1;

    t0 = c0;
    c0 = c1;
    c1 = t0;

    t0 = _mm_alignr_epi8(d0.into(), d1.into(), 8).into();
    t1 = _mm_alignr_epi8(d1.into(), d0.into(), 8).into();
    d0 = t1;
    d1 = t0;

    (a0, a1, b0, b1, c0, c1, d0, d1)
}


#[inline]
unsafe fn xor(a: u64x2, b: u64x2) -> u64x2 {
    _mm_xor_si128(a.into(), b.into()).into()
}

#[inline]
unsafe fn and(a: u64x2, b: u64x2) -> u64x2 {
    _mm_and_si128(a.into(), b.into()).into()
}

#[inline]
unsafe fn add(a: u64x2, b: u64x2) -> u64x2 {
    _mm_add_epi64(a.into(), b.into()).into()
}

#[inline]
unsafe fn rot(x: u64x2, c: u32) -> u64x2 {
    match c {
         8 => _mm_shuffle_epi8(x.into(), _mm_set_epi8( 8, 15, 14, 13, 12, 11, 10,  9, 0, 7, 6, 5, 4, 3, 2, 1).into()).into(),
        40 => _mm_shuffle_epi8(x.into(), _mm_set_epi8(12, 11, 10,  9,  8, 15, 14, 13, 4, 3, 2, 1, 0, 7, 6, 5).into()).into(),
        63 => _mm_or_si128(_mm_add_epi64(x.into(), x.into()), _mm_srli_epi64(x.into(), 63)).into(),
        _ => _mm_or_si128(_mm_srli_epi64(x.into(), c as i32), _mm_slli_epi64(x.into(), 64 - c as i32)).into()
    }
}

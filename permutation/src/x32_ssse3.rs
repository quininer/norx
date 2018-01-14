use coresimd::simd::u32x4;
use coresimd::vendor::{
    _mm_xor_si128, _mm_and_si128,
    _mm_add_epi32, _mm_or_si128,
    _mm_shuffle_epi8, _mm_shuffle_epi32,
    _mm_set_epi8,
    _mm_srli_epi32, _mm_slli_epi32
};
use ::{ U, S, L };
use ::rot_const::*;


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
        u32x4::load(state, 0),
        u32x4::load(state, 4),
        u32x4::load(state, 8),
        u32x4::load(state, 12),
    ];

    for _ in 0..L {
        f(&mut s);
    }

    s[0].store(state, 0);
    s[1].store(state, 4);
    s[2].store(state, 8);
    s[3].store(state, 12);
}

unsafe fn g(mut a: u32x4, mut b: u32x4, mut c: u32x4, mut d: u32x4)
    -> (u32x4, u32x4, u32x4, u32x4)
{
    let (mut t0, mut t1);

    t0 = xor( a,  b);
    t1 = and( a,  b);
    t1 = add(t1, t1);
     a = xor(t0, t1);
     d = xor( d, t0);
     d = xor( d, t1);
     d = rot( d, R0);

    t0 = xor( c,  d);
    t1 = and( c,  d);
    t1 = add(t1, t1);
     c = xor(t0, t1);
     b = xor( b, t0);
     b = xor( b, t1);
     b = rot( b, R1);

    t0 = xor( a,  b);
    t1 = and( a,  b);
    t1 = add(t1, t1);
     a = xor(t0, t1);
     d = xor( d, t0);
     d = xor( d, t1);
     d = rot( d, R2);

    t0 = xor( c,  d);
    t1 = and( c,  d);
    t1 = add(t1, t1);
     c = xor(t0, t1);
     b = xor( b, t0);
     b = xor( b, t1);
     b = rot( b, R3);

     (a, b, c, d)
}

unsafe fn diagonalize(a: u32x4, mut b: u32x4, mut c: u32x4, mut d: u32x4) -> (u32x4, u32x4, u32x4, u32x4) {
    d = _mm_shuffle_epi32(d.into(), shuffle(2, 1, 0, 3)).into();
    c = _mm_shuffle_epi32(c.into(), shuffle(1, 0, 3, 2)).into();
    b = _mm_shuffle_epi32(b.into(), shuffle(0, 3, 2, 1)).into();
    (a, b, c, d)
}

unsafe fn undiagonalize(a: u32x4, mut b: u32x4, mut c: u32x4, mut d: u32x4) -> (u32x4, u32x4, u32x4, u32x4) {
    d = _mm_shuffle_epi32(d.into(), shuffle(0, 3, 2, 1)).into();
    c = _mm_shuffle_epi32(c.into(), shuffle(1, 0, 3, 2)).into();
    b = _mm_shuffle_epi32(b.into(), shuffle(2, 1, 0, 3)).into();
    (a, b, c, d)
}

#[inline]
unsafe fn xor(a: u32x4, b: u32x4) -> u32x4 {
    _mm_xor_si128(a.into(), b.into()).into()
}

#[inline]
unsafe fn and(a: u32x4, b: u32x4) -> u32x4 {
    _mm_and_si128(a.into(), b.into()).into()
}

#[inline]
unsafe fn add(a: u32x4, b: u32x4) -> u32x4 {
    _mm_add_epi32(a.into(), b.into()).into()
}

#[inline]
unsafe fn rot(x: u32x4, c: u32) -> u32x4 {
    match c {
         8 => _mm_shuffle_epi8(x.into(), _mm_set_epi8(12,15,14,13, 8,11,10, 9, 4,7,6,5, 0,3,2,1).into()).into(),
        16 => _mm_shuffle_epi8(x.into(), _mm_set_epi8(13,12,15,14, 9, 8,11,10, 5,4,7,6, 1,0,3,2).into()).into(),
        31 => _mm_or_si128(_mm_add_epi32(x.into(), x.into()).into(), _mm_srli_epi32(x.into(), 31).into()).into(),
         _ => _mm_or_si128(_mm_srli_epi32(x.into(), c as i32).into(), _mm_slli_epi32(x.into(), 32 - c as i32).into()).into()
    }
}

#[inline]
fn shuffle(fp3: i32, fp2: i32, fp1: i32, fp0: i32) -> i32 {
    (fp3 << 6) | (fp2 << 4) | (fp1 << 2) | fp0
}

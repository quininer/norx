use core::mem;
use byteorder::{ LittleEndian, ByteOrder };
use ::constant::{ U, S, STATE_LENGTH, BLOCK_LENGTH };
use ::permutation;


pub trait Tag {
    const TAG: U;
}

#[allow(dead_code)]
pub mod tags {
    use super::U;
    use super::Tag;

    macro_rules! tags {
        ( $( $( #[$attr:meta] )* $name:ident => $val:expr ),+ ) => {
            $(
                $( #[$attr] )*
                pub enum $name {}

                impl Tag for $name {
                    const TAG: U = $val;
                }
            )+
        }
    }

    tags!{
        Header => 0x01,
        Payload => 0x02,
        Trailer => 0x04,
        Final => 0x08,
        Branch => 0x10,
        Merge => 0x20
    }
}


#[inline]
pub fn with<F>(arr: &mut [u8; STATE_LENGTH], f: F)
    where F: FnOnce(&mut [U; S])
{
    #[inline]
    fn transmute(arr: &mut [u8; STATE_LENGTH]) -> &mut [U; S] {
        unsafe { mem::transmute(arr) }
//        unsafe { &mut *(arr as *mut [u8; STATE_LENGTH] as *mut [U; S]) }
    }

    #[inline]
    fn le_from_slice(arr: &mut [U; S]) {
        #[cfg(feature = "W16")] LittleEndian::from_slice_u16(arr);
        #[cfg(feature = "W32")] LittleEndian::from_slice_u32(arr);
        #[cfg(feature = "W64")] LittleEndian::from_slice_u64(arr);
    }

    let arr = transmute(arr);
    le_from_slice(arr);
    f(arr);
    le_from_slice(arr);
}

#[cfg(feature = "P4")]
#[inline]
pub fn with_x4<F>(
    p0: &mut [u8; STATE_LENGTH],
    p1: &mut [u8; STATE_LENGTH],
    p2: &mut [u8; STATE_LENGTH],
    p3: &mut [u8; STATE_LENGTH],
    f: F
)
    where F: FnOnce(&mut [U; S], &mut [U; S], &mut [U; S], &mut [U; S])
{
    with(p0, |p0| {
        with(p1, |p1| {
            with(p2, |p2| {
                with(p3, |p3| {
                    f(p0, p1, p2, p3);
                })
            })
        })
    })
}

#[inline]
pub fn pad(input: &[u8]) -> [u8; BLOCK_LENGTH] {
    assert!(input.len() < BLOCK_LENGTH);

    let mut output = [0; BLOCK_LENGTH];

    output[..input.len()].copy_from_slice(input);
    output[input.len()] = 0x01;
    output[BLOCK_LENGTH - 1] |= 0x80;

    output
}


pub fn absorb<T: Tag>(state: &mut [u8; STATE_LENGTH], aad: &[u8]) {
    #[inline]
    fn absorb_block<T: Tag>(state: &mut [u8; STATE_LENGTH], chunk: &[u8; BLOCK_LENGTH]) {
        with(state, |state| {
            state[15] ^= T::TAG;
            permutation::norx(state);
        });

        for i in 0..BLOCK_LENGTH {
            state[i] ^= chunk[i];
        }
    }

    if aad.is_empty() { return () };

    let (aad, remaining) = aad.split_at(aad.len() - aad.len() % BLOCK_LENGTH);

    for chunk in aad.chunks(BLOCK_LENGTH) {
        let chunk = array_ref!(chunk, 0, BLOCK_LENGTH);
        absorb_block::<T>(state, chunk);
    }

    let chunk = pad(remaining);
    absorb_block::<T>(state, &chunk);
}


#[cfg(feature = "P4")]
#[cfg_attr(feature = "cargo-clippy", allow(clone_on_copy))]
pub fn branch_x4(state: &[u8; STATE_LENGTH])
    -> ([u8; STATE_LENGTH], [u8; STATE_LENGTH], [u8; STATE_LENGTH], [u8; STATE_LENGTH])
{
    use ::constant::{ R, W };
    const CAPACITY: usize = R / W;

    let (mut p0, mut p1, mut p2, mut p3) =
        (state.clone(), state.clone(), state.clone(), state.clone());

    with_x4(&mut p0, &mut p1, &mut p2, &mut p3, |p0, p1, p2, p3| {
        p0[15] ^= tags::Branch::TAG;
        p1[15] ^= tags::Branch::TAG;
        p2[15] ^= tags::Branch::TAG;
        p3[15] ^= tags::Branch::TAG;

        permutation::norx_x4(p0, p1, p2, p3);

        for i in 0..CAPACITY {
            p0[i] ^= 0;
            p1[i] ^= 1;
            p2[i] ^= 2;
            p3[i] ^= 3;
        }
    });

    (p0, p1, p2, p3)
}

#[cfg(feature = "P4")]
pub fn merge_x4(
    state: &mut [u8; STATE_LENGTH],
    p0: &mut [u8; STATE_LENGTH],
    p1: &mut [u8; STATE_LENGTH],
    p2: &mut [u8; STATE_LENGTH],
    p3: &mut [u8; STATE_LENGTH],
) {
    *state = [0; STATE_LENGTH];

    with_x4(p0, p1, p2, p3, |p0, p1, p2, p3| {
        p0[15] ^= tags::Merge::TAG;
        p1[15] ^= tags::Merge::TAG;
        p2[15] ^= tags::Merge::TAG;
        p3[15] ^= tags::Merge::TAG;

        permutation::norx_x4(p0, p1, p2, p3);

        with(state, |state| for i in 0..S {
            state[i] = p0[i] ^ p1[i] ^ p2[i] ^ p3[i];
        })
    })
}

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
    fn array_as_block(arr: &mut [u8; STATE_LENGTH]) -> &mut [U; S] {
        unsafe { mem::transmute(arr) }
    }

    #[inline]
    fn le_from_slice(arr: &mut [U; S]) {
        #[cfg(feature = "W16")] LittleEndian::from_slice_u16(arr);
        #[cfg(feature = "W32")] LittleEndian::from_slice_u32(arr);
        #[cfg(feature = "W64")] LittleEndian::from_slice_u64(arr);
    }

    let arr = array_as_block(arr);
    le_from_slice(arr);
    f(arr);
    le_from_slice(arr);
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
    fn absort_block<T: Tag>(state: &mut [u8; STATE_LENGTH], chunk: &[u8; BLOCK_LENGTH]) {
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
        absort_block::<T>(state, chunk);
    }

    let chunk = pad(remaining);
    absort_block::<T>(state, &chunk);
}

#[cfg(feature = "P4")]
pub fn branch(state: &mut [u8; STATE_LENGTH], lane: U) {
    use ::constant::{ R, W };

    const CAPACITY: usize = R / W;

    with(state, |state| {
        state[15] ^= tags::Branch::TAG;
        permutation::norx(state);

        for i in 0..CAPACITY {
            state[i] ^= lane;
        }
    });
}

#[cfg(feature = "P4")]
pub fn merge(state: &mut [u8; STATE_LENGTH], state1: &mut [u8; STATE_LENGTH]) {
    with(state, |state| with(state1, |state1| {
        state1[15] ^= tags::Merge::TAG;
        permutation::norx(state1);

        for i in 0..S {
            state[i] ^= state1[i];
        }
    }));
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

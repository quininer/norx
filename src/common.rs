use core::mem;
use byteorder::{ LittleEndian, ByteOrder };
use ::{ permutation, B, U, S, R };


pub trait Tag {
    const TAG: U;
}

#[allow(dead_code)]
pub mod tags {
    use ::U;
    use super::Tag;

    macro_rules! tags {
        ( $( $( #[$attr:meta] )* $name:ident => $val:expr ),+ ) => {
            $(
                $( #[$attr] )*
                pub struct $name;

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


pub fn with<F>(arr: &mut [u8; B], f: F)
    where F: FnOnce(&mut [U; S])
{
    #[inline]
    fn array_as_block(arr: &mut [u8; B]) -> &mut [U; S] {
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
pub fn pad(input: &[u8]) -> [u8; R] {
    assert!(input.len() < R);

    let mut output = [0; R];

    output[..input.len()].copy_from_slice(input);
    output[input.len()] = 0x01;
    output[R - 1] |= 0x80;

    output
}


pub fn absorb<T: Tag>(state: &mut [u8; B], aad: &[u8]) {
    #[inline]
    fn absort_block<T: Tag>(state: &mut [u8; B], chunk: &[u8; R]) {
        with(state, |state| {
            state[15] ^= T::TAG;
            permutation::norx(state);
        });

        for i in 0..R {
            state[i] ^= chunk[i];
        }
    }

    let (aad, remaining) = aad.split_at(aad.len() - aad.len() % R);

    for chunk in aad.chunks(R) {
        let chunk = array_ref!(chunk, 0, R);
        absort_block::<T>(state, chunk);
    }

    let chunk = pad(remaining);
    absort_block::<T>(state, &chunk);
}

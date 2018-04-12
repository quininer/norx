// TODO

use subtle::slices_equal;
use ::common::{ Tag, tags, with, pad, absorb, branch, merge };
use ::constant::{ U, P, STATE_LENGTH, BLOCK_LENGTH, KEY_LENGTH, TAG_LENGTH };
use ::{ permutation, Norx, Encrypt, Decrypt };


type Lane = [u8; STATE_LENGTH];

pub struct Process<Mode> {
    state: [u8; STATE_LENGTH],
    lane: (Lane, Lane, Lane, Lane),
    index: u8,
    started: bool,
    _mode: Mode
}

macro_rules! branch {
    ( $state:expr, $i:expr ) => {{
        let mut lane = $state.clone();
        branch(&mut lane, $i as U);
        lane
    }}
}

impl Norx {
    pub fn encrypt(self, aad: &[u8]) -> Process<Encrypt> {
        let Norx(mut state) = self;
        absorb::<tags::Header>(&mut state, aad);

        let lane = (
            branch!(state, 0), branch!(state, 1),
            branch!(state, 2), branch!(state, 3)
        );

        Process {
            state, lane,
            index: 0, started: false,
            _mode: Encrypt
        }
    }

    pub fn decrypt(self, aad: &[u8]) -> Process<Decrypt> {
        let Norx(mut state) = self;
        absorb::<tags::Header>(&mut state, aad);

        let lane = (
            branch!(state, 0), branch!(state, 1),
            branch!(state, 2), branch!(state, 3)
        );

        Process {
            state, lane,
            index: 0, started: false,
            _mode: Decrypt
        }
    }
}

impl<T> Process<T> {
    #[inline]
    fn current(&mut self) -> (&mut Lane, &mut Lane, &mut Lane, &mut Lane) {
        macro_rules! current {
            ( $p0:tt, $p1:tt, $p2:tt, $p3:tt ) => {
                (
                    &mut self.lane.$p0, &mut self.lane.$p1,
                    &mut self.lane.$p2, &mut self.lane.$p3
                )
            }
        }

        match self.index % 4 {
            0 => current!(0, 1, 2, 3),
            1 => current!(1, 2, 3, 0),
            2 => current!(2, 3, 0, 1),
            3 => current!(3, 0, 1, 2),
            _ => unreachable!()
        }
    }
}

impl Process<Encrypt> {
    pub fn process<'a, I>(&mut self, bufs: I)
        where I:
            Iterator<Item = (&'a [u8; BLOCK_LENGTH], &'a mut [u8; BLOCK_LENGTH])> +
            ExactSizeIterator
    {
        unimplemented!()
    }

    pub fn finalize(mut self, key: &[u8; KEY_LENGTH], aad: &[u8], input: &[u8], output: &mut [u8]) {
        unimplemented!()
    }
}

impl Process<Decrypt> {
    pub fn process<'a, I>(&mut self, bufs: I)
        where I:
            Iterator<Item = (&'a [u8; BLOCK_LENGTH], &'a mut [u8; BLOCK_LENGTH])> +
            ExactSizeIterator
    {
        unimplemented!()
    }

    pub fn finalize(mut self, key: &[u8; KEY_LENGTH], aad: &[u8], input: &[u8], output: &mut [u8]) -> bool {
        unimplemented!()
    }
}

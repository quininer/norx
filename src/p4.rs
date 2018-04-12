// TODO

use subtle::slices_equal;
use ::common::{ Tag, tags, with, with_x4, pad, absorb, branch, merge };
use ::constant::{ U, S, P, STATE_LENGTH, BLOCK_LENGTH, KEY_LENGTH, TAG_LENGTH };
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

        let mut lane = (
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
    pub fn process<'a, I>(&mut self, mut bufs: I)
        where I: Iterator<Item = (&'a [u8; BLOCK_LENGTH], &'a mut [u8; BLOCK_LENGTH])>
    {
        let mut buff = [None, None, None, None];
        let mut index = 0;

        for next in bufs {
            self.started = true;

            buff[index] = Some(next);
            index += 1;

            if_chain!{
                if index == 4;
                if let Some((input0, output0)) = buff[0].take();
                if let Some((input1, output1)) = buff[1].take();
                if let Some((input2, output2)) = buff[2].take();
                if let Some((input3, output3)) = buff[3].take();
                then {
                    index = 0;
                    let (p0, p1, p2, p3) = self.current();

                    with_x4(p0, p1, p2, p3, |p0, p1, p2, p3| {
                        p0[15] ^= <tags::Payload as Tag>::TAG;
                        p1[15] ^= <tags::Payload as Tag>::TAG;
                        p2[15] ^= <tags::Payload as Tag>::TAG;
                        p3[15] ^= <tags::Payload as Tag>::TAG;

                        permutation::norx_x4(p0, p1, p2, p3);
                    });

                    xor!(p0, input0, BLOCK_LENGTH);
                    xor!(p1, input1, BLOCK_LENGTH);
                    xor!(p2, input2, BLOCK_LENGTH);
                    xor!(p3, input3, BLOCK_LENGTH);

                    output0.copy_from_slice(&p0[..BLOCK_LENGTH]);
                    output1.copy_from_slice(&p1[..BLOCK_LENGTH]);
                    output2.copy_from_slice(&p2[..BLOCK_LENGTH]);
                    output3.copy_from_slice(&p3[..BLOCK_LENGTH]);
                }
            }
        }

        // use `into_iter()`
        // https://github.com/rust-lang/rust/pull/49000
        for (input, output) in buff.iter_mut()
            .filter_map(|buf| buf.take())
            .fuse()
        {
            {
                let (state, ..) = self.current();

                with(state, |state| {
                    state[15] ^= <tags::Payload as Tag>::TAG;
                    permutation::norx(state);
                });

                xor!(state, input, BLOCK_LENGTH);
                output.copy_from_slice(&state[..BLOCK_LENGTH]);
            }

            self.index += 1;
            self.index %= 4;
        }
    }

    pub fn finalize(mut self, key: &[u8; KEY_LENGTH], aad2: &[u8], input: &[u8], output: &mut [u8]) {
        assert!(input.len() < BLOCK_LENGTH);
        assert_eq!(input.len() + TAG_LENGTH, output.len());

        let (output, tag) = output.split_at_mut(input.len());
        let tag = array_mut_ref!(tag, 0, TAG_LENGTH);

        if self.started || !input.is_empty() {
            self.started = true;

            let (state, ..) = self.current();
            let input_pad = pad(input);
            with(state, |state| {
                state[15] ^= <tags::Payload as Tag>::TAG;
                permutation::norx(state);
            });
            xor!(state, input_pad, BLOCK_LENGTH);
            output.copy_from_slice(&state[..input.len()]);
        }

        if self.started {
            self.state = [0; STATE_LENGTH];
            merge(&mut self.state, &mut self.lane.0);
            merge(&mut self.state, &mut self.lane.1);
            merge(&mut self.state, &mut self.lane.2);
            merge(&mut self.state, &mut self.lane.3);
        }

        Norx(self.state).finalize(key, aad2, tag);
    }
}

impl Process<Decrypt> {
    pub fn process<'a, I>(&mut self, bufs: I)
        where I: Iterator<Item = (&'a [u8; BLOCK_LENGTH], &'a mut [u8; BLOCK_LENGTH])>
    {
        unimplemented!()
    }

    pub fn finalize(mut self, key: &[u8; KEY_LENGTH], aad2: &[u8], input: &[u8], output: &mut [u8]) -> bool {
        unimplemented!()
    }
}

use subtle::ConstantTimeEq;
use ::common::{ Tag, tags, with, with_x4, pad, absorb, branch_x4, merge_x4 };
use ::constant::{ STATE_LENGTH, BLOCK_LENGTH, KEY_LENGTH, TAG_LENGTH };
use ::{ permutation, Norx, Encrypt, Decrypt };


type Lane = [u8; STATE_LENGTH];

pub struct Process<Mode> {
    state: [u8; STATE_LENGTH],
    lane: (Lane, Lane, Lane, Lane),
    index: u8,
    started: bool,
    _mode: Mode
}

impl Norx {
    pub fn encrypt(self, aad: &[u8]) -> Process<Encrypt> {
        let Norx(mut state) = self;
        absorb::<tags::Header>(&mut state, aad);

        let lane = branch_x4(&state);

        Process {
            state, lane,
            index: 0, started: false,
            _mode: Encrypt
        }
    }

    pub fn decrypt(self, aad: &[u8]) -> Process<Decrypt> {
        let Norx(mut state) = self;
        absorb::<tags::Header>(&mut state, aad);

        let lane = branch_x4(&state);

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

                    for i in 0..BLOCK_LENGTH {
                        p0[i] ^= input0[i];
                        p1[i] ^= input1[i];
                        p2[i] ^= input2[i];
                        p3[i] ^= input3[i];
                    }

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

                for i in 0..BLOCK_LENGTH {
                    state[i] ^= input[i];
                }
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

            for i in 0..BLOCK_LENGTH {
                state[i] ^= input_pad[i];
            }
            output.copy_from_slice(&state[..input.len()]);
        }

        if self.started {
            merge_x4(
                &mut self.state,
                &mut self.lane.0,
                &mut self.lane.1,
                &mut self.lane.2,
                &mut self.lane.3
            );

            // TODO zero lane
        }

        Norx(self.state).finalize(key, aad2, tag);
    }
}

impl Process<Decrypt> {
    pub fn process<'a, I>(&mut self, bufs: I)
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

                    for i in 0..BLOCK_LENGTH {
                        output0[i] = p0[i] ^ input0[i];
                        output1[i] = p1[i] ^ input1[i];
                        output2[i] = p2[i] ^ input2[i];
                        output3[i] = p3[i] ^ input3[i];
                    }

                    p0[..BLOCK_LENGTH].copy_from_slice(input0);
                    p1[..BLOCK_LENGTH].copy_from_slice(input1);
                    p2[..BLOCK_LENGTH].copy_from_slice(input2);
                    p3[..BLOCK_LENGTH].copy_from_slice(input3);
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
                for i in 0..BLOCK_LENGTH {
                    output[i] = state[i] ^ input[i];
                }
                state[..BLOCK_LENGTH].copy_from_slice(input);
            }

            self.index += 1;
            self.index %= 4;
        }
    }

    pub fn finalize(mut self, key: &[u8; KEY_LENGTH], aad2: &[u8], input: &[u8], output: &mut [u8]) -> bool {
        assert!(output.len() < BLOCK_LENGTH);
        assert_eq!(input.len(), output.len() + TAG_LENGTH);

        let (input, tag) = input.split_at(output.len());

        if self.started || !output.is_empty() {
            self.started = true;

            let mut lastblock = [0; BLOCK_LENGTH];
            let (state, ..) = self.current();

            with(state, |state| {
                state[15] ^= <tags::Payload as Tag>::TAG;
                permutation::norx(state);
            });

            lastblock.copy_from_slice(&state[..BLOCK_LENGTH]);
            lastblock[..input.len()].copy_from_slice(input);
            lastblock[input.len()] ^= 0x01;
            lastblock[BLOCK_LENGTH - 1] ^= 0x80;

            for i in 0..input.len() {
                output[i] = state[i] ^ lastblock[i];
            }
            state[..BLOCK_LENGTH].copy_from_slice(&lastblock);
        }

        if self.started {
            merge_x4(
                &mut self.state,
                &mut self.lane.0,
                &mut self.lane.1,
                &mut self.lane.2,
                &mut self.lane.3
            );

            // TODO zero lane
        }

        let mut tag2 = [0; TAG_LENGTH];
        Norx(self.state).finalize(key, aad2, &mut tag2);

        tag.ct_eq(&tag2).unwrap_u8() == 1
    }
}

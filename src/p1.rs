use subtle::slices_equal;
use ::common::{ Tag, tags, with, pad, absorb };
use ::constant::{ STATE_LENGTH, BLOCK_LENGTH, KEY_LENGTH, TAG_LENGTH };
use ::{ permutation, Norx, Encrypt, Decrypt };


pub struct Process<Mode> {
    state: [u8; STATE_LENGTH],
    started: bool,
    _mode: Mode
}

impl Norx {
    pub fn encrypt(self, aad: &[u8]) -> Process<Encrypt> {
        let Norx(mut state) = self;
        absorb::<tags::Header>(&mut state, aad);

        Process { state, started: false, _mode: Encrypt }
    }

    pub fn decrypt(self, aad: &[u8]) -> Process<Decrypt> {
        let Norx(mut state) = self;
        absorb::<tags::Header>(&mut state, aad);

        Process { state, started: false, _mode: Decrypt }
    }
}

impl Process<Encrypt> {
    pub fn process<'a, I>(&mut self, bufs: I)
        where I: Iterator<Item = (&'a [u8; BLOCK_LENGTH], &'a mut [u8; BLOCK_LENGTH])>
    {
        for (input, output) in bufs {
            self.started = true;

            with(&mut self.state, |state| {
                state[15] ^= <tags::Payload as Tag>::TAG;
                permutation::norx(state);
            });

            for i in 0..BLOCK_LENGTH {
                self.state[i] ^= input[i];
            }

            output.copy_from_slice(&self.state[..BLOCK_LENGTH]);
        }
    }

    pub fn finalize(mut self, key: &[u8; KEY_LENGTH], aad: &[u8], input: &[u8], output: &mut [u8]) {
        assert!(input.len() < BLOCK_LENGTH);
        assert_eq!(input.len() + TAG_LENGTH, output.len());

        let (output, tag) = output.split_at_mut(input.len());
        let tag = array_mut_ref!(tag, 0, TAG_LENGTH);

        if self.started || !input.is_empty() {
            let input_pad = pad(input);
            with(&mut self.state, |state| {
                state[15] ^= <tags::Payload as Tag>::TAG;
                permutation::norx(state);
            });
            for i in 0..BLOCK_LENGTH {
                self.state[i] ^= input_pad[i];
            }
            output.copy_from_slice(&self.state[..input.len()]);
        }

        Norx(self.state).finalize(key, aad, tag);
    }
}

impl Process<Decrypt> {
    pub fn process<'a, I>(&mut self, bufs: I)
        where I: Iterator<Item = (&'a [u8; BLOCK_LENGTH], &'a mut [u8; BLOCK_LENGTH])>
    {
        for (input, output) in bufs {
            self.started = true;

            with(&mut self.state, |state| {
                state[15] ^= <tags::Payload as Tag>::TAG;
                permutation::norx(state);
            });

            for i in 0..BLOCK_LENGTH {
                output[i] = self.state[i] ^ input[i];
                self.state[i] = input[i];
            }
        }
    }

    pub fn finalize(mut self, key: &[u8; KEY_LENGTH], aad: &[u8], input: &[u8], output: &mut [u8]) -> bool {
        assert!(output.len() < BLOCK_LENGTH);
        assert_eq!(input.len(), output.len() + TAG_LENGTH);

        let (input, tag) = input.split_at(output.len());

        if self.started || !output.is_empty() {
            let mut lastblock = [0; BLOCK_LENGTH];

            with(&mut self.state, |state| {
                state[15] ^= <tags::Payload as Tag>::TAG;
                permutation::norx(state);
            });

            lastblock.copy_from_slice(&self.state[..BLOCK_LENGTH]);
            lastblock[..input.len()].copy_from_slice(input);
            lastblock[input.len()] ^= 0x01;
            lastblock[BLOCK_LENGTH - 1] ^= 0x80;

            for i in 0..input.len() {
                output[i] = self.state[i] ^ lastblock[i];
            }
            self.state[..BLOCK_LENGTH].copy_from_slice(&lastblock);
        }

        let mut tag2 = [0; TAG_LENGTH];
        Norx(self.state).finalize(key, aad, &mut tag2);

        slices_equal(tag, &tag2) == 1
    }
}

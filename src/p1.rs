use subtle::ConstantTimeEq;
use ::common::{ Tag, tags, with, pad, absorb };
use ::constant::{ U, S, BLOCK_LENGTH, KEY_LENGTH, TAG_LENGTH };
use ::{ permutation, Norx, Encrypt, Decrypt };


pub struct Process<Mode> {
    state: [U; S],
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

            self.state[15] ^= <tags::Payload as Tag>::TAG;
            permutation::norx(&mut self.state);

            with(&mut self.state, |state| {
                for i in 0..BLOCK_LENGTH {
                    state[i] ^= input[i];
                }
                output.copy_from_slice(&state[..BLOCK_LENGTH]);
            });
        }
    }

    pub fn finalize(mut self, key: &[u8; KEY_LENGTH], aad2: &[u8], input: &[u8], output: &mut [u8]) {
        assert!(input.len() < BLOCK_LENGTH);
        assert_eq!(input.len() + TAG_LENGTH, output.len());

        let (output, tag) = output.split_at_mut(input.len());
        let tag = array_mut_ref!(tag, 0, TAG_LENGTH);

        if self.started || !input.is_empty() {
            let input_pad = pad(input);

            self.state[15] ^= <tags::Payload as Tag>::TAG;
            permutation::norx(&mut self.state);

            with(&mut self.state, |state| {
                for i in 0..BLOCK_LENGTH {
                    state[i] ^= input_pad[i];
                }
                output.copy_from_slice(&state[..input.len()]);
            });
        }

        Norx(self.state).finalize(key, aad2, tag);
    }
}

impl Process<Decrypt> {
    pub fn process<'a, I>(&mut self, bufs: I)
        where I: Iterator<Item = (&'a [u8; BLOCK_LENGTH], &'a mut [u8; BLOCK_LENGTH])>
    {
        for (input, output) in bufs {
            self.started = true;

            self.state[15] ^= <tags::Payload as Tag>::TAG;
            permutation::norx(&mut self.state);

            with(&mut self.state, |state| {
                for i in 0..BLOCK_LENGTH {
                    output[i] = state[i] ^ input[i];
                }
                state[..BLOCK_LENGTH].copy_from_slice(input);
            });
        }
    }

    pub fn finalize(mut self, key: &[u8; KEY_LENGTH], aad2: &[u8], input: &[u8], output: &mut [u8]) -> bool {
        assert!(output.len() < BLOCK_LENGTH);
        assert_eq!(input.len(), output.len() + TAG_LENGTH);

        let (input, tag) = input.split_at(output.len());

        if self.started || !output.is_empty() {
            let mut lastblock = [0; BLOCK_LENGTH];

            self.state[15] ^= <tags::Payload as Tag>::TAG;
            permutation::norx(&mut self.state);

            with(&mut self.state, |state| {
                lastblock[..input.len()].copy_from_slice(input);
                lastblock[input.len()..].copy_from_slice(&state[..BLOCK_LENGTH][input.len()..]);
                lastblock[input.len()] ^= 0x01;
                lastblock[BLOCK_LENGTH - 1] ^= 0x80;

                for i in 0..input.len() {
                    output[i] = state[i] ^ lastblock[i];
                }
                state[..BLOCK_LENGTH].copy_from_slice(&lastblock);
            });
        }

        let mut tag2 = [0; TAG_LENGTH];
        Norx(self.state).finalize(key, aad2, &mut tag2);

        tag.ct_eq(&tag2).unwrap_u8() == 1
    }
}

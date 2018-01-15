use subtle::slices_equal;
use ::common::{ Tag, tags, with, pad, absorb };
use ::{
    permutation,
    Norx, Encrypt, Decrypt,
    K, B, T, R
};


pub struct Process<Mode> {
    state: [u8; B],
    _mode: Mode
}

impl Norx {
    pub fn encrypt(self, aad: &[u8]) -> Process<Encrypt> {
        let Norx(mut state) = self;
        absorb::<tags::Header>(&mut state, aad);

        Process { state, _mode: Encrypt }
    }

    pub fn decrypt(self, aad: &[u8]) -> Process<Decrypt> {
        let Norx(mut state) = self;
        absorb::<tags::Header>(&mut state, aad);

        Process { state, _mode: Decrypt }
    }
}

impl Process<Encrypt> {
    pub fn process<'a, I>(&mut self, bufs: I)
        where I: Iterator<Item = (&'a [u8; R], &'a mut [u8; R])>
    {
        for (input, output) in bufs {
            with(&mut self.state, |state| {
                state[15] ^= <tags::Payload as Tag>::TAG;

                permutation::norx(state);
            });

            for i in 0..R {
                self.state[i] ^= input[i];
            }

            output.copy_from_slice(&self.state[..R]);
        }
    }

    pub fn finalize(mut self, key: &[u8; K], aad: &[u8], input: &[u8], output: &mut [u8]) {
        assert!(input.len() < R);
        assert_eq!(input.len() + T, output.len());

        let (output, tag) = output.split_at_mut(input.len());
        let tag = array_mut_ref!(tag, 0, T);

        let input_pad = pad(input);
        with(&mut self.state, |state| {
            state[15] ^= <tags::Payload as Tag>::TAG;
            permutation::norx(state);
        });
        for i in 0..R {
            self.state[i] ^= input_pad[i];
        }
        output.copy_from_slice(&self.state[..input.len()]);

        Norx(self.state).finalize(key, aad, tag);
    }
}

impl Process<Decrypt> {
    pub fn process<'a, I>(&mut self, bufs: I)
        where I: Iterator<Item = (&'a [u8; R], &'a mut [u8; R])>
    {
        for (input, output) in bufs {
            with(&mut self.state, |state| {
                state[15] ^= <tags::Payload as Tag>::TAG;

                permutation::norx(state);
            });

            for i in 0..R {
                output[i] = self.state[i] ^ input[i];
                self.state[i] = input[i];
            }
        }
    }

    pub fn finalize(mut self, key: &[u8; K], aad: &[u8], input: &[u8], output: &mut [u8]) -> bool {
        assert!(output.len() < R);
        assert_eq!(input.len(), output.len() + T);

        let (input, tag) = input.split_at(output.len());
        let mut lastblock = [0; R];

        with(&mut self.state, |state| {
            state[15] ^= <tags::Payload as Tag>::TAG;
            permutation::norx(state);
        });

        lastblock.copy_from_slice(&self.state[..R]);
        lastblock[..input.len()].copy_from_slice(input);
        lastblock[input.len()] ^= 0x01;
        lastblock[R - 1] ^= 0x80;

        for i in 0..input.len() {
            output[i] = self.state[i] ^ lastblock[i];
        }
        self.state[..R].copy_from_slice(&lastblock);

        let mut tag2 = [0; T];
        Norx(self.state).finalize(key, aad, &mut tag2);

        slices_equal(tag, &tag2) == 1
    }
}

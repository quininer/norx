#[macro_use] extern crate arrayref;
extern crate rand;
extern crate norx;

mod aead;

use rand::{ Rng, thread_rng, random };
use aead::{ aead_encrypt, aead_decrypt };
use norx::constant::{ KEY_LENGTH, NONCE_LENGTH, TAG_LENGTH };


#[test]
fn test_aead() {
    for _ in 0..100 {
        let mut key = [0; KEY_LENGTH];
        let mut nonce = [0; NONCE_LENGTH];
        let mut aad = vec![0; (random::<usize>() % 128) + 1];
        let mut m = vec![0; (random::<usize>() % 1024) + 1];
        let mut c = vec![0; m.len() + TAG_LENGTH];
        let mut p = vec![0; m.len()];

        thread_rng().fill_bytes(&mut key);
        thread_rng().fill_bytes(&mut nonce);
        thread_rng().fill_bytes(&mut aad);
        thread_rng().fill_bytes(&mut m);

        aead_encrypt(&key, &nonce, &aad, &m, &mut c);
        let r = aead_decrypt(&key, &nonce, &aad, &c, &mut p);
        assert!(r);

        assert_eq!(p, m);
    }
}

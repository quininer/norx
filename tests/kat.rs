#[macro_use] extern crate arrayref;
extern crate rand;
extern crate norx;

#[allow(dead_code)]
mod aead;

use norx::constant::{ KEY_LENGTH, NONCE_LENGTH, TAG_LENGTH };
use aead::aead_encrypt;


const KAT: [u8; 40832] = include!("kat_config_6441.txt");

#[test]
fn test_aead_kat() {
    let mut w = [0; 256];
    let mut h = [0; 256];
    let mut k = [0; KEY_LENGTH];
    let mut n = [0; NONCE_LENGTH];

    for i in 0..w.len() {
        w[i] = (255 & (i * 197 + 123)) as u8;
    }
    for i in 0..h.len() {
        h[i] = (255 & (i * 193 + 123)) as u8;
    }
    for i in 0..k.len() {
        k[i] = (255 & (i * 191 + 123)) as u8;
    }
    for i in 0..n.len() {
        n[i] = (255 & (i * 181 + 123)) as u8;
    }

    let mut kat = &KAT[..];

    for i in 0..w.len() {
        let mut c = vec![0; i + TAG_LENGTH];

        aead_encrypt(&k, &n, &h[..i], &w[..i], &mut c);
        assert_eq!(c, &kat[..c.len()], "{} times", i);
        kat = &kat[c.len()..];
    }
}

#![cfg_attr(feature = "cargo-clippy", allow(needless_range_loop))]

#[macro_use] extern crate arrayref;
extern crate rand;
extern crate norx;

#[allow(dead_code)]
mod aead;

use norx::constant::{ KEY_LENGTH, NONCE_LENGTH, TAG_LENGTH };
use aead::aead_encrypt;

#[cfg(all(feature = "W32", feature = "L4", feature = "P1"))]
const KAT: [u8; 36_736] = include!("kat_config_3241.txt");

#[cfg(all(feature = "W32", feature = "L6", feature = "P1"))]
const KAT: [u8; 36_736] = include!("kat_config_3261.txt");

#[cfg(all(feature = "W64", feature = "L4", feature = "P1"))]
const KAT: [u8; 40_832] = include!("kat_config_6441.txt");

#[cfg(all(feature = "W64", feature = "L6", feature = "P1"))]
const KAT: [u8; 40_832] = include!("kat_config_6461.txt");

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

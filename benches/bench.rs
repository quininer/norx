#![feature(test)]

#[macro_use] extern crate arrayref;
extern crate test;
extern crate norx;


#[path = "../tests/aead/mod.rs"]
mod aead;

use test::Bencher;
use norx::constant::{ KEY_LENGTH, NONCE_LENGTH, TAG_LENGTH };
use aead::*;


#[bench]
fn bench_norx_encrypt_1024(b: &mut Bencher) {
    let key = [0x42; KEY_LENGTH];
    let nonce = [0x43; NONCE_LENGTH];
    let m = [0x44; 1024];
    let mut c = vec![0; m.len() + TAG_LENGTH];

    b.bytes = m.len() as u64;
    b.iter(|| aead_encrypt(&key, &nonce, &m[..10], &m, &mut c));
}

#[bench]
fn bench_norx_decrypt_1024(b: &mut Bencher) {
    let key = [0x43; KEY_LENGTH];
    let nonce = [0x44; NONCE_LENGTH];
    let m = [0x45; 1024];
    let mut p = vec![0; m.len()];
    let mut c = vec![0; m.len() + TAG_LENGTH];
    aead_encrypt(&key, &nonce, &m[..10], &m, &mut c);

    b.bytes = c.len() as u64;
    b.iter(|| aead_decrypt(&key, &nonce, &m[..10], &c, &mut p));
}

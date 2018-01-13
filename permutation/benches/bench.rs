#![feature(test)]
#![cfg_attr(feature = "simd", feature(cfg_target_feature))]

extern crate test;
extern crate norx_permutation;

use test::{ Bencher, black_box };
use norx_permutation::S;


#[bench]
fn bench_norx_portable(b: &mut Bencher) {
    use norx_permutation::portable;

    let mut data = black_box([42; S]);

    b.iter(|| {
        portable::norx(&mut data);
    });
}

#[cfg(feature = "simd")]
#[cfg(any(feature = "W32", feature = "W64"))]
#[cfg(target_feature = "ssse3")]
#[bench]
fn bench_norx_ssse3(b: &mut Bencher) {
    use norx_permutation::ssse3;

    let mut data = black_box([42; S]);

    b.iter(|| unsafe {
        ssse3::norx(&mut data);
    });
}

#[cfg(feature = "simd")]
#[cfg(feature = "W64")]
#[cfg(target_feature = "avx2")]
#[bench]
fn bench_norx_avx2(b: &mut Bencher) {
    use norx_permutation::avx2;

    let mut data = black_box([42; S]);

    b.iter(|| unsafe {
        avx2::norx(&mut data);
    });
}

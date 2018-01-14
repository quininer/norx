#![feature(test)]
#![cfg_attr(feature = "simd", feature(cfg_target_feature))]

extern crate test;
extern crate norx_permutation;

use test::{ Bencher, black_box };
use norx_permutation::S;


#[bench]
fn bench_norx_portable(b: &mut Bencher) {
    use norx_permutation::portable;

    let mut data = black_box([0x01; S]);

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

    let mut data = black_box([0x02; S]);

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

    let mut data = black_box([0x03; S]);

    b.iter(|| unsafe {
        avx2::norx(&mut data);
    });
}

#[cfg(feature = "simd")]
#[cfg(feature = "W64")]
#[cfg(target_feature = "avx2")]
#[bench]
fn bench_norx_avx2_x4(b: &mut Bencher) {
    use norx_permutation::avx2;

    let mut data1 = black_box([0x04; S]);
    let mut data2 = black_box([0x05; S]);
    let mut data3 = black_box([0x06; S]);
    let mut data4 = black_box([0x08; S]);

    b.iter(|| unsafe {
        avx2::norx_x4(&mut data1, &mut data2, &mut data3, &mut data4);
    });
}

extern crate norx_permutation;

use norx_permutation::U;

#[cfg(feature = "config_084")] const INPUT: [U; 16] = [0x4b, 0x24, 0x21, 0x33, 0x0d, 0xa3, 0x87, 0x80, 0x78, 0x58, 0xf7, 0xa4, 0x06, 0x52, 0x45, 0x3c];
#[cfg(feature = "config_164")] const INPUT: [U; 16] = [0xaea3, 0xf8b1, 0xc76a, 0x75d9, 0xf348, 0xad00, 0x9360, 0x5dc5, 0xd6b9, 0x62ce, 0x547e, 0x9059, 0x11de, 0x804b, 0x38cd, 0x5b3a];

#[cfg(any(feature = "config_324", feature = "config_326"))]
const INPUT: [U; 16] = [
    0x0567e9ed, 0xea98dc65, 0x0bd89cec, 0x16bbf579, 0x9767a1f3, 0x3f004c54, 0x3a7c2bc2, 0x357f3882,
    0xc63d174d, 0x17ebcd84, 0x834d79ac, 0x12fd72b1, 0xe0e463e5, 0x46fb8fa4, 0xc34feaf4, 0x8f9e8686
];

#[cfg(any(feature = "config_644", feature = "config_646"))]
const INPUT: [U; 16] = [
    0xcb3664cd781577a4, 0xfa8d9e54bce5f8b2, 0x24ddf7e3dec66b33, 0xd57c8c27c9893ccd,
    0x9e9088a6ca66bb01, 0xe1478c6bc8f91159, 0x552c525d9e9722bd, 0x380c985ec23474d6,
    0x0aa618d997f17b63, 0xee68f0781c035fa4, 0x254b9765b83d50b9, 0x72b72243aba2b1dc,
    0x0d3f9d918637b26d, 0x32cd20bb135236b3, 0x88a958b29d5a9657, 0x24aba9dbd1f60066
];

#[cfg(feature = "config_084")] const OUTPUT: [U; 16] = [0x45, 0x3c, 0x83, 0xe1, 0x09, 0x41, 0xfb, 0xde, 0xa2, 0x86, 0x4f, 0xa1, 0x38, 0xdc, 0xa5, 0xee];
#[cfg(feature = "config_164")] const OUTPUT: [U; 16] = [0xa46e, 0x99d1, 0xf54c, 0x53ff, 0x47e0, 0xc58a, 0x7af8, 0x39a2, 0xef4e, 0x7eec, 0x1229, 0xd3cf, 0x12b9, 0xa8a7, 0x43b4, 0xf65e];
#[cfg(feature = "config_324")] const OUTPUT: [U; 16] = [0x03496b0e, 0x4355ef33, 0x1d13d45a, 0x94b3e50e, 0xe1b0779e, 0x369f4ebb, 0xe3187bf0, 0x5e5f3450, 0x4102c8e5, 0x3a371450, 0x5c1cd98a, 0xd803978c, 0x1d51a4bb, 0x998a9bd5, 0x1373879d, 0xc2ae9570];
#[cfg(feature = "config_326")] const OUTPUT: [U; 16] = [0xef0e19e0, 0x997e5240, 0x25702def, 0xf29d1771, 0xf7474dd1, 0x06565f6d, 0x3703ebb6, 0x2d18e331, 0xfb6a2f2f, 0x936f9d6b, 0x866c090f, 0x000d7d94, 0x8cf73f00, 0xd7b99e9d, 0xe0f858ad, 0x5881b2df];
#[cfg(feature = "config_644")] const OUTPUT: [U; 16] = [0x45e0d64db4f0925d, 0x7e0b9688439c0b7f, 0xfbf464f711655d88, 0x062d533988686e35, 0x032659137a57a222, 0x5a35da449d7cb55f, 0x11d2f1fbfe8b5c91, 0x7349081d82e285e5, 0xd01526d2b275208b, 0x29bcf31cb7e3cae1, 0xc5720c9f7345ce2b, 0x1c0aaef5f5c89e0f, 0x13aa47bbce6930a5, 0x3b3d8f454d05de6c, 0x714544d2221211a2, 0x904c9ac89b078491];
#[cfg(feature = "config_646")] const OUTPUT: [U; 16] = [0xde5e024aa6248d32, 0x2441236d1fbd0341, 0xee55e5a6dad4f7ea, 0x04cbffbbbc4280f9, 0xa600f2efdec31029, 0x94e06147b4766b01, 0x4cbf7d6c058b9bd5, 0xb3405b99d88d257d, 0xff62f28a4d1c0785, 0xf843e409d44cf30c, 0x75d76dd306d759e7, 0x42b89f1ba1d3e494, 0x14365d17d5c8ece9, 0x8c59c056608ac1ed, 0x3104e873c78ecff8, 0xeda94ce2c0364604];


#[test]
fn test_norx_portable() {
    use norx_permutation::portable;

    let mut state = [0; 16];
    state.clone_from(&INPUT);

    portable::norx(&mut state);

    assert_eq!(state, OUTPUT);
}

#[cfg(feature = "simd")]
#[cfg(any(feature = "config_644", feature = "config_646"))]
#[test]
fn test_norx_ssse3() {
    use norx_permutation::ssse3;

    let mut state = [0; 16];
    state.clone_from(&INPUT);

    unsafe { ssse3::norx(&mut state) };

    assert_eq!(state, OUTPUT);
}

#[cfg(feature = "simd")]
#[cfg(any(feature = "config_644", feature = "config_646"))]
#[test]
fn test_norx_avx2() {
    use norx_permutation::avx2;

    let mut state = [0; 16];
    state.clone_from(&INPUT);

    unsafe { avx2::norx(&mut state) };

    assert_eq!(state, OUTPUT);
}

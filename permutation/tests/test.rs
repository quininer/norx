extern crate norx_permutation;


const INPUT: [u64; 16] = [
    0xcb3664cd781577a4, 0xfa8d9e54bce5f8b2, 0x24ddf7e3dec66b33, 0xd57c8c27c9893ccd,
    0x9e9088a6ca66bb01, 0xe1478c6bc8f91159, 0x552c525d9e9722bd, 0x380c985ec23474d6,
    0x0aa618d997f17b63, 0xee68f0781c035fa4, 0x254b9765b83d50b9, 0x72b72243aba2b1dc,
    0x0d3f9d918637b26d, 0x32cd20bb135236b3, 0x88a958b29d5a9657, 0x24aba9dbd1f60066
];

#[cfg(feature = "config_646")] const OUTPUT: [u64; 16] = [0xde5e024aa6248d32, 0x2441236d1fbd0341, 0xee55e5a6dad4f7ea, 0x04cbffbbbc4280f9, 0xa600f2efdec31029, 0x94e06147b4766b01, 0x4cbf7d6c058b9bd5, 0xb3405b99d88d257d, 0xff62f28a4d1c0785, 0xf843e409d44cf30c, 0x75d76dd306d759e7, 0x42b89f1ba1d3e494, 0x14365d17d5c8ece9, 0x8c59c056608ac1ed, 0x3104e873c78ecff8, 0xeda94ce2c0364604];
#[cfg(feature = "config_644")] const OUTPUT: [u64; 16] = [0x45e0d64db4f0925d, 0x7e0b9688439c0b7f, 0xfbf464f711655d88, 0x062d533988686e35, 0x032659137a57a222, 0x5a35da449d7cb55f, 0x11d2f1fbfe8b5c91, 0x7349081d82e285e5, 0xd01526d2b275208b, 0x29bcf31cb7e3cae1, 0xc5720c9f7345ce2b, 0x1c0aaef5f5c89e0f, 0x13aa47bbce6930a5, 0x3b3d8f454d05de6c, 0x714544d2221211a2, 0x904c9ac89b078491];


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

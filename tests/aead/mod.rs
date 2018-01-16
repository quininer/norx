use norx::constant::{ KEY_LENGTH, NONCE_LENGTH, BLOCK_LENGTH, TAG_LENGTH };
use norx::Norx;


pub fn aead_encrypt(key: &[u8; KEY_LENGTH], nonce: &[u8; NONCE_LENGTH], aad: &[u8], m: &[u8], c: &mut [u8]) {
    assert_eq!(m.len() + TAG_LENGTH, c.len());

    let (m1, m2) = m.split_at(m.len() - m.len() % BLOCK_LENGTH);
    let (c1, c2) = c.split_at_mut(m1.len());

    let mut process = Norx::new(key, nonce).encrypt(aad);
    process.process(
        m1.chunks(BLOCK_LENGTH)
            .zip(c1.chunks_mut(BLOCK_LENGTH))
            .map(|(x, y)| (
                array_ref!(x, 0, BLOCK_LENGTH),
                array_mut_ref!(y, 0, BLOCK_LENGTH)
            ))
    );
    process.finalize(key, &[], m2, c2);
}

pub fn aead_decrypt(key: &[u8; KEY_LENGTH], nonce: &[u8; NONCE_LENGTH], aad: &[u8], c: &[u8], m: &mut [u8]) -> bool {
    assert!(c.len() >= TAG_LENGTH);
    assert_eq!(m.len() + TAG_LENGTH, c.len());

    let m_len = m.len();
    let (m1, m2) = m.split_at_mut(m_len - m_len % BLOCK_LENGTH);
    let (c1, c2) = c.split_at(m1.len());

    let mut process = Norx::new(key, nonce).decrypt(aad);
    process.process(
        c1.chunks(BLOCK_LENGTH)
            .zip(m1.chunks_mut(BLOCK_LENGTH))
            .map(|(x, y)| (
                array_ref!(x, 0, BLOCK_LENGTH),
                array_mut_ref!(y, 0, BLOCK_LENGTH)
            ))
    );
    process.finalize(key, &[], c2, m2)
}

use norx::{
    Norx,
    K, N, R, T
};


pub fn aead_encrypt(key: &[u8; K], nonce: &[u8; N], aad: &[u8], m: &[u8], c: &mut [u8]) {
    assert_eq!(m.len() + T, c.len());

    let (m1, m2) = m.split_at(m.len() - m.len() % R);
    let (c1, c2) = c.split_at_mut(m1.len());

    let mut process = Norx::new(key, nonce).encrypt(aad);
    process.process(
        m1.chunks(R)
            .zip(c1.chunks_mut(R))
            .map(|(x, y)| (
                array_ref!(x, 0, R),
                array_mut_ref!(y, 0, R)
            ))
    );
    process.finalize(key, &[], m2, c2);
}

pub fn aead_decrypt(key: &[u8; K], nonce: &[u8; N], aad: &[u8], c: &[u8], m: &mut [u8]) -> bool {
    assert!(c.len() >= T);
    assert_eq!(m.len() + T, c.len());

    let m_len = m.len();
    let (m1, m2) = m.split_at_mut(m_len - m_len % R);
    let (c1, c2) = c.split_at(m1.len());

    let mut process = Norx::new(key, nonce).decrypt(aad);
    process.process(
        c1.chunks(R)
            .zip(m1.chunks_mut(R))
            .map(|(x, y)| (
                array_ref!(x, 0, R),
                array_mut_ref!(y, 0, R)
            ))
    );
    process.finalize(key, &[], c2, m2)
}

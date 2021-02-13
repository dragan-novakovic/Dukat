use sha2::{Digest, Sha256};

pub trait Hashable {
    fn bytes(&self) -> Vec<u8>;
    fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(&self.bytes());

        let result = hasher.finalize();
        result.to_vec()
    }
}

// pub fn u32_bytes(u: &u32) -> [u8; 4] {
//     [
//         (u >> 8 * 0x0) as u8,
//         (u >> 8 * 0x1) as u8,
//         (u >> 8 * 0x2) as u8,
//         (u >> 8 * 0x3) as u8,
//     ]
// }

// pub fn u64_bytes(u: &u64) -> [u8; 8] {
//     [
//         (u >> 8 * 0x0) as u8,
//         (u >> 8 * 0x1) as u8,
//         (u >> 8 * 0x2) as u8,
//         (u >> 8 * 0x3) as u8,
//         (u >> 8 * 0x4) as u8,
//         (u >> 8 * 0x5) as u8,
//         (u >> 8 * 0x6) as u8,
//         (u >> 8 * 0x7) as u8,
//     ]
// }

pub fn u128_bytes(u: &u128) -> [u8; 16] {
    [
        (u >> 8 * 0x0) as u8,
        (u >> 8 * 0x1) as u8,
        (u >> 8 * 0x2) as u8,
        (u >> 8 * 0x3) as u8,
        (u >> 8 * 0x4) as u8,
        (u >> 8 * 0x5) as u8,
        (u >> 8 * 0x6) as u8,
        (u >> 8 * 0x7) as u8,
        (u >> 8 * 0x8) as u8,
        (u >> 8 * 0x9) as u8,
        (u >> 8 * 0xa) as u8,
        (u >> 8 * 0xb) as u8,
        (u >> 8 * 0xc) as u8,
        (u >> 8 * 0xd) as u8,
        (u >> 8 * 0xe) as u8,
        (u >> 8 * 0xf) as u8,
    ]
}
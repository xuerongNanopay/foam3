#![allow(unused)]

const K2: u64 = 0x9ae16a3b2f90404f;
const K0: u64 = 0xc3a5c85c97cb3127;
const K1: u64 = 0xb492b66fbe98f273;

fn rotate(val: u64, shift: u64) -> u64 {
    (val >> shift) | (val << (64 - shift))
}

fn fetch64(data: &[u8]) -> u64 {
    let mut bytes = [0u8; 8];
    bytes.copy_from_slice(&data[..8]);
    u64::from_le_bytes(bytes)
}

//NEED TODO: test
pub(crate) fn city_hash64(data: &[u8]) -> u64 {
    let len = data.len();
    if len <= 16 {
        let a = fetch64(data).wrapping_mul(K2);
        let b = fetch64(&data[len.saturating_sub(8)..]).wrapping_mul(K0);
        return rotate(a + b, 43).wrapping_mul(K1);
    }

    let mut hash = (len as u64).wrapping_mul(K0);
    let mut a = fetch64(data).wrapping_mul(K1);
    let mut b = fetch64(&data[len.saturating_sub(8)..]).wrapping_mul(K2);
    
    hash = rotate(a + b, 43).wrapping_mul(K1);
    hash = rotate(hash + b, 37).wrapping_mul(K1);
    
    hash
}
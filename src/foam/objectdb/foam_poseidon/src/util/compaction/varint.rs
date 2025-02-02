#![allow(unused)]
// Variable-Length Encoding 

use crate::{error::FP_ILLEGAL_ARGUMENT, internal::FPResult};

//TODO: untested code.
pub fn decode_uint(b: &[u8]) -> FPResult<(u64, usize)> {
    let mut result: u64 = 0;
    let mut shift = 0;

    for (i, &byte) in b.iter().enumerate() {
        let value = (byte & 0x7F) as u64;
        result |= value << shift;

        if byte & 0x80 == 0 {
            return Ok((result, i+1));
        }

        shift += 7;

        if shift >= 64 {
            return Err(FP_ILLEGAL_ARGUMENT);
        }
    }

    Err(FP_ILLEGAL_ARGUMENT)
}

pub fn encode_uint(mut v: u64) -> FPResult<Vec<u8>> {
    let mut buffer = Vec::new();
    while v >= 0x80 {
        buffer.push((v as u8 & 0x7F) | 0x80);
        v >>= 7;
    }
    buffer.push(v as u8);
    Ok(buffer)
}

pub fn encode_uint_inline(mut v: u64, buf: &mut [u8]) -> FPResult<usize> {
    let mut i = 0;
    while v >= 0x80 {
        buf[i] = (v as u8 & 0x7F) | 0x80;
        v >>= 7;
        i += 1;
    }
    buf[i] = v as u8;
    Ok(i+1)
}

pub struct VarintDecodeIterator<'a> {
    slice: &'a [u8],
    max_size: usize,
    position: usize,
}

impl<'a> VarintDecodeIterator<'a> {
    pub fn new(slice: &'a [u8]) -> Self {
        Self {
            slice,
            max_size: slice.len(),
            position: 0,
        }
    }
    pub fn new_with_limit(slice: &'a [u8], max_size: usize) -> Self {
        Self {
            slice,
            max_size,
            position: 0,
        }
    }
    pub fn cur(&self) -> Option<u8> {
        if self.position < self.slice.len() {
            return Some(self.slice[self.position]);
        }
        None
    }

    pub fn maybe_next(&self) -> bool {
        self.position < FP_MIN!(self.slice.len(), self.max_size)
    }
}

impl<'a> Iterator for VarintDecodeIterator<'a> {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.position >= self.max_size {
            return None;
        }
    
        let(v, s) = match decode_uint(self.slice) {
            Ok((v, s)) => (v, s),
            Err(e) => return None,
        };

        let (l, r) = self.slice.split_at(s);
        self.slice = r;
        self.position  += s;
        
        Some(v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode() {
        let v = 300u64;
        let ret = encode_uint(v).unwrap();
        assert_eq!(&ret, &[172, 2]);

        let v = 127u64;
        let ret = encode_uint(v).unwrap();
        assert_eq!(&ret, &[127]);

        let v = 128u64;
        let ret = encode_uint(v).unwrap();
        assert_eq!(&ret, &[128, 1]);

        let v = 18446744073709551615u64;
        let ret = encode_uint(v).unwrap();
        assert_eq!(&ret, &[255, 255, 255, 255, 255, 255, 255, 255, 255, 1]);

        let v = 4611686018427387903u64;
        let ret = encode_uint(v).unwrap();
        assert_eq!(&ret, &[255, 255, 255, 255, 255, 255, 255, 255, 63]);

        let v = 9223372036854775807u64;
        let ret = encode_uint(v).unwrap();
        assert_eq!(&ret, &[255, 255, 255, 255, 255, 255, 255, 255, 127]);
    }

    #[test]
    fn test_encode_inline() {

        let v = 300u64;
        let mut buf = [0u8; 2];
        let b = &mut buf[..];
        encode_uint_inline(v, b).unwrap();
        assert_eq!(b, &[172, 2]);

        let v = 127u64;
        let mut buf = [0u8; 1];
        let b = &mut buf[..];
        encode_uint_inline(v, b).unwrap();
        assert_eq!(b, &[127]);

        let v = 128u64;
        let mut buf = [0u8; 2];
        let b = &mut buf[..];
        encode_uint_inline(v, b).unwrap();
        assert_eq!(b, &[128, 1]);

        let v = 18446744073709551615u64;
        let mut buf = [0u8; 10];
        let b = &mut buf[..];
        encode_uint_inline(v, b).unwrap();
        assert_eq!(b, &[255, 255, 255, 255, 255, 255, 255, 255, 255, 1]);

        let v = 4611686018427387903u64;
        let mut buf = [0u8; 9];
        let b = &mut buf[..];
        encode_uint_inline(v, b).unwrap();
        assert_eq!(b, &[255, 255, 255, 255, 255, 255, 255, 255, 63]);
    }

    #[test]
    fn test_decode() {
        let (v, s) = decode_uint(&[172, 2]).unwrap();
        assert_eq!(v, 300);
        assert_eq!(s, 2);

        let (v, s) = decode_uint(&[127]).unwrap();
        assert_eq!(v, 127);
        assert_eq!(s, 1);

        let (v, s) = decode_uint(&[128, 1]).unwrap();
        assert_eq!(v, 128);
        assert_eq!(s, 2);

        let (v, s) = decode_uint(&[255, 255, 255, 255, 255, 255, 255, 255, 255, 1]).unwrap();
        assert_eq!(v, 18446744073709551615u64);
        assert_eq!(s, 10);

        let (v, s) = decode_uint(&[255, 255, 255, 255, 255, 255, 255, 255, 63]).unwrap();
        assert_eq!(v, 4611686018427387903u64);
        assert_eq!(s, 9);

        let (v, s) = decode_uint(&[255, 255, 255, 255, 255, 255, 255, 255, 127]).unwrap();
        assert_eq!(v, 9223372036854775807u64);
        assert_eq!(s, 9);
    }

    #[test]
    fn test_decode_iterator() {
        let buf: &[u8] = &[
            255, 255, 255, 255, 255, 255, 255, 255, 255, 1,
            255, 255, 255, 255, 255, 255, 255, 255, 63,
            172, 2,
            127,
            128, 1,
            255, 255, 255, 255, 255, 255, 255, 255, 127
        ];
        let mut iter = VarintDecodeIterator::new(buf);

        assert_eq!(iter.next(), Some(18446744073709551615u64));
        assert_eq!(iter.next(), Some(4611686018427387903u64));
        assert_eq!(iter.next(), Some(300));
        assert_eq!(iter.next(), Some(127));
        assert_eq!(iter.next(), Some(128));
        assert_eq!(iter.next(), Some(9223372036854775807u64));
        assert_eq!(iter.next(), None);

        let mut iter = VarintDecodeIterator::new_with_limit(buf, 22);

        assert_eq!(iter.next(), Some(18446744073709551615u64));
        assert_eq!(iter.next(), Some(4611686018427387903u64));
        assert_eq!(iter.next(), Some(300));
        assert_eq!(iter.next(), Some(127));
        assert_eq!(iter.next(), None);
    }
}
#![allow(unused)]
// Variable-Length Encoding 

use crate::{error::FP_ILLEGAL_ARGUMENT, types::FPResult};

//TODO: untested code.
pub fn decode_uint(b: &[u8]) -> FPResult<u64> {
    let mut result: u64 = 0;
    let mut shift = 0;

    for (i, &byte) in b.iter().enumerate() {
        let value = (byte & 0x7F) as u64;
        result |= value << shift;

        if byte & 0x80 == 0 {
            return Ok(result);
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
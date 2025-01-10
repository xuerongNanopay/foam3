#![allow(unused)]
// Variable-Length Encoding 

use crate::{error::FP_ILLEGAL_ARGUMENT, types::FPResult};

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

pub fn encode_uint(b: &[u8]) -> FPResult<()> {
    Err(FP_ILLEGAL_ARGUMENT)
}
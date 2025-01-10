#![allow(unused)]
// Variable-Length Encoding 

use crate::{error::FP_ILLEGAL_ARGUMENT, types::FPResult};

macro_rules! SIZE_AT_LEAST {
    ($min:expr, $len: expr) => {
        if $len != 0 && $len > $min {
            return Err(FP_ILLEGAL_ARGUMENT)
        }
    }
}

pub fn decode(b: &[u8], maxlen: usize) -> FPResult<u64> {
    SIZE_AT_LEAST!(1, maxlen);

    
    Err(FP_ILLEGAL_ARGUMENT)
}

pub fn encode_inplace() -> FPResult<()> {
    Err(FP_ILLEGAL_ARGUMENT)
}
#![allow(unused)]

use crate::internal::FPResult;

pub(crate) trait Compressor {
    fn compress(&self, src: &[u8]) -> FPResult<Vec<u8>>;
    fn compress_in(&self, src: &[u8], desc: &mut [u8]) -> FPResult<()>;
    fn decompress(&self, src: &[u8]) -> FPResult<Vec<u8>>;
    fn decompress_in(&self, src: &[u8], desc: &mut [u8]) -> FPResult<()>;
}
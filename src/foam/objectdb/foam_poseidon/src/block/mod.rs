#![allow(unused)]

use crate::meta::FP_METAFILE;

pub mod manager;
pub mod block_handle;
pub mod block_meta;

static FP_BLOCK_INVALID_OFFSET: u64 = 0;

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
struct BlockHeader {
    // 0x646464
    magic: u32,
    major: u16,
    minor: u16,
    checksum: u32,
    reserved: u32,
}

impl BlockHeader {
    fn maybe_convert_endian(&mut self) {
        if cfg!(target_endian = "big") { 
            self.magic = BIT_REVERSE_32!(self.magic);
            self.major = BIT_REVERSE_16!(self.major);
            self.minor = BIT_REVERSE_16!(self.minor);
            self.checksum = BIT_REVERSE_32!(self.checksum);
            self.reserved = BIT_REVERSE_32!(self.reserved);
        }
    }
}

fn is_internal_file(filename: &str) -> bool {
    if filename == FP_METAFILE {
        return true
    }
    false
}

#[cfg(test)]
mod tests {
    // use super::BlockHeader;

    use std::fs::File;
    use std::io::{self, Bytes, Read, Write};
    use std::{mem, vec};

    use crate::block::BlockHeader;

    #[test]
    fn write_block_header_to_file() {
        let filename = "/tmp/block_header.fp";
        let mut file = File::create(filename).unwrap();
        let mut w_buf = Vec::<u8>::with_capacity(SIZE_OF!(BlockHeader));
        unsafe { w_buf.set_len(SIZE_OF!(BlockHeader)); }
        w_buf.fill(0);
        let header = REINTERPRET_CAST_BUF_MUT!(w_buf, BlockHeader);

        header.checksum = 11;
        header.magic = 56;

        println!("{:?}", w_buf);
        println!("{:?}", header);
        file.write_all(&w_buf);
        file.sync_all();

        let mut file = File::open(filename).unwrap();
        let mut rbuf = vec![0u8; w_buf.len() as usize];
        file.read_exact(&mut rbuf);
        
        let b_header = REINTERPRET_CAST_BUF!(rbuf, BlockHeader);

        println!("{:?}", rbuf);
        println!("{:?}", b_header);

        let mut n_header = *b_header;
        println!("{:?}", n_header);
        n_header.checksum = 77;
        println!("{:?}", b_header);
        println!("{:?}", n_header);
    }
}
#![allow(unused)]

use crate::{meta::FP_METAFILE, internal::{FPFileSize}};

pub mod manager;
pub mod handle;
pub mod addr;
pub mod cache;
mod pool;
mod mgr;
mod meta;

static FP_BLOCK_INVALID_OFFSET: u64 = 0;

pub(crate) const FP_BLOCK_HEADER_LEN: usize = 30;

type RawBlk = Vec<u8>;

/**
 * Each .fp file has one FileHeader
 */
#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
struct FileHeader {
    magic: u32,
    major: u16,
    minor: u16,
    checksum: u32,
    reserved: u32,
}

impl FileHeader {
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

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct BlockRef {
    source_id: u32,
    offset: FPFileSize, /* offset in file */
    size: FPFileSize, /* size of a block in file */
    checksum: u32,
}


#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct BlockHeader {
    disk_size: u32,
    checksum: u32,
    flags: u8,
    unused: [u8; 3],
}

impl BlockHeader {
    fn maybe_convert_endian(&mut self) {
        if cfg!(target_endian = "big") { 
            self.disk_size = BIT_REVERSE_32!(self.disk_size);
            self.checksum = BIT_REVERSE_32!(self.checksum);
        }
    }
}

#[repr(C)]
#[derive(Debug, Default, Clone, Copy)]
pub(crate) struct PageHeader {
    record_number: u64, /* For column oriented storage to store the record id of the first tuple in the page */
    write_epoch: u64,
    memory_size: u32,
    entries: u32,
    overflow_data_len: u32,
    page_type: u8,
}

impl PageHeader {
    fn maybe_convert_endian(&mut self) {
        if cfg!(target_endian = "big") { 
            self.record_number = BIT_REVERSE_64!(self.record_number);
            self.write_epoch = BIT_REVERSE_64!(self.write_epoch);
            self.memory_size = BIT_REVERSE_32!(self.memory_size);
            self.entries = BIT_REVERSE_32!(self.entries);
            self.overflow_data_len = BIT_REVERSE_32!(self.overflow_data_len);
        }
    }
}

pub(crate) struct BlkItem {
    // pub(crate) reserved_header_len: usize,
    pub(crate) data: Vec<u8>,
    // pub(crate) data: &'static [u8],
    pub(crate) size: usize,
    // pub(crate) mem: Vec<u8>


}

#[derive(Default, Debug, Clone, Copy)]
pub struct BlockStat{
    allocation_size: u64,
    block_size: u64,
    block_magic: u64,
    block_major: u16,
    block_minor: u16,
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

    use crate::blk::FileHeader;

    #[test]
    fn write_block_header_to_file() {
        let filename = "/tmp/block_header.fp";
        let mut file = File::create(filename).unwrap();
        let mut w_buf = Vec::<u8>::with_capacity(FP_SIZE_OF!(FileHeader));
        unsafe { w_buf.set_len(FP_SIZE_OF!(FileHeader)); }
        w_buf.fill(0);
        let header = FP_REINTERPRET_CAST_BUF_MUT!(w_buf, FileHeader);

        header.checksum = 11;
        header.magic = 56;

        println!("{:?}", w_buf);
        println!("{:?}", header);
        file.write_all(&w_buf);
        file.sync_all();

        let mut file = File::open(filename).unwrap();
        let mut rbuf = vec![0u8; w_buf.len() as usize];
        file.read_exact(&mut rbuf);
        
        let b_header = FP_REINTERPRET_CAST_BUF!(rbuf, FileHeader);

        println!("{:?}", rbuf);
        println!("{:?}", b_header);

        let mut n_header = *b_header;
        println!("{:?}", n_header);
        n_header.checksum = 77;
        println!("{:?}", b_header);
        println!("{:?}", n_header);
    }
}
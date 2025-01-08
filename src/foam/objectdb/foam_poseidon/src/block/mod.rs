#![allow(unused)]

use std::{collections::LinkedList, sync::{Arc, RwLock}};

use crate::{os::fil::{FPFileHandle, FileHandle}, types::{FPConcurrentHashMap, FPFileSize}};

mod open;

static FP_BLOCK_INVALID_OFFSET: u64 = 0;


struct BlockManager {
    blocks: FPConcurrentHashMap<String, Arc<Block>>
}

impl BlockManager {
    fn get_block(&self, filename: &str) -> Option<Arc<Block>> {
        let blocks = self.blocks.read().unwrap();
        match blocks.get(filename) {
            Some(v) => Some(Arc::clone(v)),
            None => None
        }
    }
    fn insert_block(&self, filename: &str, block: Arc<Block>) {
        let mut blocks = self.blocks.write().unwrap();
        blocks.insert(String::from(filename), block);
    }
}

/**
 * Block; reference a singal file.
 * Physical representation of page.
 */
struct Block {
    name: String,   /* Name */
    object_id: u32,

    size: FPFileSize,       /* File size */

    allocation_size: FPFileSize,
    alloc_first: bool,

    // os_cache: usize,
    os_cache_max: usize,
    os_cache_dirty_max: usize,

    extend_len: usize,
    
    readonly: bool,

    file_handle: Arc<FPFileHandle>,
}

#[repr(C)]
#[derive(Debug)]
struct BlockHeader {
    // 0x646464
    magic: u32,
    major: u16,
    minor: u16,
    checksum: u32,
    reserved: u32,
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
        
    }
}
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
    use std::mem;

    use crate::block::BlockHeader;

    #[test]
    fn write_block_header_to_file() {
        let filename = "/tmp/block_header.fp";
        let mut file = File::create(filename).unwrap();
        let buffer = VEC_U8![0, 0, 0, 1, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 1, 1, 0];
        file.write_all(&buffer);
        file.sync_all();

        let mut file = File::open(filename).unwrap();
        let mut rbuf = vec![0u8; buffer.len() as usize];
        file.read_exact(&mut rbuf);
        
        println!("{:?}", rbuf);

        println!("len:{}, cap:{}", rbuf.len(), rbuf.capacity());
        let b_header: BlockHeader = unsafe { mem::transmute(rbuf.as_slice()) };
        let c_header = BlockHeader{
            magic: 32,
            major: 32,
            minor: 32,
            checksum: 32,
            reserved: 32,
        };
        println!("{:?}", b_header);
        println!("{:?}", c_header);

        let bytes: [u8; mem::size_of::<BlockHeader>()] = [
            1, 0, 0, 0, 0, 1, 0, 1,
            4, 0, 0, 0, 0, 0, 24, 64
        ];
        let d_header: BlockHeader = unsafe { mem::transmute(bytes) };
        println!("{:?}", d_header);

        let bytes: Vec<u8> = vec![
            1, 0, 0, 0, 0, 1, 0, 1,
            4, 0, 0, 0, 0, 0, 24, 64
        ];
        let d_header: BlockHeader = unsafe { mem::transmute(bytes.as_slice()) };
        println!("{:?}", d_header);

        {
            let mut e_header = unsafe { 
                std::ptr::read(rbuf.as_ptr() as *const BlockHeader) 
            };
            println!("{:?}", e_header);
        }

        println!("{:?}", rbuf);

        {
            let mut e_header = unsafe {
                let p = rbuf.as_mut_ptr();
                p.add(0);
                &mut *(p as *mut BlockHeader)
            };
            let mut e_header = REINTERPRET_CAST_BUF_MUT!(rbuf, BlockHeader);
            e_header.checksum = 7;
            println!("e_header {:?}", e_header);
        }

        println!("{:?}", rbuf);
        
    }
}
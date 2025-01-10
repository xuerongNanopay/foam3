use std::{collections::HashMap, sync::{Arc, RwLock, RwLockWriteGuard}, usize};

use crate::{os::fil::{FPFileSystem, FileHandle, FileSystem, FileType, FP_FS_OPEN_CREATE, FP_FS_OPEN_DURABLE, FP_FS_OPEN_EXCLUSIVE}, types::{FPConcurrentHashMap, FPResult}};

use super::block_handle::{self, block_header_write, BlockHandle};

//TODO: drop file object from directory.(block_open.c line28)

/**
 * Block manager, reference to a block(block reference to a file).
 */
pub(crate) struct BlockManager {
    block_handle: Arc<BlockHandle>,
}

/**
 * Create a file and write meta data to it.
 */
fn create(file_system: Arc<FPFileSystem>, filename: &str, alloc_size: u32) -> FPResult<()> {
    let fh = file_system.open(filename, FileType::Data, FP_FS_OPEN_CREATE | FP_FS_OPEN_DURABLE | FP_FS_OPEN_EXCLUSIVE)?;
    block_header_write(fh.clone(), alloc_size)?;
    fh.sync()?;
    file_system.close_fh(&fh.name)?;
    Ok(())
}

/**
 * Drop a file.
 */
fn drop() {

}

/**
 * Read a file.
 */
fn read(block_manager: Arc<BlockManager>, raw_addr: &[u8], addr_size: usize) {

}

// pub struct BlockManager {
//     blocks: FPConcurrentHashMap<String, Arc<Block>>,
// }

// impl BlockManager {
//     pub(crate) fn get_block(&self, filename: &str) -> Option<Arc<Block>> {
//         let blocks = self.blocks.read().unwrap();
//         match blocks.get(filename) {
//             Some(v) => Some(Arc::clone(v)),
//             None => None
//         }
//     }
//     pub(crate) fn insert_block(&self, filename: &str, block: Arc<Block>) {
//         let mut blocks = self.blocks.write().unwrap();
//         blocks.insert(String::from(filename), block);
//     }

//     pub(crate) fn get_block_or_default<F>(&self, filename: &str, init_f: F) -> FPResult<Arc<Block>>
//     where F: Fn() -> FPResult<Arc<Block>>
//     {
//         if let Some(o) = self.get_block(filename) {
//             return Ok(o);
//         }

//         let mut blocks = self.blocks.write().unwrap();
//         let new_block = init_f()?;
//         let ret = new_block.clone();
//         blocks.insert(String::from(filename), new_block);
//         Ok(ret)
//     }
// }
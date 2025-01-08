use std::{collections::HashMap, sync::{Arc, RwLock, RwLockWriteGuard}};

use crate::types::{FPConcurrentHashMap, FPResult};

use super::{block::Block};

/**
 * Block manager, reference to a block(block reference to a file).
 */
pub(crate) struct BlockManager {
    block: Arc<Block>,
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
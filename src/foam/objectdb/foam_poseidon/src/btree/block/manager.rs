#![allow(unused)]

use crate::{error::FP_NO_IMPL, file::{File, MemFile}, internal::{FPResult, FP_KILOBYTE}};

/**
 * Block manager that manager block access for btree.
 */
pub(crate) struct BlockManager {
    name: String,
    file_handle: Box<dyn File>,
    block_size: u64,
}

impl BlockManager {
    pub(crate) fn new(uri: &str) -> FPResult<BlockManager> {
        Ok(Self{
            name: uri.to_owned(),
            file_handle: Box::new(MemFile::new()),
            block_size: 4*FP_KILOBYTE, //4KB.
        })
    }
}
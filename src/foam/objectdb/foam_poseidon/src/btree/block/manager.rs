#![allow(unused)]

use crate::{error::FP_NO_IMPL, file::{File, MemFile}, internal::FPResult};

/**
 * Block manager that manager block access for btree.
 */
pub(crate) struct BlockManager {
    name: String,
    file_handle: Box<dyn File>
}

impl BlockManager {
    fn new(uri: &str) -> FPResult<BlockManager> {
        Ok(Self{
            name: uri.to_owned(),
            file_handle: Box::new(MemFile::new()),
        })
    }
}
#![allow(unused)]

use crate::{error::FP_NO_IMPL, file::File, internal::FPResult};

/**
 * Block manager that manager block access for btree.
 */
pub(crate) struct BlockManager {
    name: String,
    file_descriptor: Box<dyn File>
}
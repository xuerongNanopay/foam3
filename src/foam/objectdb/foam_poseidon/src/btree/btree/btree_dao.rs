#![allow(unused)]

use crate::{btree, dao::DAO, error::FP_NO_IMPL, types::FPResult};

use super::BTree;

pub(super) struct BTreeDAO<'b> {
    btree: &'b BTree,
}

impl BTreeDAO<'_> {
    pub(super) fn get_btree(&self) -> &BTree {
        self.btree
    }
}

impl DAO for BTreeDAO<'_> {

    fn write_size(&self, len: usize) -> FPResult<()> {
        //TODO: limit write?
        Ok(())
    }
}
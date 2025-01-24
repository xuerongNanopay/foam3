#![allow(unused)]

use crate::{dao::DAO, error::FP_NO_IMPL, types::FPResult};

pub(super) struct BTreeDAO {

}

impl DAO for BTreeDAO {

    fn write_size(&self, len: usize) -> FPResult<()> {
        //TODO: limit write?
        Ok(())
    }
}
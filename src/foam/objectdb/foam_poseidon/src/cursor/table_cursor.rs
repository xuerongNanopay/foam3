#![allow(unused)]

use std::any::Any;

use crate::{error::FP_NO_IMPL, types::FPResult};

use super::Cursor;

struct TableCursor {
    //TODO: scheme table
    cfg: String,

    primary_index: Box<dyn Cursor>,
    index_cursors: Vec<Box<dyn Cursor>>,
}


impl TableCursor {
    /**
     * __curtable_set_key, cur_table.c
     */
    fn set_key(&self, keys: &[&dyn Any]) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

    fn set_value(&self, keys: &[&dyn Any]) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }

    fn insert(&self) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }
}
#![allow(unused)]

use crate::{meta, types::FPResult};

struct Table {
    
}
/**
 * __create_table, scheme_create.c
 * key function:
 *       1. __wt_metadata_insert
 */
fn table_create(tablename: &str) -> FPResult<()> {

    //TODO: implement.
    if let Some(v) = meta::metadata_search("key") {
        return Ok(())
    }

    Ok(())
}
#![allow(unused)]

mod table;
mod checkpoint;

pub(crate) static FP_METAFILE: &str = "FoamPoseidon.fp";

pub(crate) static FP_BLOCK_MAGIC: u32 = 0x64646464;
pub(crate) static FP_BLOCK_MAJOR: u16 = 1;
pub(crate) static FP_BLOCK_MINOR: u16 = 0;


/**
 * Get correspoing metadata with giving key.
 */
pub(crate) fn metadata_search(key: &str) -> Option<()> {

    None
}
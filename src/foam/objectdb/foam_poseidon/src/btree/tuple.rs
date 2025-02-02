#![allow(unused)]

use super::zone_map::{ZMTimeAggregate, ZMTimeWindow};

pub(crate) const FP_BTREE_TUPLE_KEY_INLINE: u8 = 0x01;
pub(crate) const FP_BTREE_TUPLE_KEY_PFX_INLINE: u8 = 0x02;
pub(crate) const FP_BTREE_TUPLE_VALUE_INLINE: u8 = 0x03;

/**
 * On-disk tuple representation.
 * Each tuple represent an in-line key/value, or address info to get key/value.
 * Three types of tuple:
 *      1. Inline: key/value is embedded inside the tuple header(only for short size key/value).
 *      2. Regular: key/value is follow the tuple header.
 *      3. Overflow: key/value is store in seperate page.
 */
#[repr(C)]
pub(crate) struct TupleHeader(&'static [u8]);

impl TupleHeader {
    
}

/**
 * Common tuple fields.
 */
#[repr(C)]
pub(crate) struct TupleCommon {
    header: TupleHeader,
    col_v: u64,      /* Run-Length Encoding or Record Number in column-store */

    data: &'static [u8], /* Data */

    len: u32, /* header + data length */

    raw: u8,
    r#type: u8,
    flags: u8,
}

/**
 * Tuple for internal page.
 * The value of the tuple is the address of children page.
 */
#[repr(C)]
pub(crate) struct TupleIntl {
    common: TupleCommon,
    zm_ta: ZMTimeAggregate,
}

/**
 * Tuple for leaf page.
 * The value of the tuple it the actual value that store in b-tree.
 */
#[repr(C)]
pub(crate) struct TupleLeaf {
    common: TupleCommon,
    zm_tw: ZMTimeWindow,
}
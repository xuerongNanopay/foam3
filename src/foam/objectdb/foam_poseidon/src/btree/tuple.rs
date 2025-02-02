#![allow(unused)]

use super::zone_map::{ZMTimeAggregate, ZMTimeWindow};

pub(crate) const FP_BTREE_TUPLE_INLINE_MASK:u8 = 0x03;
pub(crate) const FP_BTREE_TUPLE_INLINE_SHIFT:u8 = 2;
pub(crate) const FP_BTREE_TUPLE_INLINE_MAX:u64 = 63;

#[repr(usize)]
pub(crate) enum TupleType {
    /**
     * Key/Value is stored in the TupleHeader.
     * The size of key or value should not be greater than FP_BTREE_TUPLE_INLINE_MAX.
     */
    KeyInline = 0x01,
    KeyPrefixInline = 0x02,
    ValueInline = 0x03,
    /**
     * Tuple stores address to other page.
     */
    AddrDel = 0x0,
    AddrInternal = 0x01 << 4,
    AddrLeaf = 0x02 << 4,
    AddrLeafOverflow = 0x03 << 4,
    /**
     * Tuple stores key/value.
     */
    KVDel = 0x04 << 4,
    Key = 0x05 << 4,
    KeyOverflow = 0x06 << 4,
    KeyOverflowDel = 0x07 << 4,
    KeyPrefix = 0x08 << 4,
    Value = 0x09 << 4,
    ValueCopy = 0x10 << 4,
    ValueOverflow = 0x11 << 4,
    ValueOverflowDel = 0x12 << 4,

}

impl TryFrom<usize> for TupleType {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            0x01 => Ok(TupleType::KeyInline),
            0x02 => Ok(TupleType::KeyPrefixInline),
            0x03 => Ok(TupleType::ValueInline),
            _ => Err(()),
        }
    }
}

/**
 * On-disk tuple representation.
 * Each tuple represent an in-line key/value, or address info to get key/value.
 * Three types of tuple store:
 *      1. Inline: key/value is embedded inside the tuple header(only for short size key/value).
 *      2. Normal: key/value is follow the tuple header.
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
#![allow(unused)]

use crate::{FP_BIT_IS_SET, FP_BIT_MASK};

use super::zone_map::{ZMTimeAggregate, ZMTimeWindow};

pub(crate) const FP_BTREE_TUPLE_TYPE_INLINE_MASK:u8 = 0x03;
pub(crate) const FP_BTREE_TUPLE_TYPE_INLINE_SHIFT:u8 = 2;
pub(crate) const FP_BTREE_TUPLE_TYPE_INLINE_MAX:u64 = 63;
pub(crate) const FP_BTREE_TUPLE_TYPE_MASK:u8 = 0x0f << 4;

#[repr(usize)]
#[derive(Debug, Clone, Copy, Default)]
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
    #[default]
    AddrDel = 0x00,
    AddrInternal = 0x10,
    AddrLeaf = 0x20,
    AddrLeafOverflow = 0x30,
    /**
     * Tuple stores key/value.
     */
    KVDel = 0x40,
    Key = 0x50,
    KeyOverflow = 0x60,
    KeyOverflowDel = 0x70,
    KeyPrefix = 0x80,
    Value = 0x90,
    ValueCopy = 0xA0,
    ValueOverflow = 0xB0,
    ValueOverflowDel = 0xC0,

}

impl TupleType {
    #[inline(always)]
    fn to_internal_type(&self) -> TupleType {
        match self {
            TupleType::KeyInline | TupleType::KeyPrefixInline | TupleType::KeyPrefix => TupleType::Key,
            TupleType::ValueInline => TupleType::Value,
            TupleType::KeyOverflowDel => TupleType::KeyOverflow,
            TupleType::ValueOverflowDel => TupleType::ValueOverflow,
            o => o.clone(),
        }
    }
}

impl TryFrom<u8> for TupleType {
    type Error = ();

    #[inline(always)]
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        if FP_BIT_IS_SET!(value, FP_BTREE_TUPLE_TYPE_INLINE_MASK) {
            return match value {
                0x01 => Ok(TupleType::KeyInline),
                0x02 => Ok(TupleType::KeyPrefixInline),
                0x03 => Ok(TupleType::ValueInline),
                _ => panic!("impossible tuple type"),
            };
        }
        let mut v = value;
        FP_BIT_MASK!(v, FP_BTREE_TUPLE_TYPE_MASK);
        return match  v {
            0x00 => Ok(TupleType::AddrDel),
            0x10 => Ok(TupleType::AddrInternal),
            0x20 => Ok(TupleType::AddrLeaf),
            0x30 => Ok(TupleType::AddrLeafOverflow),
            0x40 => Ok(TupleType::KVDel),
            0x50 => Ok(TupleType::Key),
            0x60 => Ok(TupleType::KeyOverflow),
            0x70 => Ok(TupleType::KeyOverflowDel),
            0x80 => Ok(TupleType::KeyPrefix),
            0x90 => Ok(TupleType::Value),
            0xA0 => Ok(TupleType::ValueCopy),
            0xB0 => Ok(TupleType::ValueOverflow),
            0xC0 => Ok(TupleType::ValueOverflowDel),
            _ => panic!("impossible tuple type"),
        };
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
#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct TupleHeader(&'static [u8]);

impl TupleHeader {
    #[inline(always)]
    fn descriptor(&self) -> u8 {
        self.0[0]
    }

    #[inline(always)]
    fn prefix_len(&self) -> u8 {
        self.0[1]
    }

    #[inline(always)]
    fn inline_data_len(&self) -> usize {
        (self.0[0] >> FP_BTREE_TUPLE_TYPE_INLINE_SHIFT) as usize
    }

    #[inline(always)]
    fn secondary_descriptor(&self) -> u8 {
        self.0[2]
    }
}


#[repr(C)]
pub(crate) enum Tuple {
    Internal(TupleInternal),
    Leaf(TupleLeaf)
}

impl Tuple {
    #[inline(always)]
    fn new(tuple_header: &TupleHeader) -> Tuple {
        let descriptor = tuple_header.descriptor();
        let raw_type = TupleType::try_from(descriptor).unwrap();
        let r#type = raw_type.to_internal_type();

        let mut tm = TupleCommon {
            header: *tuple_header,
            raw_type,
            r#type,
            flags: 0,
            ..Default::default()
        };

        match tm.raw_type {
            TupleType::KeyPrefixInline => {
                tm.prefix = tuple_header.prefix_len();
                tm.data = &tuple_header.0[2..tuple_header.inline_data_len()];
                tm.len = 2 + tuple_header.inline_data_len();
            },
            _ => {},
        };
    }
}

/**
 * Common tuple fields.
 */
#[repr(C)]
#[derive(Debug, Copy, Clone, Default)]
pub(crate) struct TupleCommon {
    header: TupleHeader,
    col_v: u64,      /* Run-Length Encoding or Record Number in column-store */

    data: &'static [u8], /* Data */

    len: usize, /* header + data length */

    prefix: u8,

    raw_type: TupleType,
    r#type: TupleType,
    flags: u8,
}

/**
 * Tuple for internal page.
 * The value of the tuple is the address of children page.
 */
#[repr(C)]
pub(crate) struct TupleInternal {
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
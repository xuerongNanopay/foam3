#![allow(unused)]


/**
 * On-disk tuple representation.
 * length is 98 bytes.
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
}

/**
 * Tuple for leaf page.
 * The value of the tuple it the actual value that store in b-tree.
 */
#[repr(C)]
pub(crate) struct TupleLeaf {
    common: TupleCommon,
}
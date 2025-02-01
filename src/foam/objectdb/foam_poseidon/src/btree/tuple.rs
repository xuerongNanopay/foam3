#![allow(unused)]


/**
 * On-disk compact tuple header.
 * length is 98 bytes.
 */
#[repr(C)]
pub(crate) struct TupleHeader(&'static [u8]);

/**
 * Tuple for internal page.
 */
#[repr(C)]
pub(crate) struct TupleAddr {
    header: TupleHeader,
    col_v: u64,      /* Run-Length Encoding or Record Number in column-store */

    data: &'static [u8], /* Data */

    len: u32, /* header + data length */

    raw: u8,
    r#type: u8,
    flags: u8,
}

/**
 * Tuple for leaf page.
 */
#[repr(C)]
pub(crate) struct TupleKV {
    header: TupleHeader,
    col_v: u64,      /* Run-Length Encoding or Record Number in column-store */

    data: &'static [u8], /* Data */

    len: u32, /* header + data length */

    raw: u8,
    r#type: u8,
    flags: u8,
}
#![allow(unused)]

use crate::util::compaction::varint;

#[derive(Default, Clone, Copy)]
struct BlkAddr {
    file_offset: u64,
    size: u64,
    checksum: u32,
    object_id: Option<u32>,
}

impl BlkAddr {
    fn new(raw_addr: &[u8], blk_size: u32) -> BlkAddr {
        let mut p = raw_addr;

        let (file_offset, idx) = varint::decode_uint(p).unwrap();
        p = &p[idx..];
        let (size, idx) = varint::decode_uint(p).unwrap();
        p = &p[idx..];
        let (checksum, idx) = varint::decode_uint(p).unwrap();
        p = &p[idx..];

        if p.len() > 0 {
            //NEED TODO: object_id.
        }
        
        BlkAddr {
            /* The offset 0 is marked as invalid. */
            file_offset: (file_offset + 1) * blk_size as u64,
            size: size * blk_size as u64,
            checksum: checksum as u32,
            ..Default::default()
        }
    }
}
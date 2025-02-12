#![allow(unused)]

use crate::util::compaction::varint;

/**
 * Block address
 */
#[derive(Default, Clone, Copy)]
pub(crate) struct BlkAddr {
    pub(crate) file_offset: u64,
    pub(crate) size: u64,
    pub(crate) checksum: u32,
    pub(crate) source_id: Option<u32>,
}

impl BlkAddr {
    pub(crate) fn new(raw_addr: &[u8], blk_size: u32) -> BlkAddr {
        let mut p = raw_addr;

        let (file_offset, idx) = varint::decode_uint(p).unwrap();
        p = &p[idx..];
        let (size, idx) = varint::decode_uint(p).unwrap();
        p = &p[idx..];
        let (checksum, idx) = varint::decode_uint(p).unwrap();
        p = &p[idx..];

        if p.len() > 0 {
            //NEED TODO: source_id.
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

pub(crate) const FP_BLK_HEADER_CKSUM_INCL_DATA_MK: u8 = 0x01;

/**
 * Block header.
 */
#[repr(C)]
#[derive(Clone, Copy)]
pub(crate) struct BlkHeader {
    pub(crate) disk_size: u32,
    pub(crate) checksum: u32,
    pub(crate) flags: u8,
    pub(crate) unused: [u8;3],
}

impl BlkHeader {
    fn endian_swap(&mut self) {
        if cfg!(target_endian = "big") { 
            self.disk_size = FP_BIT_REVERSE_32!(self.disk_size);
            self.checksum = FP_BIT_REVERSE_32!(self.checksum);
        }
    }
}

impl From<&[u8]> for BlkHeader {
    fn from(raw_data: &[u8]) -> Self {
        let raw_header = FP_REINTERPRET_CAST_BUF!(raw_data, BlkHeader);
        let mut blk_header = *raw_header;
        blk_header.endian_swap();
        blk_header
    }
}
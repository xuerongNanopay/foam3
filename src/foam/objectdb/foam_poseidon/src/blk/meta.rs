#![allow(unused)]

#[derive(Default, Clone, Copy)]
struct BlkAddr {
    object_id: u32,
    offset: usize,
    checksum: u32,
}

impl BlkAddr {
    fn new(raw_addr: &[u8]) -> BlkAddr {
        BlkAddr {
            ..Default::default()
        }
    }
}
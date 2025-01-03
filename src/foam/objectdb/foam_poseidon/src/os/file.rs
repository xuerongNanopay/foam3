pub static FP_FS_OPEN_ACCESS_RAND:u32 = 0x01u32;
pub static FP_FS_OPEN_ACCESS_SEQ:u32  = 0x02u32;
pub static FP_FS_OPEN_CREATE:u32      = 0x04u32;
pub static FP_FS_OPEN_DURABLE:u32     = 0x08u32;
pub static FP_FS_OPEN_EXCLUSIVE:u32   = 0x10u32;
pub static FP_FS_OPEN_FIXED:u32       = 0x20u32;
pub static FP_FS_OPEN_FORCE_MMAP:u32  = 0x40u32;
pub static FP_FS_OPEN_READONLY:u32    = 0x80u32;

pub enum AccessMode {
    Random,
    Sequential
}
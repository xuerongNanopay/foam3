use crate::types::FpErr;

pub mod posix;

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

pub enum FileType {
    Checkpoint,
    Data,
    Directory,
    Log,
    Regular,
}

pub trait FileSystem {
    /**
     * Return a list of file name under given directory
     */
    fn file_ls(dir: &str, prefix: Option<&str>, suffix: Option<&str>) 
        -> Result<Vec<String>, FpErr>;

    /**
     * Return true if file exits.
     */
    fn file_exist(name: &str) -> Result<bool, u32>;

    /**
     * Open a handle for a file.
     */
    fn file_open(name: &str, file_type: FileType, flags: u32);

    /**
     * Remove a file.
     */
    fn file_rm(name: &str, flags: u32);

    /**
     * Rename a file
     */
    fn file_mv(from: &str, to: &str);

    /**
     * Return size of file
     */
    fn file_size(name: &str);

    /**
     * close FileSystem
     */
    fn close();
}

trait FileHandle {
    /**
     * Close a file handle.
     */
    fn close(&self) -> Result<(), FpErr> {
        Ok(())
    }

    /**
     * POSIX only
     */
    fn advise(&self, offset: u64, len: u64, advice: i32) -> Result<(), FpErr> {
        Ok(())
    }

    /**
     * Extend the file.
     */
    fn extend(&self, offset: u64) -> Result<(), FpErr> {
        Ok(())
    }

    /**
     * Extend the file.
     */
    fn extend_nolock(&self, offset: u64) -> Result<(), FpErr> {
        Ok(())
    }

    /**
     * Lock/unlock a file.
     */
    fn lock(&self, offset: u64) -> Result<(), FpErr> {
        Ok(())
    }
}
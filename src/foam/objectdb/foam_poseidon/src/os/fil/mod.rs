use std::{fs, io::Error, path::Path, sync::Arc};

use crate::{errors::*, types::*, FP_IO_ERR};

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
    fn ls(&self, dir: &str, prefix: Option<&str>, suffix: Option<&str>) -> Result<Vec<String>, FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Return true if file exits.
     */
    fn exist(&self, name: &str) -> Result<bool, FPErr>{
        Err(FP_NO_IMPL)
    }

    /**
     * Open a handle for a file.
     */
    fn open(&self, name: &str, file_type: FileType, flags: u32) -> Result<Arc<dyn FileHandle>, FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Remove a file.
     */
    fn rm(&self, name: &str, flags: u32) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Rename a file
     */
    fn rename(&self, from: &str, to: &str) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Return size of file
     */
    fn size(&self, name: &str) -> Result<FileSize, FPErr> {
        Err(FP_NO_IMPL)
    }
    /**
     * close FileSystem
     */
    fn close(&self) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }
}

pub trait FileHandle {
    /**
     * Get file system.
     */
    fn get_file_system(&self) -> Arc<dyn FileSystem>;

    /**
     * Close a file handle.
     */
    fn close(&self) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * POSIX only
     */
    fn advise(&self, offset: FileOffset, len: FileSize, advice: i32) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Extend the file.
     */
    fn extend(&self, offset: FileOffset) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Extend the file.
     */
    fn extend_nolock(&self, offset: FileOffset) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Lock/unlock a file.
     * @param lock whether to lock or unlock
     */
    fn lock(&self, lock: bool) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    //TODO: mmap interface.
    //TODO: file 

    /**
     * Read from file.
     */
    fn read(&self, offset: FileOffset, len: FileSize) -> Result<FileBuf, FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Return size of file.
     */
    fn size(&self) -> Result<FileSize, FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * flush buffered change into file.
     */
    fn sync(&self) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * flush buffered change into file without wait it complete.
     */
    fn sync_nowait(&self) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Truncate file.
     */
    fn truncate(&self, offset: FileOffset) -> Result<FileBuf, FPErr> {
        Err(FP_NO_IMPL)
    }

    /**
     * Write to a file.
     */
    fn write(&self, offset: FileOffset, len: FileSize, buf: FileBuf) -> Result<(), FPErr> {
        Err(FP_NO_IMPL)
    }

}

/**
 * Default file system implementation.
 * Using native rust stb
 */
pub struct DefaultFileSystem {

}

impl FileSystem for DefaultFileSystem {
    fn ls(&self, dir: &str, prefix: Option<&str>, suffix: Option<&str>) -> FPResult<Vec<String>> {
        let entries = FP_IO_ERR!(fs::read_dir(dir));
        let mut ret: Vec<String> = vec![];

        for e in entries {
            let entry = FP_IO_ERR!(e);
            let filename = String::from(entry.file_name().to_string_lossy());

            if let Some(prefix) = prefix {
                if !filename.starts_with(prefix) {
                    continue;
                }
            }

            if let Some(suffix) = suffix {
                if !filename.ends_with(suffix) {
                    continue;
                }
            }
            ret.push(filename);
        }

        Ok(ret)
    }

    fn exist(&self, name: &str) -> Result<bool, FPErr>{
        Ok(Path::new(name).exists())
    }

    fn rm(&self, name: &str, flags: u32) -> Result<(), FPErr> {
        FP_IO_ERR!(fs::remove_file(name));
        Ok(())
    }

    fn rename(&self, from: &str, to: &str) -> Result<(), FPErr> {
        FP_IO_ERR!(fs::rename(from, to));
        Ok(())
    }

    fn size(&self, name: &str) -> Result<FileSize, FPErr> {
        let s = FP_IO_ERR!(fs::metadata(name));
        Ok(s.len())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_file_system_ls() {
        let fs = DefaultFileSystem{};
        println!("{:?}", fs.ls("./", None, None).unwrap());
        println!("{:?}", fs.ls("./", None, Some(".lock")).unwrap());
    }
}

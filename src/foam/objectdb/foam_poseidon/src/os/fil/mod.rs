use std::{fs::{self, File}, io::{Error, Read, Seek, SeekFrom, Write}, mem, path::Path, sync::{Arc, RwLock, Weak}};

use crate::{errors::*, types::*, util::hash_city, FP_IO_ERR};

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

pub trait FileSystem<T: FileHandle> {
    /**
     * Opened file.
     */

    fn open_file_count(&self) -> usize;

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
    fn open(self: &Arc<Self>, name: &str, file_type: FileType, flags: u32) -> Result<Arc<T>, FPErr>;

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
    fn read(&self, offset: FileOffset, len: FileSize) -> Result<(FileBuf, FileSize), FPErr>  {
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
    file_handles: ConcurrentHashMap<String, Arc<DefaultFileHandle>>
}

impl DefaultFileSystem {
    /**
     * Search for an existing handle.
     */
    fn search(&self, name: &str) -> Option<Arc<DefaultFileHandle>> {
        let fds = self.file_handles.read().unwrap();
        match fds.get(name) {
            Some(v) => Some(Arc::clone(v)),
            None => None
        }
    }

    /**
     * Save a filehandle.
     */
    fn save(&self, fh: DefaultFileHandle) {
        let mut fds = self.file_handles.write().unwrap();
        fds.insert(fh.name.clone(), Arc::new(fh));
    }

    /**
     * remove a filehandle.
     */
    fn remove(&self, name: &str) {
        let mut fds = self.file_handles.write().unwrap();
        fds.remove(name);
    }
}

impl FileSystem<DefaultFileHandle> for DefaultFileSystem {
    fn open_file_count(&self) -> usize {
        let fds = self.file_handles.read().unwrap();
        fds.len()
    }

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

    fn open(self: &Arc<Self>, name: &str, file_type: FileType, flags: u32) -> Result<Arc<DefaultFileHandle>, FPErr> {
        if let Some(fh) = self.search(name) {
            return Ok(fh)
        }

        let fd = DefaultFileHandle{
            name: String::from(name),
            fd: RwLock::new(FP_IO_ERR!(File::create_new(name))),
            name_hash: hash_city::city_hash_64(name, name.len()),
            written: 0,
            last_sync: 0,
            file_type,
            file_system: Arc::downgrade(self),
        };

        self.save(fd);

        if let Some(fh) = self.search(name) {
            return Ok(fh)
        }
        Err(FP_IO_NOT_FOUND)
    }
    
}

pub struct DefaultFileHandle {
    name: String, /* file name */
    file_type: FileType,
    name_hash: u64,
    last_sync: u64,
    written: FileSize,
    file_system: Weak<DefaultFileSystem>,
    fd: RwLock<std::fs::File>,
}

impl FileHandle for DefaultFileHandle {
    fn close(&self) -> Result<(), FPErr> {
        if let Some(fs) = self.file_system.upgrade() {
            fs.remove(self.name.as_str());
        }
        self.sync();
        Ok(())
    }

    fn read(&self, offset: FileOffset, len: FileSize) -> Result<(FileBuf, FileSize), FPErr> {

        //TODO: add verbose debug
        //TODO: use read_vectored when len is more than 1GB

        let mut fd = self.fd.write().unwrap();

        // Save current position.
        let cur_position = FP_IO_ERR!(fd.seek(SeekFrom::Current(0)));

        // Read to buffer.
        FP_IO_ERR!(fd.seek(SeekFrom::Start(offset)));
        let mut buffer = vec![0u8; len as usize];
        let read_size = FP_IO_ERR!(fd.read(&mut buffer[..]));

        // Restore position.
        FP_IO_ERR!(fd.seek(SeekFrom::Start(cur_position)));
    
        Ok((buffer, read_size as FileSize))
    }

    fn write(&self, offset: FileOffset, len: FileSize, buf: FileBuf) -> Result<(), FPErr> {
        //TODO: add verbose debug

        
        Err(FP_NO_IMPL)
    }

    /**
     * Return size of file.
     */
    fn size(&self) -> Result<FileSize, FPErr> {
        let fd = self.fd.read().unwrap();
        let metadata = FP_IO_ERR!(fd.metadata());
        Ok(metadata.len())
    }

    /**
     * flush buffered change into file.
     */
    fn sync(&self) -> Result<(), FPErr> {
        let mut fd = self.fd.write().unwrap();
        Ok(FP_IO_ERR!(fd.flush()))
    }

}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, sync::RwLock};

    use super::*;

    #[test]
    fn demo_file_system_ls() {
        let fs = DefaultFileSystem{
            file_handles: RwLock::new(HashMap::new())
        };
        println!("{:?}", fs.ls("./", None, None).unwrap());
        println!("{:?}", fs.ls("./", None, Some(".lock")).unwrap());
    }

    #[test]
    fn demo_hashmap_with_string_key() {
        let mut map = HashMap::<String, String>::new();
        let key = "foo";
        map.insert(key.to_string(), String::from("bb"));
        println!("{}", map.get(key).unwrap());
    }

    #[test]
    fn demo_dynamic_vec_buffer() {
        let mut buffer = vec![0u8; 1000 as usize];
        let mut b = buffer.as_mut_slice();
        let mut c = &mut buffer[..];
        print!("{}", c.len())
    }
}
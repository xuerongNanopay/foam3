use std::{fs::{self, File}, io::{Error, Read, Seek, SeekFrom, Write}, mem, path::Path, sync::{Arc, RwLock, Weak}};

use crate::{error::*, types::*, util::hash_city, FP_IO_ERR};

use super::*;

/**
 * Default file system implementation.
 * Using native rust stb
 */
pub struct DefaultFileSystem {
    file_handles: FPConcurrentHashMap<String, Arc<DefaultFileHandle>>
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

impl FileSystem for DefaultFileSystem {
    type FH = DefaultFileHandle;

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

    fn size(&self, name: &str) -> Result<FPFileSize, FPErr> {
        let s = FP_IO_ERR!(fs::metadata(name));
        Ok(s.len())
    }

    fn open(&self, name: &str, file_type: FileType, flags: u32) -> Result<Arc<DefaultFileHandle>, FPErr> {
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
            file_system: Weak::new(),
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
    written: FPFileSize,
    file_system: Weak<DefaultFileSystem>,
    fd: RwLock<std::fs::File>,
}

impl FileHandle for DefaultFileHandle {
    type FM = DefaultFileSystem;

    fn close(&self) -> Result<(), FPErr> {
        if let Some(fs) = self.file_system.upgrade() {
            fs.remove(self.name.as_str());
        }
        self.sync();
        Ok(())
    }

    fn read(&self, offset: FPFileOffset, len: FPFileSize) -> Result<(FPFileBuf, FPFileSize), FPErr> {

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
    
        Ok((buffer, read_size as FPFileSize))
    }

    fn write(&self, offset: FPFileOffset, len: FPFileSize, buffer: &FPFileBuf) -> Result<(), FPErr> {
        //TODO: add verbose debug

        let mut fd = self.fd.write().unwrap();

        // Save current position.
        let cur_position = FP_IO_ERR!(fd.seek(SeekFrom::Current(0)));

        // Write to buffer
        FP_IO_ERR!(fd.seek(SeekFrom::Start(offset)));
        FP_IO_ERR!(fd.write_all(buffer));

        // Restore position.
        FP_IO_ERR!(fd.seek(SeekFrom::Start(cur_position)));
        Err(FP_NO_IMPL)
    }

    /**
     * Return size of file.
     */
    fn size(&self) -> Result<FPFileSize, FPErr> {
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

    #[test]
    fn demo_arc_self() {
        use std::sync::Arc;

        struct SharedObject {
            data: String,
        }

        impl SharedObject {
            // Create a new instance wrapped in Arc
            fn new(data: String) -> Arc<Self> {
                Arc::new(Self { data })
            }

            // Shared method using Arc<Self>
            fn print(self: &Arc<Self>) {
                println!("Shared data: {}", Arc::strong_count(self));
            }
        }

        let obj = SharedObject::new("Hello, Arc!".to_string());

        // Clone Arc for shared ownership
        let obj_clone = Arc::clone(&obj);

        // Both Arcs can call the print method
        obj.print();        // Outputs: Shared data: Hello, Arc!
        obj_clone.print();  // Outputs: Shared data: Hello, Arc!
        obj.print(); 
    }
}
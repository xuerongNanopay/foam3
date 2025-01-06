#![allow(unused)]

use std::{collections::HashMap, sync::{Arc, RwLock}};

use crate::os::fil::{native::DefaultFileSystem, posix::PosixFileSystem};

pub type FPErr = i32;
pub type FPResult<T> = Result<T, FPErr>;

pub type FPFileOffset = u64;
pub type FPFileSize = u64;
pub type FPFileBuf = Vec<u8>;

pub type FPConcurrentHashMap<K, V> = RwLock<HashMap<K, V>>;

#[cfg(target_os = "linux")]
pub type FPFileSystem = DefaultFileSystem;
#[cfg(target_os = "macos")]
pub type FPFileSystem = DefaultFileSystem;
#[cfg(target_os = "windows")]
pub type FPFileSystem = DefaultFileSystem;
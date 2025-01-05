#![allow(unused)]

use std::{collections::HashMap, sync::{Arc, RwLock}};

pub type FPErr = i32;
pub type FileOffset = u64;
pub type FileSize = u64;
pub type FileBuf = Box<Vec<u8>>;
pub type FPResult<T> = Result<T, FPErr>;

pub type ConcurrentHashMap<K, V> = RwLock<HashMap<K, V>>;
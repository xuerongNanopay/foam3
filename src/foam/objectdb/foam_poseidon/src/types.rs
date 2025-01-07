#![allow(unused)]

use std::{collections::HashMap, sync::{Arc, RwLock}};

pub type FPErr = i32;
pub type FPResult<T> = Result<T, FPErr>;

pub type FPFileOffset = u64;
pub type FPFileSize = u64;
pub type FPFileBuf = Vec<u8>;

pub type FPConcurrentHashMap<K, V> = RwLock<HashMap<K, V>>;
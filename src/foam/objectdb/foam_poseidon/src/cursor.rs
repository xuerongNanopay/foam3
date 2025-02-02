#![allow(unused)]

mod table_cursor;

use std::{any::Any, ptr};

use crate::{error::{FP_NO_ERR, FP_NO_IMPL, FP_NO_SUPPORT}, internal::{FPErr, FPResult}, util::ptr::layout_ptr::LayoutPtr};

pub(crate) trait Cursor {
    fn get_uri(&self) -> &str;
    fn get_key_scheme(&self) -> &str;
    fn get_value_scheme(&self) -> &str;
    fn get_key(&self);
    fn get_value(&self);
    fn set_key(&self, keys: &[&dyn Any]) -> FPResult<()> {
        Err(FP_NO_IMPL)
    }
    fn set_value(&self);
    fn next(&self);
    fn prev(&self);
    fn insert(&self);
    fn update(&self);
    fn remove(&self);
    fn search(&self);
}

pub(crate) struct CursorQueue {
    pub(crate) prev: *mut ICursor,
    pub(crate) next: *mut ICursor,
}

/**
 * Raw representation of the data.
 */
#[derive(Clone)]
pub(crate) struct CursorItem {
    pub(crate) data: Vec<u8>,
    pub(crate) flags: u32,
}

impl CursorItem {
    pub(crate) fn default() -> CursorItem {
        CursorItem{
            data: Vec::with_capacity(0),
            flags: 0,
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }
}

/**
 * Compare key as string.
 */
// pub(crate) struct StringKeyOrd {

// }

// impl KeyOrd for StringKeyOrd {
//     fn compare(&self, v1: &CursorItem, v2: &CursorItem) -> i32 {
//         let l1 = v1.data.len() as i32;
//         let l2 = v1.data.len() as i32;
//         if v1.data.len() == 0 || v2.data.len() == 0 {
//             return l1 - l2;
//         }

//         let min_idx = FP_MIN!(l1, l2) as usize;
//         for i in 0..min_idx {
//             if v1.data[i] != v2.data[i] {
//                 return (v1.data[i] - v2.data[i]) as i32;
//             }
//         }
//         l2 - l1
//     }
// }

pub(crate) type CursorFlag = u64;
pub(crate) const CURSOR_BOUND_LOWER:            CursorFlag = 1 << 0;
pub(crate) const CURSOR_BOUND_LOWER_INCLUSIVE:  CursorFlag = 1 << 1;
pub(crate) const CURSOR_BOUND_UPPER:            CursorFlag = 1 << 2;
pub(crate) const CURSOR_BOUND_UPPER_INCLUSIVE:  CursorFlag = 1 << 3;
pub(crate) const CURSOR_KEY_IN:                 CursorFlag = 1 << 4;
pub(crate) const CURSOR_KEY_OUT:                CursorFlag = 1 << 5;
pub(crate) const CURSOR_VALUE_IN:               CursorFlag = 1 << 6;
pub(crate) const CURSOR_VALUE_OUT:              CursorFlag = 1 << 7;

/* Key is persisted in the datasource/btree */
pub(crate) const CURSOR_KEY_SET:CursorFlag = CURSOR_KEY_IN | CURSOR_KEY_OUT;
/* Value is persisted in the datasource/btree */
pub(crate) const CURSOR_VALUE_SET:CursorFlag = CURSOR_VALUE_IN | CURSOR_VALUE_OUT;

/**
 * Basic Cursor Interface.
 */
pub(crate) struct ICursor {
    pub(crate) uri: String,
    pub(crate) uri_hash: u64,
    pub(crate) key_scheme: String,
    pub(crate) value_scheme: String,
    pub(crate) flags: CursorFlag,
    pub(crate) queue: CursorQueue,
    pub(crate) record_number: u64, /*  */
    /*TODO: real handle*/

    pub(crate) key: CursorItem,
    pub(crate) value: CursorItem,
    pub(crate) save_err: FPErr,

}

impl ICursor {
    pub(crate) fn default() -> ICursor {
        ICursor {
            uri: "".to_owned(),
            uri_hash: 0,
            key_scheme: "".to_owned(),
            value_scheme: "".to_owned(),
            flags: 0,
            queue: CursorQueue {
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
            },
            record_number: 0,
            key: CursorItem::default(),
            value: CursorItem::default(),
            save_err: FP_NO_ERR,
        }
    }

    /**
     * Set key to cursor.
     */
    pub(crate) fn set_key(&mut self, keys: &[&dyn Any]) -> FPResult<()> {

        /* Handle existing keys. */ 
        if self.is_key_set() {
            //TODO: temp save existing key.
        }

        FP_BIT_CLR!(self.flags, CURSOR_KEY_SET);

        if self.key_scheme.starts_with("r") {
            //TODO: column store.
            return Err(FP_NO_IMPL);
        } else {
            // only support raw bytes, string and long.
            if self.key_scheme == "b" {
                // Only raw binary key.
                let key: &&[u8] = keys[0].downcast_ref::<&[u8]>().unwrap();
                let mut key = (*key).to_owned();
                key.shrink_to_fit();
                self.key.data = key;
            } else if self.key_scheme == "S" {
                // Only string key.
                let key: &&str = keys[0].downcast_ref::<&str>().unwrap();
                let mut key = (*key).to_owned().into_bytes();
                key.shrink_to_fit();
                self.key.data = key;
                
            } else {
                // Composite key.
                //TODO: support.
                return Err(FP_NO_SUPPORT)
            }
        }

        Ok(())
    }

    /**
     * Set value to cursor.
     */
    pub(crate) fn set_value(&mut self, values: &[&dyn Any]) -> FPResult<()> {

        FP_BIT_CLR!(self.flags, CURSOR_VALUE_SET);

        if self.value_scheme == "b" {
            // Only raw binary key.
            let value: &&[u8] = values[0].downcast_ref::<&[u8]>().unwrap();
            let mut value = (*value).to_owned();
            value.shrink_to_fit();
            self.value.data = value;
        } else if self.value_scheme == "S" {
            // Only string value.
            let value: &&str = values[0].downcast_ref::<&str>().unwrap();
            let mut value = (*value).to_owned().into_bytes();
            value.shrink_to_fit();
            self.value.data = value;
            
        } else {
            // Composite key.
            //TODO: support.
            return Err(FP_NO_SUPPORT)
        }

        Ok(())
    }

    pub fn is_key_set(&self) -> bool {
        FP_BIT_IS_SET!(self.flags, CURSOR_KEY_SET)
    }
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_string() {
        let a = "aaa";
        let b = a.to_owned();
        println!("{}", b.len());
        let c = b.into_bytes();
        println!("{}", c.len())
    }
}
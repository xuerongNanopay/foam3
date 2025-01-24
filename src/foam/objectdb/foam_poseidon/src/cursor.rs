#![allow(unused)]

mod table_cursor;

use std::{any::Any, ptr};

use crate::{error::{FP_NO_ERR, FP_NO_IMPL, FP_NO_SUPPORT}, types::{FPErr, FPResult}, util::ptr::layout_ptr::LayoutPtr};

pub(super) trait Cursor {
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

struct CursorQueue {
    pub(super) prev: *mut BaseCursor,
    pub(super) next: *mut BaseCursor,
}

/**
 * Raw representation of the data.
 */
 pub(super) struct Item {
    pub(super) data: Option<Vec<u8>>,
    pub(super) flags: u32,
}

impl Item {
    pub(super) fn default() -> Item {
        Item{
            data: None,
            flags: 0,
        }
    }

    pub(super) fn len(&self) -> usize {
        if let Some(d) = &self.data {
            d.len();
        }
        0
    }
}

pub(crate) type CursorFlag = u64;

pub(crate) const CURSOR_KEY_IN: CursorFlag = 1 << 0;
pub(crate) const CURSOR_KEY_OUT: CursorFlag = 1 << 1;
pub(crate) const CURSOR_VALUE_IN: CursorFlag = 1 << 2;
pub(crate) const CURSOR_VALUE_OUT: CursorFlag = 1 << 3;

/* Key is persisted in the datasource/btree */
pub(crate) const CURSOR_KEY_SET:CursorFlag = CURSOR_KEY_IN | CURSOR_KEY_OUT;
/* Value is persisted in the datasource/btree */
pub(crate) const CURSOR_VALUE_SET:CursorFlag = CURSOR_VALUE_IN | CURSOR_VALUE_OUT;

struct BaseCursor {
    pub(super) uri: String,
    pub(super) uri_hash: u64,
    pub(super) key_scheme: String,
    pub(super) value_scheme: String,
    pub(super) flags: CursorFlag,
    pub(super) queue: CursorQueue,
    pub(super) record_no: u64, /*  */
    /*TODO: real handle*/

    pub(super) key: Item,
    pub(super) value: Item,
    pub(super) save_err: FPErr,

}

impl BaseCursor {
    pub(super) fn default() -> BaseCursor {
        BaseCursor {
            uri: "".to_owned(),
            uri_hash: 0,
            key_scheme: "".to_owned(),
            value_scheme: "".to_owned(),
            flags: 0,
            queue: CursorQueue {
                prev: ptr::null_mut(),
                next: ptr::null_mut(),
            },
            record_no: 0,
            key: Item::default(),
            value: Item::default(),
            save_err: FP_NO_ERR,
        }
    }

    /**
     * Set key to cursor.
     */
    pub(super) fn set_key(&mut self, keys: &[&dyn Any]) -> FPResult<()> {

        /* Handle existing keys. */ 
        if self.is_key_set() {
            //TODO: temp save existing key.
        }

        self.flags = FP_BIT_CLR!(self.flags, CURSOR_KEY_SET);

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
                self.key.data = Some(key);
            } else if self.key_scheme == "S" {
                // Only string key.
                let key: &&str = keys[0].downcast_ref::<&str>().unwrap();
                let mut key = (*key).to_owned().into_bytes();
                key.shrink_to_fit();
                self.key.data = Some(key);
                
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
    pub(super) fn set_value(&mut self, values: &[&dyn Any]) -> FPResult<()> {

        self.flags = FP_BIT_CLR!(self.flags, CURSOR_VALUE_SET);

        if self.value_scheme == "b" {
            // Only raw binary key.
            let value: &&[u8] = values[0].downcast_ref::<&[u8]>().unwrap();
            let mut value = (*value).to_owned();
            value.shrink_to_fit();
            self.value.data = Some(value);
        } else if self.value_scheme == "S" {
            // Only string value.
            let value: &&str = values[0].downcast_ref::<&str>().unwrap();
            let mut value = (*value).to_owned().into_bytes();
            value.shrink_to_fit();
            self.value.data = Some(value);
            
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
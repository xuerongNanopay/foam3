#![allow(unused)]

use std::{any::Any, ptr};

use crate::{error::{FP_NO_ERR, FP_NO_IMPL, FP_NO_SUPPORT}, types::{FPErr, FPResult}, util::ptr::layout_ptr::LayoutPtr};

trait CursorA {
    fn get_uri(&self) -> &str;
    fn get_key_scheme(&self) -> &str;
    fn get_value_scheme(&self) -> &str;
    fn get_key(&self);
    fn get_value(&self);
    fn set_key(&self);
    fn set_value(&self);
    fn next(&self);
    fn prev(&self);
    fn insert(&self);
    fn update(&self);
    fn remove(&self);
    fn search(&self);
}

struct CursorQueue {
    pub(super) prev: *mut Cursor,
    pub(super) next: *mut Cursor,
}

/**
 * Raw representation of the data.
 */
 pub(super) struct Item {
    pub(super) data: Option<LayoutPtr<u8>>,
    pub(super) flags: u32,
}

impl Item {
    pub(super) fn default() -> Item {
        Item{
            data: None,
            flags: 0,
        }
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

struct Cursor {
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

impl Cursor {
    pub(super) fn default() -> Cursor {
        Cursor {
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
        let a = &self.key;

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
                
            } else if self.key_scheme == "S" {
                // Only string key.
                let k = keys[0];
                let key: &&str = k.downcast_ref::<&str>().unwrap();
                
            } else {
                // Composite key.
                //TODO: support.
                return Err(FP_NO_SUPPORT)
            }
        }

        Ok(())
    }

    pub fn is_key_set(&self) -> bool {
        FP_BIT_IS_SET!(self.flags, CURSOR_KEY_SET)
    }
}
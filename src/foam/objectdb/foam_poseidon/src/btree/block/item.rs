#![allow(unused)]

use std::{ops::Deref, sync::Arc};

struct BlockItem {
    data: &'static[u8],
    inner: Arc<dyn Deref<Target = [u8]> + Sync>
}

impl BlockItem {
    pub(crate) fn empty() -> BlockItem {
        Self::new(Arc::new(Vec::<u8>::with_capacity(0)))
    }

    pub(crate) fn new (
        data_holder: Arc<dyn Deref<Target = [u8]> + Sync>
    ) -> BlockItem {
        let inner = data_holder.clone();
        let bytes:&[u8] = data_holder.deref();
        /* Skip compile check. */
        let data = unsafe { &*(bytes as *const [u8]) };

        Self {
            data,
            inner,
        }
    }
}
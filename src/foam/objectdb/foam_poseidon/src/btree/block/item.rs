#![allow(unused)]

use std::{ops::{Deref, Range}, sync::Arc};

struct BlockItem {
    data: &'static[u8],
    inner: Arc<dyn Deref<Target = [u8]> + Sync>
}

impl BlockItem {
    pub(crate) fn empty() -> Self {
        Self::new(Arc::new(Vec::<u8>::with_capacity(0)))
    }

    pub(crate) fn new (
        data_holder: Arc<dyn Deref<Target = [u8]> + Sync>
    ) -> Self {
        let inner = data_holder.clone();
        let bytes:&[u8] = data_holder.deref();
        /* Skip compile check. */
        let data = unsafe { &*(bytes as *const [u8]) };

        Self {
            data,
            inner,
        }
    }

    #[must_use]
    #[inline]
    pub(crate) fn slice(&self, range: Range<usize>) -> Self {
        Self {
            data: &self.data[range],
            inner: self.inner.clone(),
        }
    }

    #[inline]
    pub(crate) fn as_slice(&self) -> &[u8] {
        self.data
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    #[must_use]
    pub fn split(self, split_len: usize) -> (Self, Self) {
        let (left_data, right_data) = self.data.split_at(split_len);
        let right_inner = self.inner.clone();
        let left = Self {
            data: left_data,
            inner: self.inner,
        };
        let right = Self {
            data: right_data,
            inner: right_inner,
        };
        (left, right)
    }

    #[inline]
    #[must_use]
    pub fn rsplit(self, split_len: usize) -> (Self, Self) {
        let data_len = self.data.len();
        self.split(data_len - split_len)
    }

    pub fn split_off(&mut self, split_len: usize) -> Self {
        let (left, right) = self.data.split_at(split_len);
        let right_inner: Arc<dyn Deref<Target = [u8]> + Sync> = self.inner.clone();
        let right = Self {
            data: right,
            inner: right_inner,
        };
        self.data = left;
        right
    }
}
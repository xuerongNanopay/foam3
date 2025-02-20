#![allow(unused)]

use core::fmt;
use std::{io, ops::{Deref, Range}, sync::Arc};

#[derive(Clone)]
pub(crate) struct ByteSlice {
    data: &'static[u8],
    inner: Arc<dyn Deref<Target = [u8]> + Sync>
}

impl ByteSlice {
    pub(crate) fn empty() -> Self {
        Self::new(&[][..])
    }

    pub(crate) fn new<T: Deref<Target = [u8]> + 'static + Send + Sync> (
        inner_data: T
    ) -> Self {
        let inner = Arc::new(inner_data);
        let bytes:&[u8] = inner.deref();
        /* Suppress compile check. */
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
    pub(crate) fn len(&self) -> usize {
        self.data.len()
    }

    #[inline]
    pub(crate) fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    #[inline]
    #[must_use]
    pub(crate) fn split(self, split_len: usize) -> (Self, Self) {
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
    pub(crate) fn rsplit(self, split_len: usize) -> (Self, Self) {
        let data_len = self.data.len();
        self.split(data_len - split_len)
    }

    pub(crate) fn split_off(&mut self, split_len: usize) -> Self {
        let (left, right) = self.data.split_at(split_len);
        let right_inner: Arc<dyn Deref<Target = [u8]> + Sync> = self.inner.clone();
        let right = Self {
            data: right,
            inner: right_inner,
        };
        self.data = left;
        right
    }

    #[inline]
    pub(crate) fn advance(&mut self, advance_len: usize) -> &[u8] {
        let (data, rest) = self.data.split_at(advance_len);
        self.data = rest;
        data
    }

    #[inline]
    fn read_n<const N: usize>(&mut self) -> [u8; N] {
        self.advance(N).try_into().unwrap()
    }

    #[inline]
    pub fn read_u8(&mut self) -> u8 {
        self.advance(1)[0]
    }

    #[inline]
    pub fn read_u16(&mut self) -> u16 {
        u16::from_le_bytes(self.read_n())
    }

    #[inline]
    pub fn read_u32(&mut self) -> u32 {
        u32::from_le_bytes(self.read_n())
    }

    #[inline]
    pub fn read_u64(&mut self) -> u64 {
        u64::from_le_bytes(self.read_n())
    }
}

impl fmt::Debug for ByteSlice {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // We truncate the bytes in order to make sure the debug string
        // is not too long.
        let bytes_truncated: &[u8] = if self.len() > 10 {
            &self.as_slice()[..10]
        } else {
            self.as_slice()
        };
        write!(f, "ByteSlice({bytes_truncated:?}, len={})", self.len())
    }
}

impl PartialEq for ByteSlice {
    fn eq(&self, other: &ByteSlice) -> bool {
        self.as_slice() == other.as_slice()
    }
}

impl Eq for ByteSlice {}

impl PartialEq<[u8]> for ByteSlice {
    fn eq(&self, other: &[u8]) -> bool {
        self.as_slice() == other
    }
}

impl PartialEq<str> for ByteSlice {
    fn eq(&self, other: &str) -> bool {
        self.as_slice() == other.as_bytes()
    }
}

impl<'a, T: ?Sized> PartialEq<&'a T> for ByteSlice
where ByteSlice: PartialEq<T>
{
    fn eq(&self, other: &&'a T) -> bool {
        *self == **other
    }
}

impl Deref for ByteSlice {
    type Target = [u8];

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.as_slice()
    }
}

impl AsRef<[u8]> for ByteSlice {
    #[inline]
    fn as_ref(&self) -> &[u8] {
        self.as_slice()
    }
}

impl io::Read for ByteSlice {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let data_len = self.data.len();
        let buf_len = buf.len();
        if data_len >= buf_len {
            let data = self.advance(buf_len);
            buf.copy_from_slice(data);
            Ok(buf_len)
        } else {
            buf[..data_len].copy_from_slice(self.data);
            self.data = &[];
            Ok(data_len)
        }
    }
    #[inline]
    fn read_to_end(&mut self, buf: &mut Vec<u8>) -> io::Result<usize> {
        buf.extend(self.data);
        let read_len = self.data.len();
        self.data = &[];
        Ok(read_len)
    }
    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> io::Result<()> {
        let read_len = self.read(buf)?;
        if read_len != buf.len() {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "failed to fill whole buffer",
            ));
        }
        Ok(())
    }
}
#![allow(unused)]

use core::fmt;
use std::{io, ops::{Deref, Range}, sync::Arc};

use crate::util::bytes::ShareBytes;

pub(crate) type BlockItem = ShareBytes;
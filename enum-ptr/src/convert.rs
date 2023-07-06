use core::mem::transmute;

use crate::{Compact, CompactCopy};

impl<T> From<Compact<T>> for CompactCopy<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
    T: From<CompactCopy<T>> + Copy,
    CompactCopy<T>: From<T>,
{
    fn from(value: Compact<T>) -> Self {
        unsafe { transmute(value) }
    }
}

impl<T> From<CompactCopy<T>> for Compact<T>
where
    T: From<Compact<T>>,
    Compact<T>: From<T>,
    T: From<CompactCopy<T>> + Copy,
    CompactCopy<T>: From<T>,
{
    fn from(value: CompactCopy<T>) -> Self {
        unsafe { transmute(value) }
    }
}

use crate::Aligned;

#[doc(hidden)]
#[repr(C)]
pub struct PtrRepr(pub usize, pub *const u8);

// TODO: impl as many traits as possible

/// Nothing but a zero value. [`UNIT`] is its only instance.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Hash)]
#[repr(transparent)]
pub struct Unit(*const ());

pub const UNIT: Unit = Unit(core::ptr::null());

unsafe impl Aligned for Unit {
    const ALIGNMENT: usize = usize::MAX;
}

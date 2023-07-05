use crate::Aligned;

#[doc(hidden)]
#[repr(C)]
pub struct PtrRepr(pub usize, pub *const u8);

/// Nothing but zeros. [`UNIT`] is its only instance.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
#[repr(transparent)]
pub struct Unit(usize);

pub const UNIT: Unit = Unit(0);

unsafe impl Aligned for Unit {
    const ALIGNMENT: usize = usize::MAX;
}

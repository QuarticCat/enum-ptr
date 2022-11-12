use core::mem::MaybeUninit;

#[doc(hidden)]
#[repr(C)]
pub struct PtrRepr(pub usize, pub *const u8);

#[doc(hidden)]
#[repr(C)]
pub struct UnitRepr(pub usize, pub MaybeUninit<*const u8>);

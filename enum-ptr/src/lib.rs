#![cfg_attr(not(feature = "std"), no_std)]

mod aligned;
mod compact;

pub use aligned::*;
pub use compact::*;

pub use enum_ptr_derive::EnumPtr;

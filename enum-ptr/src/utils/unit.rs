use crate::Aligned;

/// Placeholder of unit variants.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct Unit(usize);

impl Unit {
    pub fn new() -> Self {
        Self(0)
    }
}

unsafe impl Aligned for Unit {
    const ALIGNMENT: usize = usize::MAX;
}

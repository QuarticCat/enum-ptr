use crate::Aligned;

/// [`isize`] that shifts left by `N` bits.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct ShiftIsize<const N: isize>(isize);

impl<const N: isize> ShiftIsize<N> {
    /// Creates a new value from an unshifted number.
    #[inline]
    pub fn new(val: isize) -> Self {
        Self(val << N)
    }

    /// Returns the unshifted number.
    #[inline]
    pub fn get(&self) -> isize {
        self.0 >> N
    }

    /// Sets the value by an unshifted number.
    #[inline]
    pub fn set(&mut self, val: isize) {
        self.0 = val << N;
    }
}

unsafe impl<const N: isize> Aligned for ShiftIsize<N> {
    const ALIGNMENT: usize = 1 << N;
}

/// [`usize`] that shifts left by `N` bits.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash)]
pub struct ShiftUsize<const N: usize>(usize);

impl<const N: usize> ShiftUsize<N> {
    /// Creates a new value from a unshifted number.
    #[inline]
    pub fn new(val: usize) -> Self {
        Self(val << N)
    }

    /// Returns the unshifted number.
    #[inline]
    pub fn get(&self) -> usize {
        self.0 >> N
    }

    /// Sets the value by an unshifted number.
    #[inline]
    pub fn set(&mut self, val: usize) {
        self.0 = val << N;
    }
}

unsafe impl<const N: usize> Aligned for ShiftUsize<N> {
    const ALIGNMENT: usize = 1 << N;
}

// TODO: impl more traits for these two types, like `From<usize>`

use std::{fmt::Display, range::Step};

/// Unsigned finite natural number type, with values in [0, N].
/// Used for indexing a collection of N + 1 elements.
#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord)]
pub struct FiniteIndex<const N: usize>(usize);

impl<const N: usize> FiniteIndex<N> {
    pub const ZERO: FiniteIndex<N> = FiniteIndex(0);
    pub const MAX: FiniteIndex<N> = FiniteIndex(N);
    pub const COUNT: usize = N + 1;

    /// Creates a FiniteIndex from a raw usize value without checking bounds.
    pub const fn raw(value: usize) -> Self {
        FiniteIndex(value)
    }

    /// Shifts the value by the given amount,
    /// staying within bounds by capping/saturating at the edges.
    pub fn shift(&self, by: isize) -> Self {
        FiniteIndex(self.0.saturating_add_signed(by).clamp(0, N))
    }

    /// Adds the given amount to the value,
    /// staying within bounds by capping at the maximum.
    pub fn add(&self, by: usize) -> Self {
        FiniteIndex(self.0.saturating_add(by).min(N))
    }

    /// Subtracts the given amount from the value,
    /// staying within bounds by capping at zero.
    pub fn sub(&self, by: usize) -> Self {
        FiniteIndex(self.0.saturating_sub(by))
    }
}

impl<const N: usize> From<FiniteIndex<N>> for usize {
    fn from(value: FiniteIndex<N>) -> Self {
        value.0
    }
}

impl<const N: usize> TryFrom<usize> for FiniteIndex<N> {
    type Error = String;

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        if value <= N {
            Ok(FiniteIndex(value))
        } else {
            Err(format!(
                "FiniteIndex out of bounds error: {} >= {}",
                value, N
            ))
        }
    }
}

impl<const N: usize> TryFrom<FiniteIndex<N>> for isize {
    type Error = &'static str;

    fn try_from(value: FiniteIndex<N>) -> Result<Self, Self::Error> {
        value
            .0
            .try_into()
            .map_err(|_| "FiniteIndex conversion to isize failed")
    }
}

impl<const N: usize> Step for FiniteIndex<N> {
    fn steps_between(start: &Self, end: &Self) -> (usize, Option<usize>) {
        usize::steps_between(&usize::from(*start), &usize::from(*end))
    }

    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        if (usize::from(start) + count) <= N {
            Some(start.add(count))
        } else {
            None
        }
    }

    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        if usize::from(start) >= count {
            start.sub(count).into()
        } else {
            None
        }
    }
}

impl<const N: usize> Display for FiniteIndex<N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

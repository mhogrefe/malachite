use std::iter::{empty, Empty};

/// `Never` is a type that cannot be instantiated.
///
/// In other languages this type may be called `Nothing`, `Empty`, or `Void`.
///
/// # Examples
/// ```
/// use malachite_base::nevers::Never;
///
/// let x: Option<Never> = None;
/// ```
#[derive(Clone, Copy, Debug, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub enum Never {}

/// Generates all (none) of the `Never`s.
///
/// The output length is 0.
///
/// # Worst-case complexity per iteration
///
/// Constant time and additional memory.
///
/// # Examples
/// ```
/// use malachite_base::nevers::nevers;
///
/// assert_eq!(nevers().collect::<Vec<_>>(), &[]);
/// ```
pub const fn nevers() -> Empty<Never> {
    empty()
}

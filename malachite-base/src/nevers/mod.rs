use std::iter::{empty, Empty};
use std::str::FromStr;

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

impl FromStr for Never {
    type Err = &'static str;

    /// Would convert a `String` to a `Never`.
    ///
    /// Since a `Never` can never be instantiated, `from_str` never succeeds.
    ///
    /// # Worst-case complexity
    ///
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::nevers::Never;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Never::from_str("abc"), Err("Never has no possible values"));
    /// ```
    #[inline]
    fn from_str(_: &str) -> Result<Never, &'static str> {
        Err("Never has no possible values")
    }
}

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
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::nevers::nevers;
///
/// assert_eq!(nevers().collect_vec(), &[]);
/// ```
pub const fn nevers() -> Empty<Never> {
    empty()
}

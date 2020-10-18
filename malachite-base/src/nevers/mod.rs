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
    /// TODO
    /// Since a `Never` can never be instantiated, `from_str`
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ = `src.len()`.
    ///
    /// The worst case occurs when the input `&str` is invalid and must be copied into an `Err`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(RoundingMode::from_str("Down"), Ok(RoundingMode::Down));
    /// assert_eq!(RoundingMode::from_str("Up"), Ok(RoundingMode::Up));
    /// assert_eq!(RoundingMode::from_str("Floor"), Ok(RoundingMode::Floor));
    /// assert_eq!(RoundingMode::from_str("Ceiling"), Ok(RoundingMode::Ceiling));
    /// assert_eq!(RoundingMode::from_str("Nearest"), Ok(RoundingMode::Nearest));
    /// assert_eq!(RoundingMode::from_str("Exact"), Ok(RoundingMode::Exact));
    /// assert_eq!(RoundingMode::from_str("abc"), Err("abc".to_string()));
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
/// use malachite_base::nevers::nevers;
///
/// assert_eq!(nevers().collect::<Vec<_>>(), &[]);
/// ```
pub const fn nevers() -> Empty<Never> {
    empty()
}

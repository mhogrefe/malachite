use std::fmt::{self, Display, Formatter};

use rounding_modes::RoundingMode;

impl Display for RoundingMode {
    /// Converts a `RoundingMode` to a `String`.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::rounding_modes::RoundingMode;
    ///
    /// assert_eq!(RoundingMode::Down.to_string(), "Down");
    /// assert_eq!(RoundingMode::Up.to_string(), "Up");
    /// assert_eq!(RoundingMode::Floor.to_string(), "Floor");
    /// assert_eq!(RoundingMode::Ceiling.to_string(), "Ceiling");
    /// assert_eq!(RoundingMode::Nearest.to_string(), "Nearest");
    /// assert_eq!(RoundingMode::Exact.to_string(), "Exact");
    /// ```
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

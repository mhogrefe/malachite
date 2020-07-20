use rounding_modes::RoundingMode;
use std::str::FromStr;

impl FromStr for RoundingMode {
    type Err = String;

    /// Converts a `String` to a `RoundingMode`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `src.len()`
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
    fn from_str(src: &str) -> Result<RoundingMode, String> {
        match src {
            "Down" => Ok(RoundingMode::Down),
            "Up" => Ok(RoundingMode::Up),
            "Floor" => Ok(RoundingMode::Floor),
            "Ceiling" => Ok(RoundingMode::Ceiling),
            "Nearest" => Ok(RoundingMode::Nearest),
            "Exact" => Ok(RoundingMode::Exact),
            _ => Err(src.to_string()),
        }
    }
}

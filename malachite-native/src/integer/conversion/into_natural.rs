use integer::Integer;
use natural::Natural;

impl Integer {
    /// Converts an `Integer` to a `Natural`, consuming the `Integer`. If the `Integer` is negative,
    /// `None` is returned.
    ///
    /// # Examples
    /// ```
    /// use malachite_native::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(format!("{:?}", Integer::from(123).into_natural()), "Some(123)");
    /// assert_eq!(format!("{:?}", Integer::from(-123).into_natural()), "None");
    /// assert_eq!(format!("{:?}", Integer::from_str("1000000000000").unwrap().into_natural()),
    ///            "Some(1000000000000)");
    /// assert_eq!(format!("{:?}", Integer::from_str("-1000000000000").unwrap().into_natural()),
    ///            "None");
    /// ```
    pub fn into_natural(self) -> Option<Natural> {
        match self {
            Integer { sign: false, .. } => None,
            Integer { sign: true, abs } => Some(abs),
        }
    }
}

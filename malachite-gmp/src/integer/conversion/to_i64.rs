use integer::Integer::{self, Large, Small};
use natural::Natural;
use std::cmp::Ordering;

impl Integer {
    /// Converts an `Integer` to an `i64`, returning `None` if the `Integer` is outside the range of
    /// an `i64`.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(format!("{:?}", Integer::from(123).to_i64()), "Some(123)");
    /// assert_eq!(format!("{:?}", Integer::from(-123).to_i64()), "Some(-123)");
    /// assert_eq!(
    ///     format!("{:?}", Integer::from_str("1000000000000000000000000").unwrap().to_i64()),
    ///     "None");
    /// assert_eq!(format!("{:?}",
    ///     Integer::from_str("-1000000000000000000000000").unwrap().to_i64()),
    ///     "None");
    /// ```
    pub fn to_i64(&self) -> Option<i64> {
        match *self {
            Small(small) => Some(small as i64),
            Large(_) => {
                if self.significant_bits() < 64 ||
                    *self == -((Natural::from(1u32) << 63).into_integer())
                {
                    let abs = self.abs_ref().to_u64().unwrap() as i64;
                    Some(if self.sign() == Ordering::Less {
                        -abs
                    } else {
                        abs
                    })
                } else {
                    None
                }
            }
        }
    }

    /// Converts an `Integer` to a `i64`, wrapping mod 2^(64).
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Integer::from(123).to_i64_wrapping().to_string(), "123");
    /// assert_eq!(Integer::from(-123).to_i64_wrapping().to_string(), "-123");
    /// assert_eq!(
    ///     Integer::from_str("1000000000000000000000000").unwrap().to_i64_wrapping().to_string(),
    ///     "2003764205206896640");
    /// assert_eq!(
    ///     Integer::from_str("-1000000000000000000000000").unwrap().to_i64_wrapping().to_string(),
    ///     "-2003764205206896640");
    /// ```
    pub fn to_i64_wrapping(&self) -> i64 {
        self.to_u64_wrapping() as i64
    }
}

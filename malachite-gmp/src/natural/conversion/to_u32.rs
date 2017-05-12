use gmp_mpfr_sys::gmp;
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Converts a `Natural` to a `u32`, returning `None` if the `Natural` is too large.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(format!("{:?}", Natural::from(123).to_u32()), "Some(123)");
    /// assert_eq!(format!("{:?}", Natural::from_str("1000000000000").unwrap().to_u32()), "None");
    /// ```
    pub fn to_u32(&self) -> Option<u32> {
        match *self {
            Small(x) => Some(x),
            Large(_) => None,
        }
    }

    /// Converts a `Natural` to a `u32`, wrapping mod 2^(32).
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(123).to_u32_wrapping().to_string(), "123");
    /// assert_eq!(Natural::from_str("1000000000000").unwrap().to_u32_wrapping().to_string(),
    ///            "3567587328");
    /// ```
    pub fn to_u32_wrapping(&self) -> u32 {
        match *self {
            Small(x) => x,
            Large(ref x) => unsafe { gmp::mpz_get_ui(x) as u32 },
        }
    }
}

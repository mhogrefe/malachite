use gmp_mpfr_sys::gmp;
use natural::Natural::{self, Large, Small};

impl Natural {
    /// Converts a `Natural` to a `u64`, returning `None` if the `Natural` is too large.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(format!("{:?}", Natural::from(123u32).to_u64()), "Some(123)");
    /// assert_eq!(format!("{:?}", Natural::from_str("1000000000000000000000").unwrap().to_u64()),
    ///            "None");
    /// ```
    pub fn to_u64(&self) -> Option<u64> {
        match *self {
            Small(small) => Some(small.into()),
            Large(ref large) if unsafe { gmp::mpz_sizeinbase(large, 2) } <= 64 => unsafe {
                Some(gmp::mpz_get_ui(large))
            },
            Large(_) => None,
        }
    }

    /// Converts a `Natural` to a `u64`, wrapping mod 2^(64).
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Natural::from(123u32).to_u64_wrapping().to_string(), "123");
    /// assert_eq!(Natural::from_str("1000000000000000000000")
    ///                 .unwrap().to_u64_wrapping().to_string(),
    ///            "3875820019684212736");
    /// ```
    pub fn to_u64_wrapping(&self) -> u64 {
        match *self {
            Small(small) => small.into(),
            Large(ref large) => unsafe { gmp::mpz_get_ui(large) },
        }
    }
}

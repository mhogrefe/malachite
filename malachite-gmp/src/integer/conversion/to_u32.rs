use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};
use std::cmp::Ordering;

impl Integer {
    /// Converts an `Integer` to a `u32`, returning `None` if the `Integer` is too large.
    ///
    /// # Example
    /// ```
    /// use malachite_gmp::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(format!("{:?}", Integer::from(123).to_u32()), "Some(123)");
    /// assert_eq!(format!("{:?}", Integer::from(-123).to_u32()), "None");
    /// assert_eq!(format!("{:?}", Integer::from_str("1000000000000").unwrap().to_u32()), "None");
    /// assert_eq!(format!("{:?}", Integer::from_str("-1000000000000").unwrap().to_u32()), "None");
    /// ```
    pub fn to_u32(&self) -> Option<u32> {
        if self.sign() == Ordering::Less {
            return None;
        }
        match *self {
            Small(x) => Some(x as u32),
            Large(x) => {
                if *self <= u32::max_value() {
                    Some(unsafe { gmp::mpz_get_ui(&x) as u32 })
                } else {
                    None
                }
            }
        }
    }

    /// Converts an `Integer` to a `u32`, wrapping mod 2^(32).
    ///
    /// # Example
    /// ```
    /// use malachite_gmp::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Integer::from(123).to_u32_wrapping().to_string(), "123");
    /// assert_eq!(Integer::from(-123).to_u32_wrapping().to_string(), "4294967173");
    /// assert_eq!(Integer::from_str("1000000000000").unwrap().to_u32_wrapping().to_string(),
    ///            "3567587328");
    /// assert_eq!(Integer::from_str("-1000000000000").unwrap().to_u32_wrapping().to_string(),
    ///            "727379968");
    /// ```
    pub fn to_u32_wrapping(&self) -> u32 {
        match *self {
            Small(x) => x as u32,
            Large(ref x) => {
                let u = unsafe { gmp::mpz_get_ui(x) } as u32;
                if self.sign() != Ordering::Less {
                    u
                } else {
                    u.wrapping_neg()
                }
            }
        }
    }
}

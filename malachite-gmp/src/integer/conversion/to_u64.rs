use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use integer::make_u64;
use std::cmp::Ordering;
use std::mem;

impl Integer {
    /// Converts an `Integer` to a `u64`, returning `None` if the `Integer` is negative or too
    /// large.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(format!("{:?}", Integer::from(123).to_u64()), "Some(123)");
    /// assert_eq!(format!("{:?}", Integer::from(-123).to_u64()), "None");
    /// assert_eq!(format!("{:?}", Integer::from_str("1000000000000000000000").unwrap().to_u64()),
    ///                            "None");
    /// assert_eq!(format!("{:?}", Integer::from_str("-1000000000000000000000").unwrap().to_u64()),
    ///                            "None");
    /// ```
    pub fn to_u64(&self) -> Option<u64> {
        if self.sign() == Ordering::Less {
            return None;
        }
        match *self {
            Small(small) => Some(small as u64),
            Large(ref large) => {
                if self.significant_bits() <= 64 {
                    unsafe {
                        let mut copy: mpz_t = mem::uninitialized();
                        gmp::mpz_init_set(&mut copy, large);
                        let lower = gmp::mpz_get_ui(&copy) as u32;
                        gmp::mpz_tdiv_q_2exp(&mut copy, &copy, 32);
                        let upper = gmp::mpz_get_ui(&copy) as u32;
                        Some(make_u64(upper, lower))
                    }
                } else {
                    None
                }
            }
        }
    }

    /// Converts an `Integer` to a `u64`, wrapping mod 2^(64).
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(Integer::from(123).to_u64_wrapping().to_string(), "123");
    /// assert_eq!(Integer::from(-123).to_u64_wrapping().to_string(), "18446744073709551493");
    /// assert_eq!(
    ///     Integer::from_str("1000000000000000000000").unwrap().to_u64_wrapping().to_string(),
    ///     "3875820019684212736");
    /// assert_eq!(
    ///     Integer::from_str("-1000000000000000000000").unwrap().to_u64_wrapping().to_string(),
    ///     "14570924054025338880");
    /// ```
    pub fn to_u64_wrapping(&self) -> u64 {
        match *self {
            Small(small) => small as u64,
            Large(ref large) => unsafe {
                let mut copy: mpz_t = mem::uninitialized();
                gmp::mpz_init_set(&mut copy, large);
                let lower = gmp::mpz_get_ui(&copy) as u32;
                gmp::mpz_tdiv_q_2exp(&mut copy, &copy, 32);
                let upper = gmp::mpz_get_ui(&copy) as u32;
                let result = make_u64(upper, lower);
                if self.sign() == Ordering::Less {
                    result.wrapping_neg()
                } else {
                    result
                }
            },
        }
    }
}

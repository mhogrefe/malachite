use gmp_mpfr_sys::gmp;
use natural::{get_limb_size, LimbSize, make_u64};
use natural::Natural::{self, Large, Small};
use std::slice::from_raw_parts;

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
                Some(match get_limb_size() {
                    LimbSize::U32 => {
                        let raw_limbs =
                            from_raw_parts(gmp::mpz_limbs_read(large), gmp::mpz_size(large));
                        make_u64(raw_limbs[1] as u32, raw_limbs[0] as u32)
                    }
                    LimbSize::U64 => gmp::mpz_get_ui(large),
                })
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
            Large(ref large) => unsafe {
                match get_limb_size() {
                    LimbSize::U32 => {
                        let raw_limbs =
                            from_raw_parts(gmp::mpz_limbs_read(large), gmp::mpz_size(large));
                        make_u64(raw_limbs[1] as u32, raw_limbs[0] as u32)
                    }
                    LimbSize::U64 => gmp::mpz_get_ui(large),
                }
            },
        }
    }
}

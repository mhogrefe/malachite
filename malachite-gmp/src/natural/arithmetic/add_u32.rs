use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::Natural::{self, Large, Small};
use std::mem;
use std::ops::{Add, AddAssign};

/// Adds a `u32` to a `Natural`, taking ownership of the input `Natural`.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((Natural::from(0u32) + 123).to_string(), "123");
/// assert_eq!((Natural::from(123u32) + 0).to_string(), "123");
/// assert_eq!((Natural::from(123u32) + 456).to_string(), "579");
/// assert_eq!((Natural::from_str("1000000000000").unwrap() + 123).to_string(), "1000000000123");
/// ```
impl Add<u32> for Natural {
    type Output = Natural;

    fn add(mut self, other: u32) -> Natural {
        self += other;
        self
    }
}

/// Adds a `Natural` to a `u32`, taking ownership of the input `Natural`.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((123 + Natural::from(0u32)).to_string(), "123");
/// assert_eq!((0 + Natural::from(123u32)).to_string(), "123");
/// assert_eq!((456 + Natural::from(123u32)).to_string(), "579");
/// assert_eq!((123 + Natural::from_str("1000000000000").unwrap()).to_string(), "1000000000123");
/// ```
impl Add<Natural> for u32 {
    type Output = Natural;

    fn add(self, mut other: Natural) -> Natural {
        other += self;
        other
    }
}

/// Adds a `u32` to a `Natural` in place.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
///
/// let mut x = Natural::new();
/// x += 1;
/// x += 2;
/// x += 3;
/// x += 4;
/// assert_eq!(x.to_string(), "10");
/// ```
impl AddAssign<u32> for Natural {
    fn add_assign(&mut self, other: u32) {
        if other == 0 {
            return;
        }
        mutate_with_possible_promotion!(self,
                                        small,
                                        large,
                                        {
                                            small.checked_add(other)
                                        },
                                        {
                                            unsafe { gmp::mpz_add_ui(large, large, other.into()) }
                                        });
    }
}

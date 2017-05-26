use gmp_mpfr_sys::gmp;
use natural::Natural::{self, Large, Small};
use std::ops::{Add, AddAssign};

/// Adds a `Natural` to a `Natural`, taking ownership of both `Natural`s.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((Natural::from(0u32) + Natural::from(123u32)).to_string(), "123");
/// assert_eq!((Natural::from(123u32) + Natural::from(0u32)).to_string(), "123");
/// assert_eq!((Natural::from(123u32) + Natural::from(456u32)).to_string(), "579");
/// assert_eq!((Natural::from_str("1000000000000").unwrap() + Natural::from_str("2000000000000")
///            .unwrap()).to_string(), "3000000000000");
/// ```
impl Add<Natural> for Natural {
    type Output = Natural;

    fn add(mut self, other: Natural) -> Natural {
        self += other;
        self
    }
}

/// Adds a `Natural` to a `Natural` in place.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// let mut x = Natural::new();
/// x += Natural::from_str("1000000000000").unwrap();
/// x += Natural::from_str("2000000000000").unwrap();
/// x += Natural::from_str("3000000000000").unwrap();
/// x += Natural::from_str("4000000000000").unwrap();
/// assert_eq!(x.to_string(), "10000000000000");
/// ```
impl AddAssign<Natural> for Natural {
    fn add_assign(&mut self, mut other: Natural) {
        if other == 0 {
            return;
        }
        if let Small(y) = other {
            *self += y;
        } else if let Small(x) = *self {
            other += x;
            *self = other;
        } else {
            match (self, other) {
                (&mut Large(ref mut x), Large(ref y)) => unsafe {
                    gmp::mpz_add(x, x, y);
                },
                _ => unreachable!(),
            }
        }
    }
}

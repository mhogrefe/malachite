use gmp_mpfr_sys::gmp;
use natural::Natural::{self, Large, Small};
use std::ops::{Sub, SubAssign};

/// Subtracts a `Natural` from a `Natural`, taking ownership of both `Natural`s.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!(format!("{:?}", Natural::from(0u32) - Natural::from(123u32)), "None");
/// assert_eq!(format!("{:?}", Natural::from(123u32) - Natural::from(0u32)), "Some(123)");
/// assert_eq!(format!("{:?}", Natural::from(456u32) - Natural::from(123u32)), "Some(333)");
/// assert_eq!(format!("{:?}", Natural::from_str("3000000000000").unwrap() -
///                            Natural::from_str("1000000000000").unwrap()), "Some(2000000000000)");
/// ```
impl Sub<Natural> for Natural {
    type Output = Option<Natural>;

    fn sub(mut self, other: Natural) -> Option<Natural> {
        if self >= other {
            self -= other;
            Some(self)
        } else {
            None
        }
    }
}

/// Subtracts a `Natural` from a `Natural` in place.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// let mut x = Natural::from_str("10000000000000").unwrap();
/// x -= Natural::from_str("1000000000000").unwrap();
/// x -= Natural::from_str("2000000000000").unwrap();
/// x -= Natural::from_str("3000000000000").unwrap();
/// x -= Natural::from_str("4000000000000").unwrap();
/// assert_eq!(x.to_string(), "0");
/// ```
impl SubAssign<Natural> for Natural {
    fn sub_assign(&mut self, other: Natural) {
        if other == 0 {
            return;
        }
        if let Small(y) = other {
            *self -= y;
        } else if let Small(_) = *self {
            panic!("Cannot subtract a Natural from a smaller Natural. self: {}, other: {}",
                   *self,
                   other);
        } else if *self < other {
            panic!("Cannot subtract a Natural from a smaller Natural. self: {}, other: {}",
                   *self,
                   other);
        } else {
            match (&mut (*self), other) {
                (&mut Large(ref mut x), Large(ref y)) => unsafe {
                    gmp::mpz_sub(x, x, y);
                },
                _ => unreachable!(),
            }
        }
        self.demote_if_small();
    }
}

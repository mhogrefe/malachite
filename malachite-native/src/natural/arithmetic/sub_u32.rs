use natural::Natural::{self, Large, Small};
use std::ops::{Sub, SubAssign};

/// Subtracts a `u32` from a `Natural`, taking ownership of the input `Natural`.
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!(format!("{:?}", Natural::from(123u32) - 123), "Some(0)");
/// assert_eq!(format!("{:?}", Natural::from(123u32) - 0), "Some(123)");
/// assert_eq!(format!("{:?}", Natural::from(456u32) - 123), "Some(333)");
/// assert_eq!(format!("{:?}", Natural::from(123u32) - 456), "None");
/// assert_eq!(format!("{:?}", Natural::from_str("1000000000000").unwrap() - 123),
///            "Some(999999999877)");
/// ```
impl Sub<u32> for Natural {
    type Output = Option<Natural>;

    fn sub(mut self, other: u32) -> Option<Natural> {
        if self >= other {
            self.sub_assign(other);
            Some(self)
        } else {
            None
        }
    }
}

/// Subtracts a `Natural` from a `u32`, taking ownership of the input `Natural`. If the `Natural`
/// is greater than the `u32`, returns `None`.
///
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!(format!("{:?}", 123 - Natural::from(123u32)), "Some(0)");
/// assert_eq!(format!("{:?}", 123 - Natural::from(0u32)), "Some(123)");
/// assert_eq!(format!("{:?}", 456 - Natural::from(123u32)), "Some(333)");
/// assert_eq!(format!("{:?}", 123 - Natural::from(456u32)), "None");
/// assert_eq!(format!("{:?}", 123 - Natural::from_str("1000000000000").unwrap()), "None");
/// ```
impl Sub<Natural> for u32 {
    type Output = Option<Natural>;

    fn sub(self, other: Natural) -> Option<Natural> {
        other.to_u32().and_then(|x| self.checked_sub(x)).map(Natural::from)
    }
}

/// Subtracts a `u32` from a `Natural` in place.
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Example
/// ```
/// use malachite_native::natural::Natural;
///
/// let mut x = Natural::from(15u32);
/// x -= 1;
/// x -= 2;
/// x -= 3;
/// x -= 4;
/// assert_eq!(x.to_string(), "5");
/// ```
impl SubAssign<u32> for Natural {
    fn sub_assign(&mut self, other: u32) {
        if other == 0 {
            return;
        }
        let mut panic = false;
        match *self {
            Small(ref mut small) => {
                match small.checked_sub(other) {
                    Some(difference) => *small = difference,
                    None => panic = true,
                }
            }
            Large(ref mut limbs) => {
                let mut subtrahend = other;
                for limb in limbs.iter_mut() {
                    let (difference, overflow) = limb.overflowing_sub(subtrahend);
                    *limb = difference;
                    if overflow {
                        subtrahend = 1;
                    } else {
                        subtrahend = 0;
                        break;
                    }
                }
                if subtrahend == 1 {
                    panic = true;
                }
            }
        }
        if panic {
            panic!("Cannot subtract a u32 from a smaller Natural. self: {}, other: {}",
                   *self,
                   other);
        }
        self.trim();
    }
}

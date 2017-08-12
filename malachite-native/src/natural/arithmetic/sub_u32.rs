use natural::Natural::{self, Large, Small};
use std::ops::{Sub, SubAssign};

/// Subtracts a `u32` from a `Natural`, taking the `Natural` by value. If the `u32` is greater than
/// the `Natural`, returns `None`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
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
            self -= other;
            Some(self)
        } else {
            None
        }
    }
}

/// Subtracts a `u32` from a `Natural`, taking the `Natural` by reference. If the `u32` is greater
/// than the `Natural`, returns `None`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!(format!("{:?}", &Natural::from(123u32) - 123), "Some(0)");
/// assert_eq!(format!("{:?}", &Natural::from(123u32) - 0), "Some(123)");
/// assert_eq!(format!("{:?}", &Natural::from(456u32) - 123), "Some(333)");
/// assert_eq!(format!("{:?}", &Natural::from(123u32) - 456), "None");
/// assert_eq!(format!("{:?}", &Natural::from_str("1000000000000").unwrap() - 123),
///            "Some(999999999877)");
/// ```
impl<'a> Sub<u32> for &'a Natural {
    type Output = Option<Natural>;

    fn sub(self, other: u32) -> Option<Natural> {
        if other == 0 {
            return Some(self.clone());
        }
        match *self {
            Small(small) => small.checked_sub(other).map(Small),
            Large(ref limbs) => {
                let mut subtrahend = other;
                let mut difference_limbs = Vec::with_capacity(limbs.len());
                for limb in limbs.iter() {
                    if subtrahend == 0 {
                        difference_limbs.push(*limb);
                    } else {
                        let (difference, overflow) = limb.overflowing_sub(subtrahend);
                        difference_limbs.push(difference);
                        if overflow {
                            subtrahend = 1;
                        } else {
                            subtrahend = 0;
                        }
                    }
                }
                if subtrahend == 1 {
                    None
                } else if *difference_limbs.last().unwrap() == 0 {
                    if difference_limbs.len() == 2 {
                        Some(Small(difference_limbs[0]))
                    } else {
                        difference_limbs.pop();
                        Some(Large(difference_limbs))
                    }
                } else {
                    Some(Large(difference_limbs))
                }
            }
        }
    }
}

/// Subtracts a `Natural` from a `u32`, taking the `Natural` by reference. If the `Natural` is
/// greater than the `u32`, returns `None`.
///
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!(format!("{:?}", 123 - &Natural::from(123u32)), "Some(0)");
/// assert_eq!(format!("{:?}", 123 - &Natural::from(0u32)), "Some(123)");
/// assert_eq!(format!("{:?}", 456 - &Natural::from(123u32)), "Some(333)");
/// assert_eq!(format!("{:?}", 123 - &Natural::from(456u32)), "None");
/// assert_eq!(format!("{:?}", 123 - &Natural::from_str("1000000000000").unwrap()), "None");
/// ```
impl<'a> Sub<&'a Natural> for u32 {
    type Output = Option<Natural>;

    fn sub(self, other: &'a Natural) -> Option<Natural> {
        other.to_u32().and_then(|x| self.checked_sub(x)).map(
            Natural::from,
        )
    }
}

/// Subtracts a `u32` from a `Natural` in place.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
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
                if large_sub_u32(&mut limbs[..], other) {
                    panic = true;
                }
            }
        }
        if panic {
            panic!(
                "Cannot subtract a u32 from a smaller Natural. self: {}, other: {}",
                *self,
                other
            );
        }
        self.trim();
    }
}

pub(crate) fn large_sub_u32(limbs: &mut [u32], mut subtrahend: u32) -> bool {
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
    subtrahend != 0
}

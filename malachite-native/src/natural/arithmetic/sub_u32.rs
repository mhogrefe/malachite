use natural::Natural::{self, Large, Small};
use std::ops::{Sub, SubAssign};

// Subtract s2limb from s1, and write the s1.len() least significant limbs of the result to r.
// Return borrow. r must have size at least s1.len().
pub fn mpn_sub_1(r: &mut [u32], s1: &[u32], mut s2limb: u32) -> bool {
    for i in 0..s1.len() {
        let (difference, overflow) = s1[i].overflowing_sub(s2limb);
        r[i] = difference;
        if overflow {
            s2limb = 1;
        } else {
            s2limb = 0;
            let copy_index = i + 1;
            &r[copy_index..].copy_from_slice(&s1[copy_index..]);
            break;
        }
    }
    s2limb != 0
}

// Subtract s2limb from s1, and write the s1.len() least significant limbs of the result to s1.
// Return borrow.
pub fn mpn_sub_1_in_place(s1: &mut [u32], mut s2limb: u32) -> bool {
    for limb in s1.iter_mut() {
        let (difference, overflow) = limb.overflowing_sub(s2limb);
        *limb = difference;
        if overflow {
            s2limb = 1;
        } else {
            return false;
        }
    }
    true
}

// x -= y, return borrow
pub(crate) fn sub_assign_u32_helper(x: &mut Natural, y: u32) -> bool {
    if y == 0 {
        return false;
    }
    match *x {
        Small(ref mut small) => {
            return match small.checked_sub(y) {
                Some(difference) => {
                    *small = difference;
                    false
                }
                None => true,
            }
        }
        Large(ref mut limbs) => {
            if mpn_sub_1_in_place(&mut limbs[..], y) {
                return true;
            }
        }
    }
    x.trim();
    false
}

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
        if sub_assign_u32_helper(&mut self, other) {
            None
        } else {
            Some(self)
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
                let mut difference_limbs = vec![0; limbs.len()];
                if mpn_sub_1(&mut difference_limbs, limbs, other) {
                    None
                } else {
                    let mut difference = Large(difference_limbs);
                    difference.trim();
                    Some(difference)
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
        if sub_assign_u32_helper(self, other) {
            panic!(
                "Cannot subtract a u32 from a smaller Natural. self: {}, other: {}",
                *self,
                other
            );
        }
    }
}

use integer::Integer;
use std::cmp::Ordering;

/// Compares `self` to an `i32`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
///
/// assert!(Integer::from(-123) < -122);
/// assert!(Integer::from(-123) <= -122);
/// assert!(Integer::from(-123) > -124);
/// assert!(Integer::from(-123) >= -124);
/// assert!(Integer::trillion() > 123);
/// assert!(Integer::trillion() >= 123);
/// assert!(-Integer::trillion() < 123);
/// assert!(-Integer::trillion() <= 123);
/// ```
impl PartialOrd<i32> for Integer {
    fn partial_cmp(&self, other: &i32) -> Option<Ordering> {
        if self.sign {
            if *other >= 0 {
                self.abs.partial_cmp(&(*other as u32))
            } else {
                Some(Ordering::Greater)
            }
        } else if *other >= 0 {
            Some(Ordering::Less)
        } else {
            (other.wrapping_abs() as u32).partial_cmp(&self.abs)
        }
    }
}

/// Compares an `i32` to `self`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Examples
/// ```
/// use malachite_native::integer::Integer;
///
/// assert!(-122 > Integer::from(-123));
/// assert!(-122 >= Integer::from(-123));
/// assert!(-124 < Integer::from(-123));
/// assert!(-124 <= Integer::from(-123));
/// assert!(123 < Integer::trillion());
/// assert!(123 <= Integer::trillion());
/// assert!(123 > -Integer::trillion());
/// assert!(123 >= -Integer::trillion());
/// ```
impl PartialOrd<Integer> for i32 {
    fn partial_cmp(&self, other: &Integer) -> Option<Ordering> {
        if other.sign {
            if *self >= 0 {
                (*self as u32).partial_cmp(&other.abs)
            } else {
                Some(Ordering::Less)
            }
        } else if *self >= 0 {
            Some(Ordering::Greater)
        } else {
            other.abs.partial_cmp(&(self.wrapping_abs() as u32))
        }
    }
}

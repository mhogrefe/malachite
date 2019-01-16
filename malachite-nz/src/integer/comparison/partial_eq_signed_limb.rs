use integer::Integer;
use malachite_base::num::UnsignedAbs;
use platform::SignedLimb;

/// Determines whether an `Integer` is equal to a `SignedLimb`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert!(Integer::from(123) == 123);
/// assert!(Integer::from(123) != -5);
/// ```
impl PartialEq<SignedLimb> for Integer {
    fn eq(&self, other: &SignedLimb) -> bool {
        self.sign == (*other >= 0) && self.abs == other.unsigned_abs()
    }
}

/// Determines whether a `SignedLimb` is equal to an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::integer::Integer;
///
/// assert!(123 == Integer::from(123));
/// assert!(-5 != Integer::from(123));
/// ```
impl PartialEq<Integer> for SignedLimb {
    fn eq(&self, other: &Integer) -> bool {
        other.sign == (*self >= 0) && other.abs == self.unsigned_abs()
    }
}

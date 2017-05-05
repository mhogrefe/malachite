use integer::Integer;
use natural::Natural;
use std::cmp::Ordering;
use traits::Assign;

/// Assigns a `&Integer` to `self`.
///
/// # Panics
/// Panics if `other` is negative.
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::Assign;
///
/// let mut x = Natural::from(456);
/// x.assign(&Integer::from(123));
/// assert_eq!(x.to_string(), "123");
/// ```
impl<'a> Assign<&'a Integer> for Natural {
    fn assign(&mut self, other: &'a Integer) {
        assert_ne!(other.sign(),
                   Ordering::Less,
                   "Cannot assign from a negative Integer. Invalid other: {}",
                   other);
        self.assign(&other.clone().unsigned_abs());
    }
}

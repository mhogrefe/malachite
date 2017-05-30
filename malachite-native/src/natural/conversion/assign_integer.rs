use integer::Integer;
use natural::Natural;
use std::cmp::Ordering;
use traits::Assign;

/// Assigns an `Integer` to `self`. This implementation takes `other` by value.
///
/// Time: worst case O(1)
/// Additional memory: worst case O(1)
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
/// let mut x = Natural::from(456u32);
/// x.assign(Integer::from(123));
/// assert_eq!(x.to_string(), "123");
/// ```
impl Assign<Integer> for Natural {
    fn assign(&mut self, other: Integer) {
        assert_ne!(other.sign(),
                   Ordering::Less,
                   "Cannot assign from a negative Integer. Invalid other: {}",
                   other);
        *self = other.unsigned_abs();
    }
}

/// Assigns an `Integer` to `self`. This implementation takes `other` by reference.
///
/// Time: worst case O(n)
/// Additional memory: worst case O(n)
///
/// where n = `*other.significant_bits()`
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
/// let mut x = Natural::from(456u32);
/// x.assign(&Integer::from(123));
/// assert_eq!(x.to_string(), "123");
/// ```
impl<'a> Assign<&'a Integer> for Natural {
    fn assign(&mut self, other: &'a Integer) {
        assert_ne!(other.sign(),
                   Ordering::Less,
                   "Cannot assign from a negative Integer. Invalid other: {}",
                   other);
        self.clone_from(&other.unsigned_abs_by_ref());
    }
}

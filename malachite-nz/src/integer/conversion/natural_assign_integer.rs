use integer::Integer;
use malachite_base::num::{Assign, Sign};
use natural::Natural;
use std::cmp::Ordering;

/// Assigns an `Integer` to a `Natural`, taking the `Integer` by value.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `other` is negative.
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Assign;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(456u32);
///     x.assign(Integer::from(123));
///     assert_eq!(x.to_string(), "123");
/// }
/// ```
impl Assign<Integer> for Natural {
    fn assign(&mut self, other: Integer) {
        assert_ne!(
            other.sign(),
            Ordering::Less,
            "Cannot assign from a negative Integer. Invalid other: {}",
            other
        );
        *self = other.abs;
    }
}

/// Assigns an `Integer` to a `Natural`, taking the `Integer` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits()`
///
/// # Panics
/// Panics if `other` is negative.
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Assign;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(456u32);
///     x.assign(&Integer::from(123));
///     assert_eq!(x.to_string(), "123");
/// }
/// ```
impl<'a> Assign<&'a Integer> for Natural {
    fn assign(&mut self, other: &'a Integer) {
        assert_ne!(
            other.sign(),
            Ordering::Less,
            "Cannot assign from a negative Integer. Invalid other: {}",
            other
        );
        self.clone_from(&other.abs);
    }
}

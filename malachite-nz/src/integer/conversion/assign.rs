use integer::Integer;
use malachite_base::num::Assign;

/// Assigns an `Integer` to another `Integer`, taking the `Integer` on the RHS by value.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Assign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(45);
///     x.assign(Integer::from(-123));
///     assert_eq!(x.to_string(), "-123");
/// }
/// ```
impl Assign<Integer> for Integer {
    fn assign(&mut self, other: Integer) {
        *self = other;
    }
}

/// Assigns an `Integer` to another `Integer`, taking the `Integer` on the RHS by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits()`
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Assign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(456);
///     x.assign(&Integer::from(-123));
///     assert_eq!(x.to_string(), "-123");
/// }
/// ```
impl<'a> Assign<&'a Integer> for Integer {
    fn assign(&mut self, other: &'a Integer) {
        self.clone_from(other);
    }
}

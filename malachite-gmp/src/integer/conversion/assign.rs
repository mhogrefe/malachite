use integer::Integer;
use malachite_base::traits::Assign;

/// Assigns a `Integer` to another `Integer`, taking the `Integer` on the RHS by value.
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Assign;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(456);
///     x.assign(Integer::from(-123));
///     assert_eq!(x.to_string(), "-123");
/// }
/// ```
impl Assign<Integer> for Integer {
    fn assign(&mut self, other: Integer) {
        *self = other;
    }
}

/// Assigns a `Integer` to another `Integer`, taking the `Integer` on the RHS by reference.
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Assign;
/// use malachite_gmp::integer::Integer;
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

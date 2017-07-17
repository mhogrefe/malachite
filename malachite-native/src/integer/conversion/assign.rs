use integer::Integer;
use traits::Assign;

/// Assigns an `Integer` to another `Integer`, taking the `Integer` on the RHS by value.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::traits::Assign;
///
/// let mut x = Integer::from(45);
/// x.assign(Integer::from(-123));
/// assert_eq!(x.to_string(), "-123");
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
/// use malachite_native::integer::Integer;
/// use malachite_native::traits::Assign;
///
/// let mut x = Integer::from(456);
/// x.assign(&Integer::from(-123));
/// assert_eq!(x.to_string(), "-123");
/// ```
impl<'a> Assign<&'a Integer> for Integer {
    fn assign(&mut self, other: &'a Integer) {
        self.clone_from(other);
    }
}

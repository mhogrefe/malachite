use integer::Integer;
use traits::Assign;

/// Assigns an `Integer` to `self`.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::Assign;
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

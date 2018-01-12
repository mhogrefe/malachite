use integer::Integer;

/// Determines whether an `Integer` is equal to a `u32`.
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
/// assert!(Integer::from(123) != 5);
/// ```
impl PartialEq<u32> for Integer {
    fn eq(&self, other: &u32) -> bool {
        self.sign && self.abs == *other
    }
}

/// Determines whether a `u32` is equal to an `Integer`.
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
/// assert!(5 != Integer::from(123));
/// ```
impl PartialEq<Integer> for u32 {
    fn eq(&self, other: &Integer) -> bool {
        other.sign && other.abs == *self
    }
}

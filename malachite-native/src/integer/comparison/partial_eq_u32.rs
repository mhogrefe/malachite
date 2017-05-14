use integer::Integer;

/// Determines whether `self` is equal to a `u32`.
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
///
/// assert!(Integer::from(123) == 123);
/// assert!(Integer::from(123) != 5);
/// ```
impl PartialEq<u32> for Integer {
    fn eq(&self, other: &u32) -> bool {
        self.sign && self.abs == *other
    }
}

/// Determines whether a `u32` is equal to `self`.
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
///
/// assert!(123 == Integer::from(123));
/// assert!(5 != Integer::from(123));
/// ```
impl PartialEq<Integer> for u32 {
    fn eq(&self, other: &Integer) -> bool {
        other.sign && other.abs == *self
    }
}

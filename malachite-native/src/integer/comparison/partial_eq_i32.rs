use integer::Integer;

/// Determines whether an `Integer` is equal to an `i32`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
///
/// assert!(Integer::from(123) == 123);
/// assert!(Integer::from(123) != -5);
/// ```
impl PartialEq<i32> for Integer {
    fn eq(&self, other: &i32) -> bool {
        self.sign == (*other >= 0) && self.abs == (other.abs() as u32)
    }
}

/// Determines whether an `i32` is equal to an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
///
/// assert!(123 == Integer::from(123));
/// assert!(-5 != Integer::from(123));
/// ```
impl PartialEq<Integer> for i32 {
    fn eq(&self, other: &Integer) -> bool {
        other.sign == (*self >= 0) && other.abs == (self.abs() as u32)
    }
}

use integer::Integer;

/// Determines whether `self` is equal to an `i32`.
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
///
/// assert!(Integer::from(123) == 123);
/// assert!(Integer::from(123) != -5);
/// ```
impl PartialEq<i32> for Integer {
    fn eq(&self, i: &i32) -> bool {
        self.sign == (*i >= 0) && self.abs == (i.abs() as u32)
    }
}

/// Determines whether an `i32` is equal to `self`.
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
///
/// assert!(123 == Integer::from(123));
/// assert!(-5 != Integer::from(123));
/// ```
impl PartialEq<Integer> for i32 {
    fn eq(&self, i: &Integer) -> bool {
        i.sign == (*self >= 0) && i.abs == (self.abs() as u32)
    }
}

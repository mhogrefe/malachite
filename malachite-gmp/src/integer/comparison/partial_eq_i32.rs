use integer::Integer::{self, Large, Small};

/// Determines whether `self` is equal to an `i32`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert!(Integer::from(123) == 123);
/// assert!(Integer::from(123) != -5);
/// ```
impl PartialEq<i32> for Integer {
    fn eq(&self, i: &i32) -> bool {
        match *self {
            Small(x) => x == *i,
            Large(_) => false,
        }
    }
}

/// Determines whether an `i32` is equal to `self`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert!(123 == Integer::from(123));
/// assert!(-5 != Integer::from(123));
/// ```
impl PartialEq<Integer> for i32 {
    fn eq(&self, i: &Integer) -> bool {
        match *i {
            Small(y) => y == *self,
            Large(_) => false,
        }
    }
}

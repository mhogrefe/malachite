use natural::Natural::{self, Large, Small};

/// Determines whether `self` is equal to a `u32`.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
///
/// assert!(Natural::from(123) == 123);
/// assert!(Natural::from(123) != 5);
/// ```
impl PartialEq<u32> for Natural {
    fn eq(&self, other: &u32) -> bool {
        match *self {
            Small(x) => x == *other,
            Large(_) => false,
        }
    }
}

/// Determines whether a `u32` is equal to `self`.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
///
/// assert!(123 == Natural::from(123));
/// assert!(5 != Natural::from(123));
/// ```
impl PartialEq<Natural> for u32 {
    fn eq(&self, other: &Natural) -> bool {
        match *other {
            Small(y) => y == *self,
            Large(_) => false,
        }
    }
}

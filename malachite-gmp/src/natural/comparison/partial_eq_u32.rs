use natural::Natural::{self, Large, Small};

/// Determines whether a `Natural` is equal to a `u32`.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
///
/// assert!(Natural::from(123u32) == 123);
/// assert!(Natural::from(123u32) != 5);
/// ```
impl PartialEq<u32> for Natural {
    fn eq(&self, other: &u32) -> bool {
        match *self {
            Small(x) => x == *other,
            Large(_) => false,
        }
    }
}

/// Determines whether a `Natural` is equal to `self`.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
///
/// assert!(123 == Natural::from(123u32));
/// assert!(5 != Natural::from(123u32));
/// ```
impl PartialEq<Natural> for u32 {
    fn eq(&self, other: &Natural) -> bool {
        match *other {
            Small(y) => y == *self,
            Large(_) => false,
        }
    }
}

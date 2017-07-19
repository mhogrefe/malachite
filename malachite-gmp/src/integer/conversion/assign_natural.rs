use integer::Integer;
use natural::Natural;
use traits::Assign;

/// Assigns an `Integer` to a `Natural`, taking the `Integer` by value.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::Assign;
///
/// let mut x = Integer::from(-456);
/// x.assign(&Natural::from(123u32));
/// assert_eq!(x.to_string(), "123");
/// ```
impl Assign<Natural> for Integer {
    fn assign(&mut self, other: Natural) {
        *self = other.into_integer();
    }
}

/// Assigns an `Integer` to a `Natural`, taking the `Integer` by reference.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::Assign;
///
/// let mut x = Integer::from(-456);
/// x.assign(&Natural::from(123u32));
/// assert_eq!(x.to_string(), "123");
/// ```
impl<'a> Assign<&'a Natural> for Integer {
    fn assign(&mut self, other: &'a Natural) {
        *self = other.to_integer();
    }
}

use integer::Integer;
use natural::Natural;
use traits::Assign;

/// Assigns a `Natural` to `self`.
///
/// # Panics
/// Panics if `other` is negative.
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::Assign;
///
/// let mut x = Integer::from(-456);
/// x.assign(&Natural::from(123u32));
/// assert_eq!(x.to_string(), "123");
/// ```
impl<'a> Assign<&'a Natural> for Integer {
    fn assign(&mut self, other: &'a Natural) {
        self.sign = true;
        self.abs.assign(other);
    }
}

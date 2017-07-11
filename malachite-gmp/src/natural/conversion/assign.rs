use natural::Natural;
use traits::Assign;

/// Assigns a `Natural` to another `Natural`, taking the `Natural` on the RHS by value.
///
/// # Example
/// ```
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::Assign;
///
/// let mut x = Natural::from(456u32);
/// x.assign(Natural::from(123u32));
/// assert_eq!(x.to_string(), "123");
/// ```
impl Assign<Natural> for Natural {
    fn assign(&mut self, other: Natural) {
        *self = other;
    }
}

/// Assigns a `Natural` to another `Natural`, taking the `Natural` on the RHS by reference.
///
/// # Example
/// ```
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::Assign;
///
/// let mut x = Natural::from(456u32);
/// x.assign(&Natural::from(123u32));
/// assert_eq!(x.to_string(), "123");
/// ```
impl<'a> Assign<&'a Natural> for Natural {
    fn assign(&mut self, other: &'a Natural) {
        self.clone_from(other);
    }
}

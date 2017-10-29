use natural::Natural;
use malachite_base::traits::Assign;

/// Assigns a `Natural` to another `Natural`, taking the `Natural` on the RHS by value.
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Assign;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(456u32);
///     x.assign(Natural::from(123u32));
///     assert_eq!(x.to_string(), "123");
/// }
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
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Assign;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(456u32);
///     x.assign(&Natural::from(123u32));
///     assert_eq!(x.to_string(), "123");
/// }
/// ```
impl<'a> Assign<&'a Natural> for Natural {
    fn assign(&mut self, other: &'a Natural) {
        self.clone_from(other);
    }
}

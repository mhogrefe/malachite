use natural::Natural;
use traits::Assign;

/// Assigns a `Natural` to another `Natural`, taking the `Natural` on the RHS by value.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::Assign;
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
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits()`
///
/// # Example
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::Assign;
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

use malachite_base::num::Assign;
use natural::Natural;

/// Assigns a `Natural` to another `Natural`, taking the `Natural` on the RHS by value.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Assign;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(456u32);
///     x.assign(Natural::from(123u32));
///     assert_eq!(x.to_string(), "123");
/// }
/// ```
impl Assign<Natural> for Natural {
    #[inline]
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
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Assign;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(456u32);
///     x.assign(&Natural::from(123u32));
///     assert_eq!(x.to_string(), "123");
/// }
/// ```
impl<'a> Assign<&'a Natural> for Natural {
    #[inline]
    fn assign(&mut self, other: &'a Natural) {
        self.clone_from(other);
    }
}

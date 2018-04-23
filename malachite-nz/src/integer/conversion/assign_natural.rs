use integer::Integer;
use malachite_base::num::Assign;
use natural::Natural;

/// Assigns a `Natural` to an `Integer`, taking the `Natural` by value.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `other` is negative.
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Assign;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Integer::from(-456);
///     x.assign(Natural::from(123u32));
///     assert_eq!(x.to_string(), "123");
/// }
/// ```
impl Assign<Natural> for Integer {
    fn assign(&mut self, other: Natural) {
        self.sign = true;
        self.abs.assign(other);
    }
}

/// Assigns a `Natural` to an `Integer`, taking the `Natural` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `other.significant_bits()`
///
/// # Panics
/// Panics if `other` is negative.
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::Assign;
/// use malachite_nz::integer::Integer;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Integer::from(-456);
///     x.assign(&Natural::from(123u32));
///     assert_eq!(x.to_string(), "123");
/// }
/// ```
impl<'a> Assign<&'a Natural> for Integer {
    fn assign(&mut self, other: &'a Natural) {
        self.sign = true;
        self.abs.assign(other);
    }
}

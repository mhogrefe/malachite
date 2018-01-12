use malachite_base::traits::Assign;
use natural::Natural;

/// Assigns a `u64` to a `Natural`.
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
/// use malachite_base::traits::Assign;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(123u32);
///     x.assign(1000000000000u64);
///     assert_eq!(x.to_string(), "1000000000000");
/// }
/// ```
impl Assign<u64> for Natural {
    fn assign(&mut self, other: u64) {
        *self = Natural::from(other);
    }
}

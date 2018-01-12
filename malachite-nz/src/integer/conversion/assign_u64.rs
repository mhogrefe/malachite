use integer::Integer;
use malachite_base::traits::Assign;

/// Assigns a `u64` to to an `Integer`.
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
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(-123);
///     x.assign(456u64);
///     assert_eq!(x.to_string(), "456");
/// }
/// ```
impl Assign<u64> for Integer {
    fn assign(&mut self, other: u64) {
        self.sign = true;
        self.abs.assign(other);
    }
}

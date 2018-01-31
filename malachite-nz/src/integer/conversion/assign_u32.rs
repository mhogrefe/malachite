use integer::Integer;
use malachite_base::num::Assign;

/// Assigns a `u32` to to an `Integer`.
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
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(-123);
///     x.assign(456);
///     assert_eq!(x.to_string(), "456");
/// }
/// ```
impl Assign<u32> for Integer {
    fn assign(&mut self, other: u32) {
        self.sign = true;
        self.abs.assign(other);
    }
}

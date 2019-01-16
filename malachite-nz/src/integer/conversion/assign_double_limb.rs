use integer::Integer;
use malachite_base::num::Assign;
use platform::DoubleLimb;

/// Assigns a `DoubleLimb` to to an `Integer`.
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
///     x.assign(456u64);
///     assert_eq!(x.to_string(), "456");
/// }
/// ```
impl Assign<DoubleLimb> for Integer {
    fn assign(&mut self, other: DoubleLimb) {
        self.sign = true;
        self.abs.assign(other);
    }
}

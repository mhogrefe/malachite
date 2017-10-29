use integer::Integer;
use malachite_base::traits::Assign;

/// Assigns an `i64` to an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Assign;
/// use malachite_native::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(456);
///     x.assign(-123i64);
///     assert_eq!(x.to_string(), "-123");
/// }
/// ```
impl Assign<i64> for Integer {
    fn assign(&mut self, other: i64) {
        self.sign = other >= 0;
        self.abs.assign(other.wrapping_abs() as u64);
    }
}

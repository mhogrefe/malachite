use integer::Integer;
use traits::Assign;

/// Assigns a `u64` to to an `Integer`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::traits::Assign;
///
/// let mut x = Integer::from(-123);
/// x.assign(456u64);
/// assert_eq!(x.to_string(), "456");
/// ```
impl Assign<u64> for Integer {
    fn assign(&mut self, other: u64) {
        self.sign = true;
        self.abs.assign(other);
    }
}

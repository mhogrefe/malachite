use integer::Integer;
use natural::Natural;

/// Determines whether `self` is equal to an `Integer`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `other.significant_bits()`
///
/// # Example
/// ```
/// use malachite_native::integer::Integer;
/// use malachite_native::natural::Natural;
///
/// assert!(Natural::from(123u32) == Integer::from(123));
/// assert!(Natural::from(123u32) != Integer::from(5));
/// ```
impl PartialEq<Integer> for Natural {
    fn eq(&self, other: &Integer) -> bool {
        other == self
    }
}

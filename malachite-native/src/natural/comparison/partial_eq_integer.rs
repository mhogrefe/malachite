use integer::Integer;
use natural::Natural;

/// Determines whether a `Natural` is equal to an `Integer`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = min(`self.significant_bits(), other.significant_bits()`)
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
        match *other {
            Integer { sign: true, ref abs } if self == abs => true,
            _ => false,
        }
    }
}

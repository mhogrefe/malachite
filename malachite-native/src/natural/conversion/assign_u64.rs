use natural::Natural;
use traits::Assign;

/// Assigns a `u64` to `self`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::Assign;
///
/// let mut x = Natural::from(123u32);
/// x.assign(1000000000000u64);
/// assert_eq!(x.to_string(), "1000000000000");
/// ```
impl Assign<u64> for Natural {
    fn assign(&mut self, other: u64) {
        *self = Natural::from(other);
    }
}

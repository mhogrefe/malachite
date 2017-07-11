use natural::Natural::{self, Small};
use traits::Assign;

/// Assigns a `u32` to a `Natural`.
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
/// x.assign(456u32);
/// assert_eq!(x.to_string(), "456");
/// ```
impl Assign<u32> for Natural {
    fn assign(&mut self, other: u32) {
        *self = Small(other);
    }
}

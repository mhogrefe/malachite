use natural::Natural::{self, Small};
use traits::Assign;

/// Assigns a `u32` to a `Natural`.
///
/// # Example
/// ```
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::Assign;
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

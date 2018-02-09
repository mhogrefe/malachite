use malachite_base::num::SplitInHalf;
use natural::Natural::{self, Large, Small};
use std::u32;

/// Converts a `u64` to a `Natural`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::natural::Natural;
///
/// assert_eq!(Natural::from(1000000000000u64).to_string(), "1000000000000");
/// ```
impl From<u64> for Natural {
    fn from(u: u64) -> Natural {
        if u <= u32::MAX.into() {
            Small(u as u32)
        } else {
            Large(vec![u.lower_half(), u.upper_half()])
        }
    }
}

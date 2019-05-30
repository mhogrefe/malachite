use malachite_base::num::conversion::traits::SplitInHalf;

use natural::Natural::{self, Large, Small};
use platform::DoubleLimb;

/// Converts a `DoubleLimb` to a `Natural`.
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
impl From<DoubleLimb> for Natural {
    fn from(u: DoubleLimb) -> Natural {
        let (upper, lower) = u.split_in_half();
        if upper == 0 {
            Small(lower)
        } else {
            Large(vec![lower, upper])
        }
    }
}

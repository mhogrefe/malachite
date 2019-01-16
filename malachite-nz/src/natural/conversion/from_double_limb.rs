use malachite_base::num::SplitInHalf;
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
        let (hi, lo) = u.split_in_half();
        if hi == 0 {
            Small(lo)
        } else {
            Large(vec![lo, hi])
        }
    }
}

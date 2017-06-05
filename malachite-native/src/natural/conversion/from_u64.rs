use natural::{get_lower, get_upper};
use natural::Natural::{self, Large, Small};

/// Converts a `u64` to a `Natural`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_native::natural::Natural;
///
/// assert_eq!(Natural::from(1000000000000u64).to_string(), "1000000000000");
/// ```
impl From<u64> for Natural {
    fn from(u: u64) -> Natural {
        if u < u32::max_value() as u64 {
            Small(u as u32)
        } else {
            Large(vec![get_lower(u), get_upper(u)])
        }
    }
}

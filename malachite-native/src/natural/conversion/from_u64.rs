use natural::{get_lower, get_upper, Natural};

/// Converts a `u64` to a `Natural`.
///
/// Time: worst case O(1)
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
        let mut n = Natural::new();
        n.assign_limbs_le(&[get_lower(u), get_upper(u)]);
        n
    }
}

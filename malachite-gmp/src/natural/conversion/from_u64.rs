use natural::{get_lower, get_upper, Natural};

/// Converts a `u64` to a `Natural`.
///
/// # Example
/// ```
/// use malachite_gmp::natural::Natural;
///
/// assert_eq!(Natural::from(1000000000000u64).to_string(), "1000000000000");
/// ```
impl From<u64> for Natural {
    fn from(u: u64) -> Natural {
        Natural::from_limbs_le(&[get_lower(u), get_upper(u)])
    }
}

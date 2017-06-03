use natural::{get_lower, get_upper, Natural};
use traits::Assign;

/// Assigns a `u64` to `self`.
///
/// # Example
/// ```
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::Assign;
///
/// let mut x = Natural::from(123u32);
/// x.assign(1000000000000u64);
/// assert_eq!(x.to_string(), "1000000000000");
/// ```
impl Assign<u64> for Natural {
    fn assign(&mut self, other: u64) {
        self.assign_limbs_le(&[get_lower(other), get_upper(other)]);
    }
}

use natural::Natural;
use malachite_base::num::{get_lower, get_upper};
use malachite_base::traits::Assign;

/// Assigns a `u64` to a `Natural`.
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Assign;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(123u32);
///     x.assign(1000000000000u64);
///     assert_eq!(x.to_string(), "1000000000000");
/// }
/// ```
impl Assign<u64> for Natural {
    fn assign(&mut self, other: u64) {
        *self = Natural::from_limbs_le(&[get_lower(other), get_upper(other)]);
    }
}

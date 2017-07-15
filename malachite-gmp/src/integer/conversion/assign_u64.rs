use gmp_mpfr_sys::gmp;
use integer::{get_lower, get_upper};
use integer::Integer;
use traits::Assign;

/// Assigns a `u64` to an `Integer`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::Assign;
///
/// let mut x = Integer::from(-123);
/// x.assign(456u64);
/// assert_eq!(x.to_string(), "456");
/// ```
impl Assign<u64> for Integer {
    fn assign(&mut self, other: u64) {
        let (lower, upper) = (get_lower(other), get_upper(other));
        if upper == 0 {
            self.assign(other as u32)
        } else {
            let mut large = self.promote_in_place();
            unsafe {
                gmp::mpz_set_ui(large, upper.into());
                gmp::mpz_mul_2exp(large, large, 32);
                gmp::mpz_add_ui(large, large, lower.into());
            }
        }
    }
}

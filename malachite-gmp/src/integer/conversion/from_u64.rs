use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::{Integer, get_lower, get_upper, Large};
use std::mem;

/// Converts a `u64` to an `Integer`.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert_eq!(Integer::from(123u64).to_string(), "123");
/// ```
impl From<u64> for Integer {
    fn from(u: u64) -> Integer {
        let (lower, upper) = (get_lower(u), get_upper(u));
        if upper == 0 {
            Integer::from(u as u32)
        } else {
            unsafe {
                let mut large: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut large, upper.into());
                gmp::mpz_mul_2exp(&mut large, &large, 32);
                gmp::mpz_add_ui(&mut large, &large, lower.into());
                Large(large)
            }
        }
    }
}

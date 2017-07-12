use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::{Integer, Large, Small};
use std::mem;

/// Converts a `u32` to an `Integer`.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert_eq!(Integer::from(123).to_string(), "123");
/// ```
impl From<u32> for Integer {
    fn from(u: u32) -> Integer {
        if u & 0x8000_0000 == 0 {
            Small(u as i32)
        } else {
            unsafe {
                let mut large: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut large, u.into());
                Large(large)
            }
        }
    }
}

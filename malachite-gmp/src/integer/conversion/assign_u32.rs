use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Small};
use std::mem;
use traits::Assign;

/// Assigns a `u32` to `self`.
///
/// # Example
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::Assign;
///
/// let mut x = Integer::from(-123);
/// x.assign(456);
/// assert_eq!(x.to_string(), "456");
/// ```
impl Assign<u32> for Integer {
    fn assign(&mut self, u: u32) {
        if u & 0x8000_0000 == 0 {
            *self = Small(u as i32);
        } else {
            unsafe {
                let mut assigned: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut assigned, u.into());
                self.assign_mpz_t(assigned);
            }
        }
    }
}

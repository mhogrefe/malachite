use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Small};
use std::mem;
use malachite_base::traits::Assign;

/// Assigns a `u32` to an `Integer`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Assign;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(-123);
///     x.assign(456);
///     assert_eq!(x.to_string(), "456");
/// }
/// ```
impl Assign<u32> for Integer {
    fn assign(&mut self, other: u32) {
        if other & 0x8000_0000 == 0 {
            *self = Small(other as i32);
        } else {
            unsafe {
                let mut large: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut large, other.into());
                self.assign_mpz_t(large);
            }
        }
    }
}

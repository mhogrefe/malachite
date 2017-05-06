use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::mem;
use std::ops::Neg;

/// Takes the negative of `self`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use std::ops::Neg;
/// use std::str::FromStr;
///
/// assert_eq!((-Integer::from(0)).to_string(), "0");
/// assert_eq!((-Integer::from(123)).to_string(), "-123");
/// assert_eq!((-Integer::from(-123)).to_string(), "123");
/// assert_eq!((-Integer::from_str("1000000000000").unwrap()).to_string(), "-1000000000000");
/// assert_eq!((-Integer::from_str("-1000000000000").unwrap()).to_string(), "1000000000000");
/// ```
impl Neg for Integer {
    type Output = Integer;

    fn neg(mut self) -> Integer {
        match self {
            Small(x) if x == i32::min_value() => unsafe {
                let mut x: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut x, 1 << 31);
                return Large(x);
            },
            Small(x) => return Small(-x),
            Large(ref mut x) => unsafe {
                gmp::mpz_neg(x, x);
            },
        };
        self
    }
}

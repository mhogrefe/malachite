use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::mem;
use std::ops::Neg;
use traits::NegAssign;

/// Returns the negative of `self`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert_eq!((-Integer::from(0)).to_string(), "0");
/// assert_eq!((-Integer::from(123)).to_string(), "-123");
/// assert_eq!((-Integer::from(-123)).to_string(), "123");
/// ```
impl Neg for Integer {
    type Output = Integer;

    fn neg(mut self) -> Integer {
        self.neg_assign();
        self
    }
}

/// Negates `self`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::NegAssign;
///
/// let mut x = Integer::from(0);
/// x.neg_assign();
/// assert_eq!(x.to_string(), "0");
///
/// let mut x = Integer::from(123);
/// x.neg_assign();
/// assert_eq!(x.to_string(), "-123");
///
/// let mut x = Integer::from(-123);
/// x.neg_assign();
/// assert_eq!(x.to_string(), "123");
/// ```
impl NegAssign for Integer {
    fn neg_assign(&mut self) {
        match *self {
            Small(x) if x == i32::min_value() => unsafe {
                let mut x: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut x, 1 << 31);
                *self = Large(x);
            },
            Small(ref mut x) => *x = -*x,
            Large(ref mut x) => unsafe {
                gmp::mpz_neg(x, x);
            },
        }
    }
}

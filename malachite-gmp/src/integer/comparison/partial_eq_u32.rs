use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};

/// Determines whether `self` is equal to a `u32`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert!(Integer::from(123) == 123);
/// assert!(Integer::from(123) != 5);
/// ```
impl PartialEq<u32> for Integer {
    fn eq(&self, u: &u32) -> bool {
        let u = *u;
        match *self {
            Small(x) => x >= 0 && x as u32 == u,
            Large(_) if u & 0x8000_0000 == 0 => false,
            Large(x) => (unsafe { gmp::mpz_cmp_ui(&x, u.into()) }) == 0,
        }
    }
}

/// Determines whether a `u32` is equal to `self`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert!(123 == Integer::from(123));
/// assert!(5 != Integer::from(123));
/// ```
impl PartialEq<Integer> for u32 {
    fn eq(&self, i: &Integer) -> bool {
        let x = *self;
        match *i {
            Small(y) => y >= 0 && y as u32 == x,
            Large(_) if x & 0x8000_0000 == 0 => false,
            Large(y) => (unsafe { gmp::mpz_cmp_ui(&y, x.into()) }) == 0,
        }
    }
}

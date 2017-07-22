use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};

/// Determines whether an `Integer` is equal to a `u32`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert!(Integer::from(123) == 123);
/// assert!(Integer::from(123) != 5);
/// ```
impl PartialEq<u32> for Integer {
    fn eq(&self, other: &u32) -> bool {
        let u = *other;
        match *self {
            Small(small) => small >= 0 && small as u32 == u,
            Large(_) if u <= i32::max_value() as u32 => false,
            Large(ref large) => (unsafe { gmp::mpz_cmp_ui(large, u.into()) }) == 0,
        }
    }
}

/// Determines whether a `u32` is equal to an `Integer`.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
///
/// assert!(123 == Integer::from(123));
/// assert!(5 != Integer::from(123));
/// ```
impl PartialEq<Integer> for u32 {
    fn eq(&self, other: &Integer) -> bool {
        let u = *self;
        match *other {
            Small(small) => small >= 0 && small as u32 == u,
            Large(_) if u <= i32::max_value() as u32 => false,
            Large(ref large) => (unsafe { gmp::mpz_cmp_ui(large, u.into()) }) == 0,
        }
    }
}

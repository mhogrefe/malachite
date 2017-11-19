use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer::{self, Large, Small};
use std::mem;
use std::ops::Neg;
use malachite_base::traits::NegAssign;

/// Returns the negative of an `Integer`, taking the `Integer` by value.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     assert_eq!((-Integer::zero()).to_string(), "0");
///     assert_eq!((-Integer::from(123)).to_string(), "-123");
///     assert_eq!((-Integer::from(-123)).to_string(), "123");
/// }
/// ```
impl Neg for Integer {
    type Output = Integer;

    fn neg(mut self) -> Integer {
        self.neg_assign();
        self
    }
}

/// Returns the negative of an `Integer`, taking the `Integer` by reference.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::Zero;
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     assert_eq!((-&Integer::zero()).to_string(), "0");
///     assert_eq!((-&Integer::from(123)).to_string(), "-123");
///     assert_eq!((-&Integer::from(-123)).to_string(), "123");
/// }
/// ```
impl<'a> Neg for &'a Integer {
    type Output = Integer;

    fn neg(self) -> Integer {
        match *self {
            Small(small) if small == i32::min_value() => unsafe {
                let mut negative: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut negative, 1 << 31);
                Large(negative)
            },
            Small(small) => Small(-small),
            Large(ref large) if unsafe { gmp::mpz_cmp_ui(large, 0x8000_0000) == 0 } => {
                Small(i32::min_value())
            }
            Large(ref large) => unsafe {
                let mut negative: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut negative);
                gmp::mpz_neg(&mut negative, large);
                Integer::Large(negative)
            },
        }
    }
}

/// Replaces an `Integer` with its negative.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::{NegAssign, Zero};
/// use malachite_gmp::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::zero();
///     x.neg_assign();
///     assert_eq!(x.to_string(), "0");
///
///     let mut x = Integer::from(123);
///     x.neg_assign();
///     assert_eq!(x.to_string(), "-123");
///
///     let mut x = Integer::from(-123);
///     x.neg_assign();
///     assert_eq!(x.to_string(), "123");
/// }
/// ```
impl NegAssign for Integer {
    fn neg_assign(&mut self) {
        let result_is_i32_min = match *self {
            Small(small) if small == i32::min_value() => unsafe {
                let mut x: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut x, 1 << 31);
                *self = Large(x);
                false
            },
            Small(ref mut small) => {
                *small = -*small;
                false
            }
            Large(ref mut large) if unsafe { gmp::mpz_cmp_ui(large, 0x8000_0000) == 0 } => true,
            Large(ref mut large) => unsafe {
                gmp::mpz_neg(large, large);
                false
            },
        };
        if result_is_i32_min {
            *self = Small(i32::min_value());
        }
    }
}

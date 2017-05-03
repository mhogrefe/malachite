use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer;
use natural::Natural;
use std::mem;

impl Integer {
    /// Takes the absolute value of `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// assert_eq!(Integer::from(0).abs().to_string(), "0");
    /// assert_eq!(Integer::from(123).abs().to_string(), "123");
    /// assert_eq!(Integer::from(-123).abs().to_string(), "123");
    /// ```
    pub fn abs(&mut self) -> &mut Integer {
        match self {
            &mut Integer::Small(x) if x == i32::min_value() => unsafe {
                let mut x: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut x, 1 << 31);
                *self = Integer::Large(x);
            },
            &mut Integer::Small(ref mut x) => *x = x.abs(),
            &mut Integer::Large(ref mut x) => unsafe {
                gmp::mpz_abs(x, x);
            },
        }
        self
    }

    /// Takes the absolute value of `self`, converting the result to a `Natural`.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// assert_eq!(Integer::from(0).unsigned_abs().to_string(), "0");
    /// assert_eq!(Integer::from(123).unsigned_abs().to_string(), "123");
    /// assert_eq!(Integer::from(-123).unsigned_abs().to_string(), "123");
    /// ```
    pub fn unsigned_abs(self) -> Natural {
        match self {
            Integer::Small(x) => Natural::Small(x.abs() as u32),
            Integer::Large(ref x) => unsafe {
                let mut abs: mpz_t = mem::uninitialized();
                gmp::mpz_init_set(&mut abs, x);
                gmp::mpz_abs(&mut abs, &abs);
                if gmp::mpz_sizeinbase(&abs, 2) <= 32 {
                    Natural::Small(gmp::mpz_get_ui(&abs) as u32)
                } else {
                    Natural::Large(abs)
                }
            },
        }
    }
}

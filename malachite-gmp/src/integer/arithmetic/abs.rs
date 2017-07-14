use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer;
use natural::Natural;
use std::mem;
use traits::AbsAssign;

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
    pub fn abs(mut self) -> Integer {
        self.abs_assign();
        self
    }

    pub fn abs_ref(&self) -> Integer {
        self.clone().abs()
    }

    pub fn unsigned_abs_ref(&self) -> Natural {
        self.clone().unsigned_abs()
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
            Integer::Small(small) => Natural::Small(small.abs() as u32),
            Integer::Large(ref large) => unsafe {
                let mut abs: mpz_t = mem::uninitialized();
                gmp::mpz_init_set(&mut abs, large);
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

/// Replaces `self` with its absolute value.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use malachite_gmp::traits::AbsAssign;
///
/// let mut x = Integer::from(0);
/// x.abs_assign();
/// assert_eq!(x.to_string(), "0");
///
/// let mut x = Integer::from(123);
/// x.abs_assign();
/// assert_eq!(x.to_string(), "123");
///
/// let mut x = Integer::from(-123);
/// x.abs_assign();
/// assert_eq!(x.to_string(), "123");
/// ```
impl AbsAssign for Integer {
    fn abs_assign(&mut self) {
        match *self {
            Integer::Small(small) if small == i32::min_value() => unsafe {
                let mut x: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut x, 1 << 31);
                *self = Integer::Large(x);
            },
            Integer::Small(ref mut small) => *small = small.abs(),
            Integer::Large(ref mut large) => unsafe {
                gmp::mpz_abs(large, large);
            },
        }
    }
}

use gmp_mpfr_sys::gmp::{self, mpz_t};
use integer::Integer;
use natural::Natural;
use std::mem;
use traits::AbsAssign;

impl Integer {
    /// Finds the absolute value of an `Integer`, taking the `Integer` by value.
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

    /// Finds the absolute value of an `Integer`, taking the `Integer` by reference.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// assert_eq!(Integer::from(0).abs_ref().to_string(), "0");
    /// assert_eq!(Integer::from(123).abs_ref().to_string(), "123");
    /// assert_eq!(Integer::from(-123).abs_ref().to_string(), "123");
    /// ```
    pub fn abs_ref(&self) -> Integer {
        match *self {
            Integer::Small(small) if small == i32::min_value() => unsafe {
                let mut x: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut x, 1 << 31);
                Integer::Large(x)
            },
            Integer::Small(ref small) => Integer::Small(small.abs()),
            Integer::Large(ref large) => unsafe {
                let mut abs: mpz_t = mem::uninitialized();
                gmp::mpz_init_set(&mut abs, large);
                gmp::mpz_abs(&mut abs, &abs);
                Integer::Large(abs)
            },
        }
    }

    /// Finds the absolute value of an `Integer`, taking the `Integer` by value and converting the
    /// result to a `Natural`.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// assert_eq!(Integer::from(0).natural_abs().to_string(), "0");
    /// assert_eq!(Integer::from(123).natural_abs().to_string(), "123");
    /// assert_eq!(Integer::from(-123).natural_abs().to_string(), "123");
    /// ```
    pub fn natural_abs(mut self) -> Natural {
        match self {
            Integer::Small(small) => Natural::Small(small.abs() as u32),
            Integer::Large(ref mut large) => unsafe {
                let mut abs: mpz_t = mem::uninitialized();
                gmp::mpz_init(&mut abs);
                mem::swap(&mut abs, large);
                gmp::mpz_abs(&mut abs, &abs);
                if gmp::mpz_sizeinbase(&abs, 2) <= 32 {
                    Natural::Small(gmp::mpz_get_ui(&abs) as u32)
                } else {
                    Natural::Large(abs)
                }
            },
        }
    }

    /// Finds the absolute value of an `Integer`, taking the `Integer` by reference and converting
    /// the result to a `Natural`.
    ///
    /// # Examples
    /// ```
    /// use malachite_gmp::integer::Integer;
    ///
    /// assert_eq!(Integer::from(0).natural_abs().to_string(), "0");
    /// assert_eq!(Integer::from(123).natural_abs().to_string(), "123");
    /// assert_eq!(Integer::from(-123).natural_abs().to_string(), "123");
    /// ```
    pub fn natural_abs_ref(&self) -> Natural {
        match *self {
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

/// Replaces an `Integer` with its absolute value.
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

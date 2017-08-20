use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::Natural::{self, Large, Small};
use std::mem;
use std::ops::{Mul, MulAssign};

/// Multiplies a `Natural` by a `Natural`, taking both `Natural`s by value.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((Natural::from(1u32) * Natural::from(123u32)).to_string(), "123");
/// assert_eq!((Natural::from(123u32) * Natural::from(0u32)).to_string(), "0");
/// assert_eq!((Natural::from(123u32) * Natural::from(456u32)).to_string(), "56088");
/// assert_eq!((Natural::from_str("123456789000").unwrap() * Natural::from_str("987654321000")
///            .unwrap()).to_string(), "121932631112635269000000");
/// ```
impl Mul<Natural> for Natural {
    type Output = Natural;

    fn mul(mut self, mut other: Natural) -> Natural {
        if self.significant_bits() >= other.significant_bits() {
            self *= other;
            self
        } else {
            other *= self;
            other
        }
    }
}

/// Multiplies a `Natural` by a `Natural`, taking the left `Natural` by value and the right
/// `Natural` by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((Natural::from(1u32) * &Natural::from(123u32)).to_string(), "123");
/// assert_eq!((Natural::from(123u32) * &Natural::from(0u32)).to_string(), "0");
/// assert_eq!((Natural::from(123u32) * &Natural::from(456u32)).to_string(), "56088");
/// assert_eq!((Natural::from_str("123456789000").unwrap() * &Natural::from_str("987654321000")
///            .unwrap()).to_string(), "121932631112635269000000");
/// ```
impl<'a> Mul<&'a Natural> for Natural {
    type Output = Natural;

    fn mul(mut self, other: &'a Natural) -> Natural {
        self *= other;
        self
    }
}

/// Multiplies a `Natural` by a `Natural`, taking the left `Natural` by reference and the right
/// `Natural` by value.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(1u32) * Natural::from(123u32)).to_string(), "123");
/// assert_eq!((&Natural::from(123u32) * Natural::from(0u32)).to_string(), "0");
/// assert_eq!((&Natural::from(123u32) * Natural::from(456u32)).to_string(), "56088");
/// assert_eq!((&Natural::from_str("123456789000").unwrap() * Natural::from_str("987654321000")
///            .unwrap()).to_string(), "121932631112635269000000");
/// ```
impl<'a> Mul<Natural> for &'a Natural {
    type Output = Natural;

    fn mul(self, mut other: Natural) -> Natural {
        other *= self;
        other
    }
}

/// Multiplies a `Natural` by a `Natural`, taking both `Natural`s by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(1u32) * &Natural::from(123u32)).to_string(), "123");
/// assert_eq!((&Natural::from(123u32) * &Natural::from(0u32)).to_string(), "0");
/// assert_eq!((&Natural::from(123u32) * &Natural::from(456u32)).to_string(), "56088");
/// assert_eq!((&Natural::from_str("123456789000").unwrap() * &Natural::from_str("987654321000")
///            .unwrap()).to_string(), "121932631112635269000000");
/// ```
impl<'a, 'b> Mul<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn mul(self, other: &'a Natural) -> Natural {
        if *self == 0 || *other == 0 {
            return Small(0);
        } else if *self == 1 {
            return other.clone();
        } else if *other == 1 {
            return self.clone();
        }
        if let Small(y) = *other {
            self * y
        } else if let Small(x) = *self {
            other * x
        } else {
            match (self, other) {
                (&Large(ref x), &Large(ref y)) => unsafe {
                    let mut result: mpz_t = mem::uninitialized();
                    gmp::mpz_init(&mut result);
                    gmp::mpz_mul(&mut result, x, y);
                    Large(result)
                },
                _ => unreachable!(),
            }
        }
    }
}

/// Multiplies a `Natural` by a `Natural` in place, taking the `Natural` on the RHS by value.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// let mut x = Natural::from(1u32);
/// x *= Natural::from_str("1000").unwrap();
/// x *= Natural::from_str("2000").unwrap();
/// x *= Natural::from_str("3000").unwrap();
/// x *= Natural::from_str("4000").unwrap();
/// assert_eq!(x.to_string(), "24000000000000");
/// ```
impl MulAssign<Natural> for Natural {
    fn mul_assign(&mut self, mut other: Natural) {
        if other == 0 {
            *self = Small(0);
        } else if *self == 1 {
            *self = other;
        } else if *self == 0 || other == 1 {
        } else if let Small(y) = other {
            *self *= y;
        } else if let Small(x) = *self {
            other *= x;
            *self = other;
        } else {
            match (self, other) {
                (&mut Large(ref mut x), Large(ref y)) => unsafe {
                    gmp::mpz_mul(x, x, y);
                },
                _ => unreachable!(),
            }
        }
    }
}

/// Multiplies a `Natural` by a `Natural` in place, taking the `Natural` on the RHS by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// let mut x = Natural::from(1u32);
/// x *= &Natural::from_str("1000").unwrap();
/// x *= &Natural::from_str("2000").unwrap();
/// x *= &Natural::from_str("3000").unwrap();
/// x *= &Natural::from_str("4000").unwrap();
/// assert_eq!(x.to_string(), "24000000000000");
/// ```
impl<'a> MulAssign<&'a Natural> for Natural {
    fn mul_assign(&mut self, other: &'a Natural) {
        if *other == 0 {
            *self = Small(0);
        } else if *self == 1 {
            self.clone_from(other);
        } else if *self == 0 || *other == 1 {
        } else if let Small(y) = *other {
            *self *= y;
        } else if let Small(x) = *self {
            *self = other * x;
        } else {
            match (self, other) {
                (&mut Large(ref mut x), &Large(ref y)) => unsafe {
                    gmp::mpz_mul(x, x, y);
                },
                _ => unreachable!(),
            }
        }
    }
}

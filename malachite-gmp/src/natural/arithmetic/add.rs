use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::Natural::{self, Large, Small};
use std::mem;
use std::ops::{Add, AddAssign};

/// Adds a `Natural` to a `Natural`, taking both `Natural`s by value.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((Natural::from(0u32) + Natural::from(123u32)).to_string(), "123");
/// assert_eq!((Natural::from(123u32) + Natural::from(0u32)).to_string(), "123");
/// assert_eq!((Natural::from(123u32) + Natural::from(456u32)).to_string(), "579");
/// assert_eq!((Natural::from_str("1000000000000").unwrap() + Natural::from_str("2000000000000")
///            .unwrap()).to_string(), "3000000000000");
/// ```
impl Add<Natural> for Natural {
    type Output = Natural;

    fn add(mut self, mut other: Natural) -> Natural {
        if self.limb_count() >= other.limb_count() {
            self += other;
            self
        } else {
            other += self;
            other
        }
    }
}

/// Adds a `Natural` to a `Natural`, taking the left `Natural` by value and the right `Natural` by
/// reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((Natural::from(0u32) + &Natural::from(123u32)).to_string(), "123");
/// assert_eq!((Natural::from(123u32) + &Natural::from(0u32)).to_string(), "123");
/// assert_eq!((Natural::from(123u32) + &Natural::from(456u32)).to_string(), "579");
/// assert_eq!((Natural::from_str("1000000000000").unwrap() + &Natural::from_str("2000000000000")
///            .unwrap()).to_string(), "3000000000000");
/// ```
impl<'a> Add<&'a Natural> for Natural {
    type Output = Natural;

    fn add(mut self, other: &'a Natural) -> Natural {
        self += other;
        self
    }
}

/// Adds a `Natural` to a `Natural`, taking the left `Natural` by reference and the right `Natural`
/// by value.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(0u32) + Natural::from(123u32)).to_string(), "123");
/// assert_eq!((&Natural::from(123u32) + Natural::from(0u32)).to_string(), "123");
/// assert_eq!((&Natural::from(123u32) + Natural::from(456u32)).to_string(), "579");
/// assert_eq!((&Natural::from_str("1000000000000").unwrap() + Natural::from_str("2000000000000")
///            .unwrap()).to_string(), "3000000000000");
/// ```
impl<'a> Add<Natural> for &'a Natural {
    type Output = Natural;

    fn add(self, mut other: Natural) -> Natural {
        other += self;
        other
    }
}

/// Adds a `Natural` to a `Natural`, taking both `Natural`s by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(0u32) + &Natural::from(123u32)).to_string(), "123");
/// assert_eq!((&Natural::from(123u32) + &Natural::from(0u32)).to_string(), "123");
/// assert_eq!((&Natural::from(123u32) + &Natural::from(456u32)).to_string(), "579");
/// assert_eq!((&Natural::from_str("1000000000000").unwrap() + &Natural::from_str("2000000000000")
///            .unwrap()).to_string(), "3000000000000");
/// ```
impl<'a, 'b> Add<&'a Natural> for &'b Natural {
    type Output = Natural;

    fn add(self, other: &'a Natural) -> Natural {
        if *self == 0 {
            return other.clone();
        } else if *other == 0 {
            return self.clone();
        }
        if let Small(y) = *other {
            self + y
        } else if let Small(x) = *self {
            other + x
        } else {
            match (self, other) {
                (&Large(ref x), &Large(ref y)) => unsafe {
                    let mut result: mpz_t = mem::uninitialized();
                    gmp::mpz_init(&mut result);
                    gmp::mpz_add(&mut result, x, y);
                    Large(result)
                },
                _ => unreachable!(),
            }
        }
    }
}

/// Adds a `Natural` to a `Natural` in place, taking the `Natural` on the RHS by value.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// let mut x = Natural::new();
/// x += Natural::from_str("1000000000000").unwrap();
/// x += Natural::from_str("2000000000000").unwrap();
/// x += Natural::from_str("3000000000000").unwrap();
/// x += Natural::from_str("4000000000000").unwrap();
/// assert_eq!(x.to_string(), "10000000000000");
/// ```
impl AddAssign<Natural> for Natural {
    fn add_assign(&mut self, mut other: Natural) {
        if *self == 0 {
            *self = other;
        } else if other == 0 {
        } else if let Small(y) = other {
            *self += y;
        } else if let Small(x) = *self {
            other += x;
            *self = other;
        } else {
            match (self, other) {
                (&mut Large(ref mut x), Large(ref y)) => unsafe {
                    gmp::mpz_add(x, x, y);
                },
                _ => unreachable!(),
            }
        }
    }
}

/// Adds a `Natural` to a `Natural` in place, taking the `Natural` on the RHS by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// let mut x = Natural::new();
/// x += &Natural::from_str("1000000000000").unwrap();
/// x += &Natural::from_str("2000000000000").unwrap();
/// x += &Natural::from_str("3000000000000").unwrap();
/// x += &Natural::from_str("4000000000000").unwrap();
/// assert_eq!(x.to_string(), "10000000000000");
/// ```
impl<'a> AddAssign<&'a Natural> for Natural {
    fn add_assign(&mut self, other: &'a Natural) {
        if *self == 0 {
            self.clone_from(other);
        } else if *other == 0 {
        } else if let Small(y) = *other {
            *self += y;
        } else if let Small(x) = *self {
            *self = other + x;
        } else {
            match (self, other) {
                (&mut Large(ref mut x), &Large(ref y)) => unsafe {
                    gmp::mpz_add(x, x, y);
                },
                _ => unreachable!(),
            }
        }
    }
}

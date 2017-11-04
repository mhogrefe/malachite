use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::arithmetic::sub_u32::sub_assign_u32_helper;
use natural::Natural::{self, Large, Small};
use std::mem;
use std::ops::{Sub, SubAssign};

/// Subtracts a `Natural` from a `Natural`, taking the left `Natural` by value and the right
/// `Natural` by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!(format!("{:?}", Natural::from(0u32) - &Natural::from(123u32)), "None");
/// assert_eq!(format!("{:?}", Natural::from(123u32) - &Natural::from(0u32)), "Some(123)");
/// assert_eq!(format!("{:?}", Natural::from(456u32) - &Natural::from(123u32)), "Some(333)");
/// assert_eq!(format!("{:?}", Natural::from_str("3000000000000").unwrap() -
///                            &Natural::from_str("1000000000000").unwrap()),
///            "Some(2000000000000)");
/// ```
impl<'a> Sub<&'a Natural> for Natural {
    type Output = Option<Natural>;

    fn sub(mut self, other: &'a Natural) -> Option<Natural> {
        if sub_assign_helper(&mut self, other) {
            None
        } else {
            Some(self)
        }
    }
}

/// Subtracts a `Natural` from a `Natural`, taking both `Natural`s by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!(format!("{:?}", &Natural::from(0u32) - &Natural::from(123u32)), "None");
/// assert_eq!(format!("{:?}", &Natural::from(123u32) - &Natural::from(0u32)), "Some(123)");
/// assert_eq!(format!("{:?}", &Natural::from(456u32) - &Natural::from(123u32)), "Some(333)");
/// assert_eq!(format!("{:?}", &Natural::from_str("3000000000000").unwrap() -
///                            &Natural::from_str("1000000000000").unwrap()),
///            "Some(2000000000000)");
/// ```
impl<'a, 'b> Sub<&'a Natural> for &'b Natural {
    type Output = Option<Natural>;

    fn sub(self, other: &'a Natural) -> Option<Natural> {
        if *other == 0 {
            Some(self.clone())
        } else if let Small(y) = *other {
            self - y
        } else if let Small(_) = *self {
            None
        } else {
            match (self, other) {
                (&Large(ref x), &Large(ref y)) => unsafe {
                    let mut result: mpz_t = mem::uninitialized();
                    gmp::mpz_init(&mut result);
                    gmp::mpz_sub(&mut result, x, y);
                    if gmp::mpz_sgn(&result) == -1 {
                        None
                    } else {
                        let mut result = Large(result);
                        result.demote_if_small();
                        Some(result)
                    }
                },
                _ => unreachable!(),
            }
        }
    }
}

/// Subtracts a `Natural` from a `Natural` in place, taking the `Natural` on the RHS by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use std::str::FromStr;
///
/// let mut x = Natural::from_str("10000000000000").unwrap();
/// x -= &Natural::from_str("1000000000000").unwrap();
/// x -= &Natural::from_str("2000000000000").unwrap();
/// x -= &Natural::from_str("3000000000000").unwrap();
/// x -= &Natural::from_str("4000000000000").unwrap();
/// assert_eq!(x.to_string(), "0");
/// ```
impl<'a> SubAssign<&'a Natural> for Natural {
    fn sub_assign(&mut self, other: &'a Natural) {
        if sub_assign_helper(self, other) {
            panic!("Cannot subtract a Natural from a smaller Natural");
        }
    }
}

fn sub_assign_helper<'a>(x: &mut Natural, y: &'a Natural) -> bool {
    if *y == 0 {
        false
    } else if let Small(y) = *y {
        sub_assign_u32_helper(x, y)
    } else if let Small(_) = *x {
        true
    } else {
        match (&mut (*x), y) {
            (&mut Large(ref mut x), &Large(ref y)) => unsafe {
                gmp::mpz_sub(x, x, y);
                if gmp::mpz_sgn(x) == -1 {
                    return true;
                }
            },
            _ => unreachable!(),
        }
        x.demote_if_small();
        false
    }
}

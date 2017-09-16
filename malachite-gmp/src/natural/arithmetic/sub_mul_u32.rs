use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::arithmetic::sub_u32::sub_assign_u32_helper;
use natural::Natural::{self, Large, Small};
use std::mem;
use traits::{SubMul, SubMulAssign};

/// Subtracts the product of a `Natural` (b) and a `u32` (c) from a `Natural` (self), taking `self`
/// by value and b by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::SubMul;
/// use std::str::FromStr;
///
/// assert_eq!(format!("{:?}", Natural::from(10u32).sub_mul(&Natural::from(3u32), 4)), "None");
/// assert_eq!(format!("{:?}", Natural::from(15u32).sub_mul(&Natural::from(3u32), 4)), "Some(3)");
/// assert_eq!(format!("{:?}", Natural::from_str("1000000000000").unwrap()
///                     .sub_mul(&Natural::from(65536u32), 65536)),
///             "Some(995705032704)");
/// ```
impl<'a> SubMul<&'a Natural, u32> for Natural {
    type Output = Option<Natural>;

    fn sub_mul(mut self, b: &'a Natural, c: u32) -> Option<Natural> {
        if sub_mul_assign_u32_helper(&mut self, b, c) {
            Some(self)
        } else {
            None
        }
    }
}

/// Subtracts the product of a `Natural` (b) and a `u32` (c) from a `Natural` (self), taking `self`
/// and b by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::SubMul;
/// use std::str::FromStr;
///
/// assert_eq!(format!("{:?}", (&Natural::from(10u32)).sub_mul(&Natural::from(3u32), 4)),
///             "None");
/// assert_eq!(format!("{:?}", (&Natural::from(15u32)).sub_mul(&Natural::from(3u32), 4)),
///             "Some(3)");
/// assert_eq!(format!("{:?}", (&Natural::from_str("1000000000000").unwrap())
///                     .sub_mul(&Natural::from(65536u32), 65536)),
///             "Some(995705032704)");
/// ```
impl<'a, 'b> SubMul<&'a Natural, u32> for &'b Natural {
    type Output = Option<Natural>;

    fn sub_mul(self, b: &'a Natural, c: u32) -> Option<Natural> {
        if c == 0 || *b == 0 {
            return Some(self.clone());
        }
        if let Small(small_b) = *b {
            if let Some(product) = small_b.checked_mul(c) {
                return self - product;
            }
        }
        unsafe {
            let mut result: mpz_t = mem::uninitialized();
            match *self {
                Small(small) => gmp::mpz_init_set_ui(&mut result, small.into()),
                Large(ref large) => gmp::mpz_init_set(&mut result, large),
            }
            match b {
                &Small(small) => {
                    let mut large_b: mpz_t = mem::uninitialized();
                    gmp::mpz_init_set_ui(&mut large_b, small.into());
                    gmp::mpz_submul_ui(&mut result, &large_b, c.into());
                }
                &Large(ref large_b) => gmp::mpz_submul_ui(&mut result, large_b, c.into()),
            }
            if gmp::mpz_sgn(&result) == -1 {
                None
            } else {
                let mut result = Large(result);
                result.demote_if_small();
                Some(result)
            }
        }
    }
}

/// Subtracts the product of a `Natural` (b) and a `u32` (c) from a `Natural` (self), in place,
/// taking b by reference.
///
/// # Examples
/// ```
/// use malachite_gmp::natural::Natural;
/// use malachite_gmp::traits::SubMulAssign;
/// use std::str::FromStr;
///
/// let mut x = Natural::from(15u32);
/// x.sub_mul_assign(&Natural::from(3u32), 4);
/// assert_eq!(x, 3);
///
/// let mut x = Natural::from_str("1000000000000").unwrap();
/// x.sub_mul_assign(&Natural::from(65536u32), 65536);
/// assert_eq!(x.to_string(), "995705032704");
/// ```
impl<'a> SubMulAssign<&'a Natural, u32> for Natural {
    fn sub_mul_assign(&mut self, b: &'a Natural, c: u32) {
        if !sub_mul_assign_u32_helper(self, b, c) {
            panic!("Natural sub_mul_assign can not have a negative result");
        }
    }
}

fn sub_mul_assign_u32_helper(a: &mut Natural, b: &Natural, c: u32) -> bool {
    if c == 0 || *b == 0 {
        return true;
    }
    if let Small(small_b) = *b {
        if let Some(product) = small_b.checked_mul(c) {
            return sub_assign_u32_helper(a, product);
        }
    }
    let valid = unsafe {
        let large_a = a.promote_in_place();
        match b {
            &Small(small) => {
                let mut large_b: mpz_t = mem::uninitialized();
                gmp::mpz_init_set_ui(&mut large_b, small.into());
                gmp::mpz_submul_ui(large_a, &large_b, c.into());
            }
            &Large(ref large_b) => gmp::mpz_submul_ui(large_a, large_b, c.into()),
        }
        gmp::mpz_sgn(large_a) != -1
    };
    if valid {
        a.demote_if_small();
    }
    valid
}
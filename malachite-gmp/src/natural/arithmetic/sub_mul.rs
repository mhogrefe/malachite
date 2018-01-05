use gmp_mpfr_sys::gmp::{self, mpz_t};
use natural::Natural::{self, Large, Small};
use malachite_base::traits::{SubMul, SubMulAssign};
use natural::arithmetic::sub_mul_u32::sub_mul_assign_u32_helper;
use std::mem;

/// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
/// `self` by value and b and c by reference.
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMul;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert_eq!(format!("{:?}", Natural::from(20u32)
///         .sub_mul(&Natural::from(3u32), &Natural::from(4u32))),
///         "Some(8)");
///     assert_eq!(Natural::from(10u32).sub_mul(&Natural::from(3u32), &Natural::from(4u32)),
///         None);
///     assert_eq!(format!("{:?}", Natural::trillion().sub_mul(&Natural::from(0x1_0000u32),
///         &Natural::from(0x1_0000u32))), "Some(995705032704)");
/// }
/// ```
impl<'a, 'b> SubMul<&'a Natural, &'b Natural> for Natural {
    type Output = Option<Natural>;

    fn sub_mul(mut self, b: &'a Natural, c: &'b Natural) -> Option<Natural> {
        if sub_mul_assign_helper(&mut self, b, c) {
            None
        } else {
            Some(self)
        }
    }
}

/// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
/// `self`, b, and c by reference.
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMul;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     assert_eq!(format!("{:?}", (&Natural::from(20u32))
///         .sub_mul(&Natural::from(3u32), &Natural::from(4u32))),
///         "Some(8)");
///     assert_eq!((&Natural::from(10u32)).sub_mul(&Natural::from(3u32), &Natural::from(4u32)),
///         None);
///     assert_eq!(format!("{:?}", (&Natural::trillion()).sub_mul(&Natural::from(0x1_0000u32),
///         &Natural::from(0x1_0000u32))), "Some(995705032704)");
/// }
/// ```
impl<'a, 'b, 'c> SubMul<&'a Natural, &'b Natural> for &'c Natural {
    type Output = Option<Natural>;

    fn sub_mul(self, b: &'a Natural, c: &'b Natural) -> Option<Natural> {
        if let Small(small_b) = *b {
            self.sub_mul(c, small_b)
        } else if let Small(small_c) = *c {
            self.sub_mul(b, small_c)
        } else if self.limb_count() < b.limb_count() + c.limb_count() - 1 {
            None
        } else {
            unsafe {
                let mut result: mpz_t = mem::uninitialized();
                match *self {
                    Small(small) => gmp::mpz_init_set_ui(&mut result, small.into()),
                    Large(ref large) => gmp::mpz_init_set(&mut result, large),
                }
                if let Large(ref large_c) = *c {
                    if let Large(ref large_b) = *b {
                        gmp::mpz_submul(&mut result, large_b, large_c);
                    }
                }
                if gmp::mpz_sgn(&result) < 0 {
                    return None;
                }
                let mut result = Large(result);
                result.demote_if_small();
                Some(result)
            }
        }
    }
}

/// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), in place,
/// taking b and c by reference.
///
/// # Panics
/// Panics if `b * c` is greater than `self`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_gmp;
///
/// use malachite_base::traits::SubMulAssign;
/// use malachite_gmp::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(20u32);
///     x.sub_mul_assign(&Natural::from(3u32), &Natural::from(4u32));
///     assert_eq!(x, 8);
///
///     let mut x = Natural::trillion();
///     x.sub_mul_assign(&Natural::from(0x1_0000u32), &Natural::from(0x1_0000u32));
///     assert_eq!(x.to_string(), "995705032704");
/// }
/// ```
impl<'a, 'b> SubMulAssign<&'a Natural, &'b Natural> for Natural {
    fn sub_mul_assign(&mut self, b: &'a Natural, c: &'b Natural) {
        if sub_mul_assign_helper(self, b, c) {
            panic!("Natural sub_mul_assign cannot have a negative result");
        }
    }
}

fn sub_mul_assign_helper(a: &mut Natural, b: &Natural, c: &Natural) -> bool {
    if let Small(small_b) = *b {
        sub_mul_assign_u32_helper(a, c, small_b)
    } else if let Small(small_c) = *c {
        sub_mul_assign_u32_helper(a, b, small_c)
    } else if a.limb_count() < b.limb_count() + c.limb_count() - 1 {
        true
    } else {
        {
            let large_a = a.promote_in_place();
            if let Large(ref large_c) = *c {
                if let Large(ref large_b) = *b {
                    unsafe {
                        gmp::mpz_submul(large_a, large_b, large_c);
                        if gmp::mpz_sgn(large_a) < 0 {
                            return true;
                        }
                    }
                }
            }
        }
        a.demote_if_small();
        false
    }
}

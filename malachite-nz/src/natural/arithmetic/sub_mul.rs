use malachite_base::num::traits::{SubMul, SubMulAssign};

use natural::arithmetic::add_mul::mpz_aorsmul;
use natural::arithmetic::sub_mul_limb::sub_mul_assign_limb_helper;
use natural::Natural::{self, Large, Small};

impl<'a, 'b> SubMul<&'a Natural, &'b Natural> for Natural {
    type Output = Option<Natural>;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self` by value and b and c by reference.
    ///
    /// Time: worst case O(m+np)
    ///
    /// Additional memory: worst case O(np)
    ///
    /// where m = `a.significant_bits()`,
    ///       n = `b.significant_bits()`
    ///       p = `c.significant_bits()`
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::SubMul;
    /// use malachite_nz::natural::Natural;
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
    fn sub_mul(mut self, b: &'a Natural, c: &'b Natural) -> Option<Natural> {
        if sub_mul_assign_helper(&mut self, b, c) {
            None
        } else {
            Some(self)
        }
    }
}

impl<'a, 'b, 'c> SubMul<&'a Natural, &'b Natural> for &'c Natural {
    type Output = Option<Natural>;

    /// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), taking
    /// `self`, b, and c by reference.
    ///
    /// Time: worst case O(m+np)
    ///
    /// Additional memory: worst case O(np)
    ///
    /// where m = `a.significant_bits()`,
    ///       n = `b.significant_bits()`
    ///       p = `c.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::SubMul;
    /// use malachite_nz::natural::Natural;
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
    fn sub_mul(self, b: &'a Natural, c: &'b Natural) -> Option<Natural> {
        if let Small(small_b) = *b {
            self.sub_mul(c, small_b)
        } else if let Small(small_c) = *c {
            self.sub_mul(b, small_c)
        } else if self.limb_count() < b.limb_count() + c.limb_count() - 1 {
            None
        } else {
            let mut self_limbs = self.to_limbs_asc();
            if let Large(ref c_limbs) = *c {
                let mut self_sign = false;
                if let Large(ref b_limbs) = *b {
                    mpz_aorsmul(
                        &mut self_sign,
                        &mut self_limbs,
                        false,
                        b_limbs,
                        false,
                        c_limbs,
                        false,
                    );
                }
                if self_sign {
                    return None;
                }
            }
            let mut result = Large(self_limbs);
            result.trim();
            Some(result)
        }
    }
}

/// Subtracts the product of a `Natural` (b) and a `Natural` (c) from a `Natural` (self), in place,
/// taking b and c by reference.
///
/// Time: worst case O(m+np)
///
/// Additional memory: worst case O(np)
///
/// where m = `a.significant_bits()`,
///       n = `b.significant_bits()`
///       p = `c.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::traits::SubMulAssign;
/// use malachite_nz::natural::Natural;
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
        sub_mul_assign_limb_helper(a, c, small_b)
    } else if let Small(small_c) = *c {
        sub_mul_assign_limb_helper(a, b, small_c)
    } else if a.limb_count() < b.limb_count() + c.limb_count() - 1 {
        true
    } else {
        {
            let a_limbs = a.promote_in_place();
            if let Large(ref c_limbs) = *c {
                let mut self_sign = false;
                if let Large(ref b_limbs) = *b {
                    mpz_aorsmul(
                        &mut self_sign,
                        a_limbs,
                        false,
                        b_limbs,
                        false,
                        c_limbs,
                        false,
                    );
                }
                if self_sign {
                    return true;
                }
            }
        }
        a.trim();
        false
    }
}

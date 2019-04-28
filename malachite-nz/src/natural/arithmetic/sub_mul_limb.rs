use malachite_base::num::traits::{CheckedSub, SplitInHalf};
use malachite_base::num::traits::{SubMul, SubMulAssign, WrappingAddAssign};

use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::Natural::{self, Large, Small};
use platform::{DoubleLimb, Limb};

// Multiply s1 and s2limb, and subtract the s1.len() least significant limbs of the product from r
// and write the result to r. Return the most significant limb of the product, plus borrow-out from
// the subtraction. r.len() >= s1.len().
pub fn mpn_submul_1(r: &mut [Limb], s1: &[Limb], s2limb: Limb) -> Limb {
    let s1_len = s1.len();
    assert!(r.len() >= s1_len);
    let mut borrow = 0;
    let s2limb_double = DoubleLimb::from(s2limb);
    for i in 0..s1_len {
        let product = DoubleLimb::from(s1[i]) * s2limb_double;
        let (upper, mut lower) = product.split_in_half();
        lower.wrapping_add_assign(borrow);
        if lower < borrow {
            borrow = upper.wrapping_add(1);
        } else {
            borrow = upper;
        }
        let limb = r[i];
        lower = limb.wrapping_sub(lower);
        if lower > limb {
            borrow.wrapping_add_assign(1);
        }
        r[i] = lower;
    }
    borrow
}

impl<'a> SubMul<&'a Natural, Limb> for Natural {
    type Output = Option<Natural>;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` by value and b by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
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
    ///     assert_eq!(format!("{:?}", Natural::from(10u32).sub_mul(&Natural::from(3u32), 4)),
    ///         "None");
    ///     assert_eq!(format!("{:?}", Natural::from(15u32).sub_mul(&Natural::from(3u32), 4)),
    ///         "Some(3)");
    ///     assert_eq!(format!("{:?}", Natural::trillion().sub_mul(&Natural::from(0x1_0000u32),
    ///         0x1_0000u32)), "Some(995705032704)");
    /// }
    /// ```
    fn sub_mul(mut self, b: &'a Natural, c: Limb) -> Option<Natural> {
        if sub_mul_assign_limb_helper(&mut self, b, c) {
            None
        } else {
            Some(self)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SubMul<&'a Natural, u32> for Natural {
    type Output = Option<Natural>;

    #[inline]
    fn sub_mul(self, b: &'a Natural, c: u32) -> Option<Natural> {
        self.sub_mul(b, Limb::from(c))
    }
}

impl<'a, 'b> SubMul<&'a Natural, Limb> for &'b Natural {
    type Output = Option<Natural>;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` and b by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
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
    ///     assert_eq!(format!("{:?}", (&Natural::from(10u32)).sub_mul(&Natural::from(3u32), 4)),
    ///                 "None");
    ///     assert_eq!(format!("{:?}", (&Natural::from(15u32)).sub_mul(&Natural::from(3u32), 4)),
    ///                 "Some(3)");
    ///     assert_eq!(format!("{:?}", (&Natural::trillion()).sub_mul(&Natural::from(0x1_0000u32),
    ///         0x1_0000u32)), "Some(995705032704)");
    /// }
    /// ```
    fn sub_mul(self, b: &'a Natural, c: Limb) -> Option<Natural> {
        if c == 0 || *b == 0 as Limb {
            return Some(self.clone());
        }
        let a_limb_count = self.limb_count();
        let b_limb_count = b.limb_count();
        if a_limb_count < b_limb_count {
            return None;
        } else if let Small(small_b) = *b {
            if let Some(product) = small_b.checked_mul(c) {
                return self.checked_sub(product);
            }
        }
        let mut a_limbs = self.to_limbs_asc();
        let borrow = match *b {
            Small(small_b) => mpn_submul_1(&mut a_limbs, &[small_b], c),
            Large(ref b_limbs) => mpn_submul_1(&mut a_limbs, b_limbs, c),
        };
        let nonzero_borrow = {
            if a_limb_count == b_limb_count {
                borrow != 0
            } else {
                limbs_sub_limb_in_place(&mut a_limbs[b_limb_count as usize..], borrow)
            }
        };
        if nonzero_borrow {
            None
        } else {
            let mut difference = Large(a_limbs);
            difference.trim();
            Some(difference)
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a, 'b> SubMul<&'a Natural, u32> for &'b Natural {
    type Output = Option<Natural>;

    #[inline]
    fn sub_mul(self, b: &'a Natural, c: u32) -> Option<Natural> {
        self.sub_mul(b, Limb::from(c))
    }
}

impl<'a> SubMulAssign<&'a Natural, Limb> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), in place,
    /// taking b by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
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
    ///     let mut x = Natural::from(15u32);
    ///     x.sub_mul_assign(&Natural::from(3u32), 4);
    ///     assert_eq!(x, 3);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.sub_mul_assign(&Natural::from(0x1_0000u32), 0x1_0000u32);
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul_assign(&mut self, b: &'a Natural, c: Limb) {
        if sub_mul_assign_limb_helper(self, b, c) {
            panic!("Natural sub_mul_assign cannot have a negative result");
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SubMulAssign<&'a Natural, u32> for Natural {
    #[inline]
    fn sub_mul_assign(&mut self, b: &'a Natural, c: u32) {
        self.sub_mul_assign(b, Limb::from(c));
    }
}

pub(crate) fn sub_mul_assign_limb_helper(a: &mut Natural, b: &Natural, c: Limb) -> bool {
    if c == 0 || *b == 0 as Limb {
        return false;
    }
    if let Small(small_b) = *b {
        if let Some(product) = small_b.checked_mul(c) {
            return a.sub_assign_limb_no_panic(product);
        }
    }
    let a_limb_count = a.limb_count();
    let b_limb_count = b.limb_count();
    if a_limb_count < b_limb_count {
        return true;
    }
    let nonzero_borrow = {
        let a_limbs = a.promote_in_place();
        let borrow = match *b {
            Small(small) => mpn_submul_1(a_limbs, &[small], c),
            Large(ref b_limbs) => mpn_submul_1(a_limbs, b_limbs, c),
        };
        if a_limb_count == b_limb_count {
            borrow != 0
        } else {
            limbs_sub_limb_in_place(&mut a_limbs[b_limb_count as usize..], borrow)
        }
    };
    if !nonzero_borrow {
        a.trim();
    }
    nonzero_borrow
}

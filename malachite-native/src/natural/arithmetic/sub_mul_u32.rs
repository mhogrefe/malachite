use natural::arithmetic::sub_u32::{mpn_sub_1_in_place, sub_assign_u32_helper};
use natural::{get_lower, get_upper};
use natural::Natural::{self, Large, Small};
use traits::{SubMul, SubMulAssign};

// Multiply s1 and s2limb, and subtract the s1.len() least significant limbs of the product from r
// and write the result to r. Return the most significant limb of the product, plus borrow-out from
// the subtraction. r.len() >= s1.len().
pub fn mpn_submul_1(r: &mut [u32], s1: &[u32], s2limb: u32) -> u32 {
    let mut borrow = 0;
    let s2limb_u64 = s2limb as u64;
    for i in 0..s1.len() {
        let product = s1[i] as u64 * s2limb_u64;
        let upper = get_upper(product);
        let mut lower = get_lower(product);
        lower = lower.wrapping_add(borrow);
        if lower < borrow {
            borrow = upper.wrapping_add(1);
        } else {
            borrow = upper;
        }
        let limb = r[i];
        lower = limb.wrapping_sub(lower);
        if lower > limb {
            borrow = borrow.wrapping_add(1);
        }
        r[i] = lower;
    }
    borrow
}

/// Subtracts the product of a `Natural` (b) and a `u32` (c) from a `Natural` (self), taking `self`
/// by value and b by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::SubMul;
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
            None
        } else {
            Some(self)
        }
    }
}

/// Subtracts the product of a `Natural` (b) and a `u32` (c) from a `Natural` (self), taking `self`
/// and b by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::SubMul;
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
        let a_limb_count = self.limb_count();
        let b_limb_count = b.limb_count();
        if a_limb_count < b_limb_count {
            return None;
        } else if let Small(small_b) = *b {
            if let Some(product) = small_b.checked_mul(c) {
                return self - product;
            }
        }
        let mut a_limbs = self.to_limbs_le();
        let borrow = match b {
            &Small(small_b) => mpn_submul_1(&mut a_limbs[..], &[small_b], c),
            &Large(ref b_limbs) => mpn_submul_1(&mut a_limbs[..], b_limbs, c),
        };
        let nonzero_borrow = {
            if a_limb_count == b_limb_count {
                borrow != 0
            } else {
                mpn_sub_1_in_place(&mut a_limbs[b_limb_count as usize..], borrow)
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

/// Subtracts the product of a `Natural` (b) and a `u32` (c) from a `Natural` (self), in place,
/// taking b by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::SubMulAssign;
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
        if sub_mul_assign_u32_helper(self, b, c) {
            panic!("Natural sub_mul_assign can not have a negative result");
        }
    }
}

fn sub_mul_assign_u32_helper(a: &mut Natural, b: &Natural, c: u32) -> bool {
    if c == 0 || *b == 0 {
        return false;
    }
    if let Small(small_b) = *b {
        if let Some(product) = small_b.checked_mul(c) {
            return sub_assign_u32_helper(a, product);
        }
    }
    let a_limb_count = a.limb_count();
    let b_limb_count = b.limb_count();
    if a_limb_count < b_limb_count {
        return true;
    }
    let nonzero_borrow = {
        let a_limbs = a.promote_in_place();
        let borrow = match b {
            &Small(small) => mpn_submul_1(a_limbs, &[small], c),
            &Large(ref b_limbs) => mpn_submul_1(a_limbs, b_limbs, c),
        };
        if a_limb_count == b_limb_count {
            borrow != 0
        } else {
            mpn_sub_1_in_place(&mut a_limbs[b_limb_count as usize..], borrow)
        }
    };
    if !nonzero_borrow {
        a.trim();
    }
    nonzero_borrow
}

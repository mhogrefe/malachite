use natural::arithmetic::sub_u32::sub_assign_u32_helper;
use natural::{get_lower, get_upper};
use natural::Natural::{self, Large, Small};
use traits::{SubMul, SubMulAssign};

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
            Some(self)
        } else {
            None
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
        } else if self.limb_count() < b.limb_count() {
            return None;
        } else if let Small(small_b) = *b {
            if let Some(product) = small_b.checked_mul(c) {
                return self - product;
            }
        }
        let (borrow, difference_limbs) = match (self, b) {
            (&Small(small_self), &Small(small_b)) => {
                large_sub_mul_u32(&[small_self], &[small_b], c)
            }
            (&Small(small_self), &Large(ref b_limbs)) => {
                large_sub_mul_u32(&[small_self], b_limbs, c)
            }
            (&Large(ref self_limbs), &Small(small_b)) => {
                large_sub_mul_u32(self_limbs, &[small_b], c)
            }
            (&Large(ref self_limbs), &Large(ref b_limbs)) => {
                large_sub_mul_u32(self_limbs, b_limbs, c)
            }
        };
        if borrow == 0 {
            let mut difference = Large(difference_limbs);
            difference.trim();
            Some(difference)
        } else {
            None
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
        if !sub_mul_assign_u32_helper(self, b, c) {
            panic!("Natural sub_mul_assign can not have a negative result");
        }
    }
}

fn sub_mul_and_borrow(x: u32, y: u32, multiplicand: u64, borrow: &mut u32) -> u32 {
    let difference = (x as u64)
        .wrapping_sub(y as u64 * multiplicand)
        .wrapping_sub(*borrow as u64);
    let lower = get_lower(difference);
    let upper = get_upper(difference);
    *borrow = upper.wrapping_neg();
    lower
}

// xs.len() must be >= ys.len()
fn large_sub_mul_u32_in_place(xs: &mut [u32], ys: &[u32], multiplicand: u32) -> u32 {
    let mut borrow = 0;
    let mut ys_iter = ys.iter();
    let multiplicand = multiplicand as u64;
    for x in xs.iter_mut() {
        match ys_iter.next() {
            Some(y) => *x = sub_mul_and_borrow(*x, *y, multiplicand, &mut borrow),
            None if borrow != 0 => *x = sub_mul_and_borrow(*x, 0, multiplicand, &mut borrow),
            None => break,
        }
    }
    borrow
}

// xs.len() must be >= ys.len()
fn large_sub_mul_u32(xs: &[u32], ys: &[u32], multiplicand: u32) -> (u32, Vec<u32>) {
    let mut difference_limbs = Vec::new();
    let mut borrow = 0;
    let mut ys_iter = ys.iter();
    let multiplicand = multiplicand as u64;
    for x in xs.iter() {
        difference_limbs.push(match ys_iter.next() {
            Some(y) => sub_mul_and_borrow(*x, *y, multiplicand, &mut borrow),
            None if borrow != 0 => sub_mul_and_borrow(*x, 0, multiplicand, &mut borrow),
            None => *x,
        });
    }
    (borrow, difference_limbs)
}

fn sub_mul_assign_u32_helper(a: &mut Natural, b: &Natural, c: u32) -> bool {
    if c == 0 || *b == 0 {
        return true;
    }
    if let Small(small_b) = *b {
        if let Some(product) = small_b.checked_mul(c) {
            return !sub_assign_u32_helper(a, product);
        }
    }
    if a.limb_count() < b.limb_count() {
        return false;
    }
    let valid = {
        let a_limbs = a.promote_in_place();
        match b {
            &Small(small) => large_sub_mul_u32_in_place(a_limbs, &[small], c) == 0,
            &Large(ref b_limbs) => large_sub_mul_u32_in_place(a_limbs, b_limbs, c) == 0,
        }
    };
    if valid {
        a.trim();
    }
    valid
}

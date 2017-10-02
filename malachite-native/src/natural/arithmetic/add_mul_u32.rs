use natural::arithmetic::add_u32::mpn_add_1_in_place;
use natural::Natural::{self, Large, Small};
use natural::{get_lower, LIMB_BITS};
use traits::{AddMul, AddMulAssign};

// Multiply s1 and s2limb, and add the s1.len() least significant limbs of the product to r and
// write the result to r. Return the most significant limb of the product, plus carry-out from the
// addition. r.len() >= s1.len()
pub fn mpn_addmul_1(r: &mut [u32], s1: &[u32], s2limb: u32) -> u32 {
    let s1_len = s1.len();
    assert!(r.len() >= s1_len);
    let mut carry = 0;
    let s2limb_u64 = s2limb as u64;
    for i in 0..s1_len {
        let limb_result = r[i] as u64 + s1[i] as u64 * s2limb_u64 + carry;
        r[i] = get_lower(limb_result);
        carry = limb_result >> LIMB_BITS;
    }
    carry as u32
}

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), taking `self` and b
/// by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `min(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!(Natural::from(10u32).add_mul(Natural::from(3u32), 4), 22);
/// assert_eq!(Natural::from_str("1000000000000").unwrap()
///                     .add_mul(Natural::from(65536u32), 65536).to_string(),
///            "1004294967296");
/// ```
impl AddMul<Natural, u32> for Natural {
    type Output = Natural;

    fn add_mul(mut self, b: Natural, c: u32) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), taking `self` by
/// value and b by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `b.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!(Natural::from(10u32).add_mul(&Natural::from(3u32), 4), 22);
/// assert_eq!(Natural::from_str("1000000000000").unwrap()
///                     .add_mul(&Natural::from(65536u32), 65536).to_string(),
///            "1004294967296");
/// ```
impl<'a> AddMul<&'a Natural, u32> for Natural {
    type Output = Natural;

    fn add_mul(mut self, b: &'a Natural, c: u32) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), taking `self` by
/// reference and b by value.
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
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(10u32)).add_mul(Natural::from(3u32), 4), 22);
/// assert_eq!((&Natural::from_str("1000000000000").unwrap())
///                     .add_mul(Natural::from(65536u32), 65536).to_string(),
///            "1004294967296");
/// ```
impl<'a> AddMul<Natural, u32> for &'a Natural {
    type Output = Natural;

    fn add_mul(self, b: Natural, c: u32) -> Natural {
        if c == 0 || b == 0 {
            return self.clone();
        }
        if c == 1 {
            return self + b;
        }
        if let Small(small_b) = b {
            if let Some(product) = small_b.checked_mul(c) {
                return self + product;
            }
        }
        let mut result_limbs = self.to_limbs_le();
        let a_len = result_limbs.len();
        let b_len = b.limb_count() as usize;
        if a_len < b_len {
            result_limbs.resize(b_len, 0);
        }
        let carry = match b {
            Small(small) => mpn_addmul_1(&mut result_limbs[..], &[small], c),
            Large(ref b_limbs) => mpn_addmul_1(&mut result_limbs[..], b_limbs, c),
        };
        if carry != 0 {
            if a_len > b_len {
                mpn_add_1_in_place(&mut result_limbs[b_len..], carry);
            } else {
                result_limbs.push(carry);
            }
        }
        Large(result_limbs)
    }
}

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), taking `self` and b
/// by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `max(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMul;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(10u32)).add_mul(&Natural::from(3u32), 4), 22);
/// assert_eq!((&Natural::from_str("1000000000000").unwrap())
///                     .add_mul(&Natural::from(65536u32), 65536).to_string(),
///             "1004294967296");
/// ```
impl<'a, 'b> AddMul<&'a Natural, u32> for &'b Natural {
    type Output = Natural;

    fn add_mul(self, b: &'a Natural, c: u32) -> Natural {
        if c == 0 || *b == 0 {
            return self.clone();
        }
        if c == 1 {
            return self + b;
        }
        if let Small(small_b) = *b {
            if let Some(product) = small_b.checked_mul(c) {
                return self + product;
            }
        }
        let mut result_limbs = self.to_limbs_le();
        let a_len = result_limbs.len();
        let b_len = b.limb_count() as usize;
        if a_len < b_len {
            result_limbs.resize(b_len, 0);
        }
        let carry = match b {
            &Small(small) => mpn_addmul_1(&mut result_limbs[..], &[small], c),
            &Large(ref b_limbs) => mpn_addmul_1(&mut result_limbs[..], b_limbs, c),
        };
        if carry != 0 {
            if a_len > b_len {
                mpn_add_1_in_place(&mut result_limbs[b_len..], carry);
            } else {
                result_limbs.push(carry);
            }
        }
        Large(result_limbs)
    }
}

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), in place, taking b by
/// value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `min(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMulAssign;
/// use std::str::FromStr;
///
/// let mut x = Natural::from(10u32);
/// x.add_mul_assign(Natural::from(3u32), 4);
/// assert_eq!(x, 22);
///
/// let mut x = Natural::from_str("1000000000000").unwrap();
/// x.add_mul_assign(Natural::from(65536u32), 65536);
/// assert_eq!(x.to_string(), "1004294967296");
/// ```
impl AddMulAssign<Natural, u32> for Natural {
    fn add_mul_assign(&mut self, b: Natural, c: u32) {
        if c == 0 || b == 0 {
            return;
        }
        if c == 1 {
            *self += b;
            return;
        }
        if let Small(small_b) = b {
            if let Some(product) = small_b.checked_mul(c) {
                *self += product;
                return;
            }
        }
        {
            let mut self_limbs = self.promote_in_place();
            let a_len = self_limbs.len();
            let b_len = b.limb_count() as usize;
            if a_len < b_len {
                self_limbs.resize(b_len, 0);
            }
            let carry = match b {
                Small(small) => mpn_addmul_1(self_limbs, &[small], c),
                Large(ref b_limbs) => mpn_addmul_1(self_limbs, b_limbs, c),
            };
            if carry != 0 {
                if a_len > b_len {
                    mpn_add_1_in_place(&mut self_limbs[b_len..], carry);
                } else {
                    self_limbs.push(carry);
                }
            }
        }
    }
}

/// Adds the product of a `Natural` (b) and a `u32` (c) to a `Natural` (self), in place, taking b by
/// reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `b.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
/// use malachite_native::traits::AddMulAssign;
/// use std::str::FromStr;
///
/// let mut x = Natural::from(10u32);
/// x.add_mul_assign(&Natural::from(3u32), 4);
/// assert_eq!(x, 22);
///
/// let mut x = Natural::from_str("1000000000000").unwrap();
/// x.add_mul_assign(&Natural::from(65536u32), 65536);
/// assert_eq!(x.to_string(), "1004294967296");
/// ```
impl<'a> AddMulAssign<&'a Natural, u32> for Natural {
    fn add_mul_assign(&mut self, b: &'a Natural, c: u32) {
        if c == 0 || *b == 0 {
            return;
        }
        if c == 1 {
            *self += b;
            return;
        }
        if let Small(small_b) = *b {
            if let Some(product) = small_b.checked_mul(c) {
                *self += product;
                return;
            }
        }
        {
            let mut self_limbs = self.promote_in_place();
            let a_len = self_limbs.len();
            let b_len = b.limb_count() as usize;
            if a_len < b_len {
                self_limbs.resize(b_len, 0);
            }
            let carry = match b {
                &Small(small) => mpn_addmul_1(self_limbs, &[small], c),
                &Large(ref b_limbs) => mpn_addmul_1(self_limbs, b_limbs, c),
            };
            if carry != 0 {
                if a_len > b_len {
                    mpn_add_1_in_place(&mut self_limbs[b_len..], carry);
                } else {
                    self_limbs.push(carry);
                }
            }
        }
    }
}

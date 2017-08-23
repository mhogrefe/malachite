use natural::{get_lower, get_upper};
use natural::Natural::{self, Large, Small};
use std::ops::{Mul, MulAssign};
use traits::Assign;

/// Multiplies a `Natural` by a `u32`, taking the `Natural` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((Natural::from(0u32) * 123).to_string(), "0");
/// assert_eq!((Natural::from(123u32) * 1).to_string(), "123");
/// assert_eq!((Natural::from(123u32) * 456).to_string(), "56088");
/// assert_eq!((Natural::from_str("1000000000000").unwrap() * 123).to_string(), "123000000000000");
/// ```
impl Mul<u32> for Natural {
    type Output = Natural;

    fn mul(mut self, other: u32) -> Natural {
        self *= other;
        self
    }
}

/// Multiplies a `Natural` by a `u32`, taking the `Natural` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((&Natural::from(0u32) * 123).to_string(), "0");
/// assert_eq!((&Natural::from(123u32) * 1).to_string(), "123");
/// assert_eq!((&Natural::from(123u32) * 456).to_string(), "56088");
/// assert_eq!((&Natural::from_str("1000000000000").unwrap() * 123).to_string(), "123000000000000");
/// ```
impl<'a> Mul<u32> for &'a Natural {
    type Output = Natural;

    fn mul(self, other: u32) -> Natural {
        if *self == 0 || other == 0 {
            return Natural::from(0u32);
        }
        if other == 1 {
            return self.clone();
        }
        match *self {
            Small(small) => {
                let product = small as u64 * other as u64;
                let lower = get_lower(product);
                let upper = get_upper(product);
                if upper == 0 {
                    Small(lower)
                } else {
                    Large(vec![lower, upper])
                }
            }
            Large(ref limbs) => Large(large_mul_u32(limbs, other)),
        }
    }
}

/// Multiplies a `u32` by a `Natural`, taking the `Natural` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `other.significant_bits()`
///
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((123 * Natural::from(0u32)).to_string(), "0");
/// assert_eq!((1 * Natural::from(123u32)).to_string(), "123");
/// assert_eq!((456 * Natural::from(123u32)).to_string(), "56088");
/// assert_eq!((123 * Natural::from_str("1000000000000").unwrap()).to_string(), "123000000000000");
/// ```
impl Mul<Natural> for u32 {
    type Output = Natural;

    fn mul(self, mut other: Natural) -> Natural {
        other *= self;
        other
    }
}

/// Multiplies a `u32` by a `Natural`, taking the `Natural` by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// ```
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// assert_eq!((123 * &Natural::from(0u32)).to_string(), "0");
/// assert_eq!((1 * &Natural::from(123u32)).to_string(), "123");
/// assert_eq!((456 * &Natural::from(123u32)).to_string(), "56088");
/// assert_eq!((123 * &Natural::from_str("1000000000000").unwrap()).to_string(), "123000000000000");
/// ```
impl<'a> Mul<&'a Natural> for u32 {
    type Output = Natural;

    fn mul(self, other: &'a Natural) -> Natural {
        other * self
    }
}

/// Multiplies a `Natural` by a `u32` in place.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Examples
/// ```
/// use malachite_native::natural::Natural;
///
/// let mut x = Natural::from(1u32);
/// x *= 1;
/// x *= 2;
/// x *= 3;
/// x *= 4;
/// assert_eq!(x.to_string(), "24");
/// ```
impl MulAssign<u32> for Natural {
    fn mul_assign(&mut self, other: u32) {
        if *self == 0 || other == 0 {
            self.assign(0u32);
            return;
        }
        if other == 1 {
            return;
        }
        if *self == 1 {
            self.assign(other);
            return;
        }
        mutate_with_possible_promotion!(
            self,
            small,
            limbs,
            {
                small.checked_mul(other)
            },
            {
                let carry = large_mul_u32_in_place(&mut limbs[..], other);
                if carry != 0 {
                    limbs.push(carry);
                }
            }
        );
    }
}

fn large_mul_u32_in_place(limbs: &mut [u32], multiplicand: u32) -> u32 {
    let mut carry = 0;
    let multiplicand_u64 = multiplicand as u64;
    for limb in limbs.iter_mut() {
        let limb_result = *limb as u64 * multiplicand_u64 + carry as u64;
        *limb = get_lower(limb_result);
        carry = get_upper(limb_result);
    }
    carry
}

pub(crate) fn large_mul_u32(limbs: &[u32], multiplicand: u32) -> Vec<u32> {
    let mut product_limbs = Vec::with_capacity(limbs.len());
    let mut carry = 0;
    let multiplicand_u64 = multiplicand as u64;
    for limb in limbs.iter() {
        let limb_result = *limb as u64 * multiplicand_u64 + carry as u64;
        product_limbs.push(get_lower(limb_result));
        carry = get_upper(limb_result);
    }
    if carry != 0 {
        product_limbs.push(carry);
    }
    product_limbs
}

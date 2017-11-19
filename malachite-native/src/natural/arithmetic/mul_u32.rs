use malachite_base::num::{get_lower, get_upper};
use malachite_base::traits::{Assign, Zero};
use natural::Natural::{self, Large, Small};
use std::ops::{Mul, MulAssign};

// Multiply s1 by s2limb, and write the n least significant limbs of the product to r. Return the
// most significant limb of the product. r.len() >= s1.len().
pub fn mpn_mul_1(r: &mut [u32], s1: &[u32], s2limb: u32) -> u32 {
    let s1_len = s1.len();
    assert!(r.len() >= s1_len);
    let mut carry = 0;
    let s2limb_u64 = s2limb as u64;
    for i in 0..s1_len {
        let limb_result = s1[i] as u64 * s2limb_u64 + carry as u64;
        r[i] = get_lower(limb_result);
        carry = get_upper(limb_result);
    }
    carry
}

// Multiply s1 by s2limb, and write the n least significant limbs of the product to s1. Return the
// most significant limb of the product.
pub fn mpn_mul_1_in_place(s1: &mut [u32], s2limb: u32) -> u32 {
    let mut carry = 0;
    let s2limb_u64 = s2limb as u64;
    for limb in s1.iter_mut() {
        let limb_result = *limb as u64 * s2limb_u64 + carry as u64;
        *limb = get_lower(limb_result);
        carry = get_upper(limb_result);
    }
    carry
}

pub(crate) fn mpn_mul_1c(r: &mut [u32], s1: &[u32], s2limb: u32, mut carry: u32) -> u32 {
    let s1_len = s1.len();
    assert!(r.len() >= s1_len);
    let s2limb_u64 = s2limb as u64;
    for i in 0..s1_len {
        let limb_result = s1[i] as u64 * s2limb_u64 + carry as u64;
        r[i] = get_lower(limb_result);
        carry = get_upper(limb_result);
    }
    carry
}

/// Multiplies a `Natural` by a `u32`, taking the `Natural` by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// ```
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((Natural::zero() * 123).to_string(), "0");
///     assert_eq!((Natural::from(123u32) * 1).to_string(), "123");
///     assert_eq!((Natural::from(123u32) * 456).to_string(), "56088");
///     assert_eq!((Natural::from_str("1000000000000").unwrap() * 123).to_string(),
///         "123000000000000");
/// }
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
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((&Natural::zero() * 123).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) * 1).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) * 456).to_string(), "56088");
///     assert_eq!((&Natural::from_str("1000000000000").unwrap() * 123).to_string(),
///         "123000000000000");
/// }
/// ```
impl<'a> Mul<u32> for &'a Natural {
    type Output = Natural;

    fn mul(self, other: u32) -> Natural {
        if *self == 0 || other == 0 {
            return Natural::zero();
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
            Large(ref limbs) => {
                let mut product_limbs = vec![0; limbs.len()];
                let carry = mpn_mul_1(&mut product_limbs, limbs, other);
                if carry != 0 {
                    product_limbs.push(carry);
                }
                Large(product_limbs)
            }
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
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((123 * Natural::zero()).to_string(), "0");
///     assert_eq!((1 * Natural::from(123u32)).to_string(), "123");
///     assert_eq!((456 * Natural::from(123u32)).to_string(), "56088");
///     assert_eq!((123 * Natural::from_str("1000000000000").unwrap()).to_string(),
///         "123000000000000");
/// }
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
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::Zero;
/// use malachite_native::natural::Natural;
/// use std::str::FromStr;
///
/// fn main() {
///     assert_eq!((123 * &Natural::zero()).to_string(), "0");
///     assert_eq!((1 * &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((456 * &Natural::from(123u32)).to_string(), "56088");
///     assert_eq!((123 * &Natural::from_str("1000000000000").unwrap()).to_string(),
///         "123000000000000");
/// }
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
/// extern crate malachite_base;
/// extern crate malachite_native;
///
/// use malachite_base::traits::One;
/// use malachite_native::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::one();
///     x *= 1;
///     x *= 2;
///     x *= 3;
///     x *= 4;
///     assert_eq!(x.to_string(), "24");
/// }
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
                let carry = mpn_mul_1_in_place(limbs, other);
                if carry != 0 {
                    limbs.push(carry);
                }
            }
        );
    }
}

use malachite_base::num::SplitInHalf;
use malachite_base::num::{Assign, Zero};
use natural::Natural::{self, Large, Small};
use std::ops::{Mul, MulAssign};

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the product of the `Natural` and a `u32`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul_u32::limbs_mul_limb;
///
/// assert_eq!(limbs_mul_limb(&[123, 456], 789), &[97_047, 359_784]);
/// assert_eq!(limbs_mul_limb(&[0xffff_ffff, 5], 2), &[4_294_967_294, 11]);
/// assert_eq!(limbs_mul_limb(&[0xffff_ffff], 2), &[4_294_967_294, 1]);
/// ```
pub fn limbs_mul_limb(limbs: &[u32], limb: u32) -> Vec<u32> {
    let mut carry = 0;
    let limb = u64::from(limb);
    let mut result_limbs = Vec::with_capacity(limbs.len());
    for &x in limbs {
        let limb_result = u64::from(x) * limb + u64::from(carry);
        result_limbs.push(limb_result.lower_half());
        carry = limb_result.upper_half();
    }
    if carry != 0 {
        result_limbs.push(carry);
    }
    result_limbs
}

pub(crate) fn limbs_mul_limb_with_carry_to_out(
    out_limbs: &mut [u32],
    in_limbs: &[u32],
    limb: u32,
    mut carry: u32,
) -> u32 {
    let len = in_limbs.len();
    assert!(out_limbs.len() >= len);
    let limb = u64::from(limb);
    for i in 0..len {
        let limb_result = u64::from(in_limbs[i]) * limb + u64::from(carry);
        out_limbs[i] = limb_result.lower_half();
        carry = limb_result.upper_half();
    }
    carry
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the product of the `Natural` and a `u32` to an output slice. The output slice must be
/// at least as long as the input slice. Returns the 32-bit carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out_limbs` is shorter than `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul_u32::limbs_mul_limb_to_out;
///
/// let mut out_limbs = vec![0, 0, 0];
/// assert_eq!(limbs_mul_limb_to_out(&mut out_limbs, &[123, 456], 789), 0);
/// assert_eq!(out_limbs, &[97_047, 359_784, 0]);
///
/// let mut out_limbs = vec![0, 0, 0];
/// assert_eq!(limbs_mul_limb_to_out(&mut out_limbs, &[0xffff_ffff], 2), 1);
/// assert_eq!(out_limbs, &[4_294_967_294, 0, 0]);
/// ```
pub fn limbs_mul_limb_to_out(out_limbs: &mut [u32], in_limbs: &[u32], limb: u32) -> u32 {
    limbs_mul_limb_with_carry_to_out(out_limbs, in_limbs, limb, 0)
}

/// Interpreting a slice of `u32`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the product of the `Natural` and a `u32` to the input slice. Returns the 32-bit carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul_u32::limbs_slice_mul_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_slice_mul_limb_in_place(&mut limbs, 789), 0);
/// assert_eq!(limbs, &[97_047, 359_784]);
///
/// let mut limbs = vec![0xffff_ffff];
/// assert_eq!(limbs_slice_mul_limb_in_place(&mut limbs, 2), 1);
/// assert_eq!(limbs, &[4_294_967_294]);
/// ```
pub fn limbs_slice_mul_limb_in_place(limbs: &mut [u32], limb: u32) -> u32 {
    let mut carry = 0;
    let limb = u64::from(limb);
    for x in limbs.iter_mut() {
        let limb_result = u64::from(*x) * limb + u64::from(carry);
        *x = limb_result.lower_half();
        carry = limb_result.upper_half();
    }
    carry
}

/// Interpreting a `Vec` of `u32`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the product of the `Natural` and a `u32` to the input `Vec`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul_u32::limbs_vec_mul_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_mul_limb_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[97_047, 359_784]);
///
/// let mut limbs = vec![0xffff_ffff];
/// limbs_vec_mul_limb_in_place(&mut limbs, 2);
/// assert_eq!(limbs, &[4_294_967_294, 1]);
/// ```
pub fn limbs_vec_mul_limb_in_place(limbs: &mut Vec<u32>, limb: u32) {
    let carry = limbs_slice_mul_limb_in_place(limbs, limb);
    if carry != 0 {
        limbs.push(carry);
    }
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
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Natural::ZERO * 123).to_string(), "0");
///     assert_eq!((Natural::from(123u32) * 1).to_string(), "123");
///     assert_eq!((Natural::from(123u32) * 456).to_string(), "56088");
///     assert_eq!((Natural::trillion() * 123).to_string(), "123000000000000");
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
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::ZERO * 123).to_string(), "0");
///     assert_eq!((&Natural::from(123u32) * 1).to_string(), "123");
///     assert_eq!((&Natural::from(123u32) * 456).to_string(), "56088");
///     assert_eq!((&Natural::trillion() * 123).to_string(), "123000000000000");
/// }
/// ```
impl<'a> Mul<u32> for &'a Natural {
    type Output = Natural;

    fn mul(self, other: u32) -> Natural {
        if *self == 0 || other == 0 {
            return Natural::ZERO;
        }
        if other == 1 {
            return self.clone();
        }
        match *self {
            Small(small) => {
                let product = u64::from(small) * u64::from(other);
                let (upper, lower) = product.split_in_half();
                if upper == 0 {
                    Small(lower)
                } else {
                    Large(vec![lower, upper])
                }
            }
            Large(ref limbs) => Large(limbs_mul_limb(limbs, other)),
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
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((123 * Natural::ZERO).to_string(), "0");
///     assert_eq!((1 * Natural::from(123u32)).to_string(), "123");
///     assert_eq!((456 * Natural::from(123u32)).to_string(), "56088");
///     assert_eq!((123 * Natural::trillion()).to_string(), "123000000000000");
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
/// extern crate malachite_nz;
///
/// use malachite_base::num::Zero;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((123 * &Natural::ZERO).to_string(), "0");
///     assert_eq!((1 * &Natural::from(123u32)).to_string(), "123");
///     assert_eq!((456 * &Natural::from(123u32)).to_string(), "56088");
///     assert_eq!((123 * &Natural::trillion()).to_string(), "123000000000000");
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
/// extern crate malachite_nz;
///
/// use malachite_base::num::One;
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::ONE;
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
        mutate_with_possible_promotion!(self, small, limbs, { small.checked_mul(other) }, {
            limbs_vec_mul_limb_in_place(limbs, other);
        });
    }
}

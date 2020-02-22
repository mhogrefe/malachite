use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::SplitInHalf;

use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::{DoubleLimb, Limb};

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
/// limbs of the product of the `Natural` and a `Limb`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul::limb::limbs_mul_limb;
///
/// assert_eq!(limbs_mul_limb(&[123, 456], 789), &[97_047, 359_784]);
/// assert_eq!(limbs_mul_limb(&[0xffff_ffff, 5], 2), &[4_294_967_294, 11]);
/// assert_eq!(limbs_mul_limb(&[0xffff_ffff], 2), &[4_294_967_294, 1]);
/// ```
///
/// This is mpn_mul_1 from mpn/generic/mul_1.c, where the result is returned.
pub fn limbs_mul_limb(limbs: &[Limb], limb: Limb) -> Vec<Limb> {
    let mut carry = 0;
    let limb = DoubleLimb::from(limb);
    let mut result_limbs = Vec::with_capacity(limbs.len());
    for &x in limbs {
        let limb_result = DoubleLimb::from(x) * limb + DoubleLimb::from(carry);
        result_limbs.push(limb_result.lower_half());
        carry = limb_result.upper_half();
    }
    if carry != 0 {
        result_limbs.push(carry);
    }
    result_limbs
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the product of the `Natural` and a `Limb`, plus a carry, to an output slice. The output
/// slice must be at least as long as the input slice. Returns the carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul::limb::limbs_mul_limb_with_carry_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_mul_limb_with_carry_to_out(&mut out, &[123, 456], 789, 10), 0);
/// assert_eq!(out, &[97_057, 359_784, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_mul_limb_with_carry_to_out(&mut out, &[0xffff_ffff], 2, 3), 2);
/// assert_eq!(out, &[1, 0, 0]);
/// ```
///
/// This is mul_1c from gmp-impl.h.
pub fn limbs_mul_limb_with_carry_to_out(
    out: &mut [Limb],
    in_limbs: &[Limb],
    limb: Limb,
    mut carry: Limb,
) -> Limb {
    let len = in_limbs.len();
    assert!(out.len() >= len);
    let limb = DoubleLimb::from(limb);
    for i in 0..len {
        let limb_result = DoubleLimb::from(in_limbs[i]) * limb + DoubleLimb::from(carry);
        out[i] = limb_result.lower_half();
        carry = limb_result.upper_half();
    }
    carry
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the product of the `Natural` and a `Limb` to an output slice. The output slice must be
/// at least as long as the input slice. Returns the carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `in_limbs`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_mul_limb_to_out(&mut out, &[123, 456], 789), 0);
/// assert_eq!(out, &[97_047, 359_784, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_mul_limb_to_out(&mut out, &[0xffff_ffff], 2), 1);
/// assert_eq!(out, &[4_294_967_294, 0, 0]);
/// ```
///
/// This is mpn_mul_1 from mpn/generic/mul_1.c.
#[inline]
pub fn limbs_mul_limb_to_out(out: &mut [Limb], in_limbs: &[Limb], limb: Limb) -> Limb {
    limbs_mul_limb_with_carry_to_out(out, in_limbs, limb, 0)
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the product of the `Natural` and a `Limb`, plus a carry, to the input slice. Returns
/// the carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul::limb::limbs_slice_mul_limb_with_carry_in_place;
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_slice_mul_limb_with_carry_in_place(&mut limbs, 789, 10), 0);
/// assert_eq!(limbs, &[97_057, 359_784]);
///
/// let mut limbs = vec![0xffff_ffff];
/// assert_eq!(limbs_slice_mul_limb_with_carry_in_place(&mut limbs, 2, 3), 2);
/// assert_eq!(limbs, &[1]);
/// ```
///
/// This is mul_1c from gmp-impl.h, where the output is the same as the input.
pub fn limbs_slice_mul_limb_with_carry_in_place(
    limbs: &mut [Limb],
    limb: Limb,
    mut carry: Limb,
) -> Limb {
    let limb = DoubleLimb::from(limb);
    for x in limbs.iter_mut() {
        let limb_result = DoubleLimb::from(*x) * limb + DoubleLimb::from(carry);
        *x = limb_result.lower_half();
        carry = limb_result.upper_half();
    }
    carry
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the product of the `Natural` and a `Limb` to the input slice. Returns the carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul::limb::limbs_slice_mul_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// assert_eq!(limbs_slice_mul_limb_in_place(&mut limbs, 789), 0);
/// assert_eq!(limbs, &[97_047, 359_784]);
///
/// let mut limbs = vec![0xffff_ffff];
/// assert_eq!(limbs_slice_mul_limb_in_place(&mut limbs, 2), 1);
/// assert_eq!(limbs, &[4_294_967_294]);
/// ```
///
/// This is mpn_mul_1 from mpn/generic/mul_1.c, where rp == up.
#[inline]
pub fn limbs_slice_mul_limb_in_place(limbs: &mut [Limb], limb: Limb) -> Limb {
    limbs_slice_mul_limb_with_carry_in_place(limbs, limb, 0)
}

/// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the product of the `Natural` and a `Limb` to the input `Vec`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul::limb::limbs_vec_mul_limb_in_place;
///
/// let mut limbs = vec![123, 456];
/// limbs_vec_mul_limb_in_place(&mut limbs, 789);
/// assert_eq!(limbs, &[97_047, 359_784]);
///
/// let mut limbs = vec![0xffff_ffff];
/// limbs_vec_mul_limb_in_place(&mut limbs, 2);
/// assert_eq!(limbs, &[4_294_967_294, 1]);
/// ```
///
/// This is mpn_mul_1 from mpn/generic/mul_1.c, where the rp == up and instead of returning the
/// carry, it is appended to rp.
pub fn limbs_vec_mul_limb_in_place(limbs: &mut Vec<Limb>, limb: Limb) {
    let carry = limbs_slice_mul_limb_in_place(limbs, limb);
    if carry != 0 {
        limbs.push(carry);
    }
}

impl Natural {
    pub(crate) fn mul_assign_limb(&mut self, other: Limb) {
        if *self == 0 || other == 0 {
            *self = Natural::ZERO;
            return;
        }
        if other == 1 {
            return;
        }
        if *self == 1 {
            *self = Natural::from(other);
            return;
        }
        mutate_with_possible_promotion!(self, small, limbs, { small.checked_mul(other) }, {
            limbs_vec_mul_limb_in_place(limbs, other);
        });
    }

    pub(crate) fn mul_limb_ref(&self, other: Limb) -> Natural {
        if *self == 0 || other == 0 {
            return Natural::ZERO;
        }
        if other == 1 {
            return self.clone();
        }
        Natural(match *self {
            Natural(Small(small)) => {
                let product = DoubleLimb::from(small) * DoubleLimb::from(other);
                let (upper, lower) = product.split_in_half();
                if upper == 0 {
                    Small(lower)
                } else {
                    Large(vec![lower, upper])
                }
            }
            Natural(Large(ref limbs)) => Large(limbs_mul_limb(limbs, other)),
        })
    }
}

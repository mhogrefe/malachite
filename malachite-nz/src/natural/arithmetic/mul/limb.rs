use malachite_base::num::arithmetic::traits::XMulYIsZZ;
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
/// This is mpn_mul_1 from mpn/generic/mul_1.c, GMP 6.1.2, where the result is returned.
pub fn limbs_mul_limb(xs: &[Limb], limb: Limb) -> Vec<Limb> {
    let mut carry = 0;
    let limb = DoubleLimb::from(limb);
    let mut result_limbs = Vec::with_capacity(xs.len());
    for &x in xs {
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
/// This is mul_1c from gmp-impl.h, GMP 6.1.2.
pub fn limbs_mul_limb_with_carry_to_out(
    out: &mut [Limb],
    xs: &[Limb],
    y: Limb,
    mut carry: Limb,
) -> Limb {
    let len = xs.len();
    assert!(out.len() >= len);
    let y = DoubleLimb::from(y);
    for i in 0..len {
        let limb_result = DoubleLimb::from(xs[i]) * y + DoubleLimb::from(carry);
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
/// This is mpn_mul_1 from mpn/generic/mul_1.c, GMP 6.1.2.
#[inline]
pub fn limbs_mul_limb_to_out(out: &mut [Limb], xs: &[Limb], y: Limb) -> Limb {
    limbs_mul_limb_with_carry_to_out(out, xs, y, 0)
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
/// This is mul_1c from gmp-impl.h, GMP 6.1.2, where the output is the same as the input.
pub fn limbs_slice_mul_limb_with_carry_in_place(xs: &mut [Limb], y: Limb, mut carry: Limb) -> Limb {
    let limb = DoubleLimb::from(y);
    for x in xs.iter_mut() {
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
/// This is mpn_mul_1 from mpn/generic/mul_1.c, GMP 6.1.2, where rp == up.
#[inline]
pub fn limbs_slice_mul_limb_in_place(xs: &mut [Limb], y: Limb) -> Limb {
    limbs_slice_mul_limb_with_carry_in_place(xs, y, 0)
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
/// This is mpn_mul_1 from mpn/generic/mul_1.c, GMP 6.1.2, where the rp == up and instead of
/// returning the carry, it is appended to rp.
pub fn limbs_vec_mul_limb_in_place(xs: &mut Vec<Limb>, y: Limb) {
    let carry = limbs_slice_mul_limb_in_place(xs, y);
    if carry != 0 {
        xs.push(carry);
    }
}

impl Natural {
    pub(crate) fn mul_assign_limb(&mut self, other: Limb) {
        match (&mut *self, other) {
            (&mut natural_zero!(), _) | (_, 1) => {}
            (_, 0) => *self = natural_zero!(),
            (&mut natural_one!(), _) => *self = Natural::from(other),
            (&mut Natural(Small(ref mut small)), other) => {
                let (upper, lower) = Limb::x_mul_y_is_zz(*small, other);
                if upper == 0 {
                    *small = lower;
                } else {
                    *self = Natural(Large(vec![lower, upper]));
                }
            }
            (&mut Natural(Large(ref mut limbs)), other) => {
                limbs_vec_mul_limb_in_place(limbs, other);
            }
        }
    }

    pub(crate) fn mul_limb_ref(&self, other: Limb) -> Natural {
        if *self == 0 || other == 0 {
            Natural::ZERO
        } else if other == 1 {
            self.clone()
        } else {
            Natural(match *self {
                Natural(Small(small)) => {
                    let (upper, lower) = Limb::x_mul_y_is_zz(small, other);
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
}

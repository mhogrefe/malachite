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
/// where n = `xs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul::limb::limbs_mul_limb;
///
/// assert_eq!(limbs_mul_limb(&[123, 456], 789), &[97047, 359784]);
/// assert_eq!(limbs_mul_limb(&[u32::MAX, 5], 2), &[u32::MAX - 1, 11]);
/// assert_eq!(limbs_mul_limb(&[u32::MAX], 2), &[u32::MAX - 1, 1]);
/// ```
///
/// This is mpn_mul_1 from mpn/generic/mul_1.c, GMP 6.1.2, where the result is returned.
pub fn limbs_mul_limb(xs: &[Limb], y: Limb) -> Vec<Limb> {
    let mut carry = 0;
    let y = DoubleLimb::from(y);
    let mut out = Vec::with_capacity(xs.len());
    for &x in xs {
        let product = DoubleLimb::from(x) * y + DoubleLimb::from(carry);
        out.push(product.lower_half());
        carry = product.upper_half();
    }
    if carry != 0 {
        out.push(carry);
    }
    out
}

/// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
/// limbs of the product of the `Natural` and a `Limb`, plus a carry, to an output slice. The output
/// slice must be at least as long as the input slice. Returns the carry.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `xs`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul::limb::limbs_mul_limb_with_carry_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_mul_limb_with_carry_to_out(&mut out, &[123, 456], 789, 10), 0);
/// assert_eq!(out, &[97057, 359784, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_mul_limb_with_carry_to_out(&mut out, &[u32::MAX], 2, 3), 2);
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
    let y = DoubleLimb::from(y);
    for (out, x) in out[..xs.len()].iter_mut().zip(xs.iter()) {
        let product = DoubleLimb::from(*x) * y + DoubleLimb::from(carry);
        *out = product.lower_half();
        carry = product.upper_half();
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
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `out` is shorter than `xs`.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul::limb::limbs_mul_limb_to_out;
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_mul_limb_to_out(&mut out, &[123, 456], 789), 0);
/// assert_eq!(out, &[97047, 359784, 0]);
///
/// let mut out = vec![0, 0, 0];
/// assert_eq!(limbs_mul_limb_to_out(&mut out, &[u32::MAX], 2), 1);
/// assert_eq!(out, &[u32::MAX - 1, 0, 0]);
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
/// where n = `xs.len()`
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::mul::limb::limbs_slice_mul_limb_with_carry_in_place;
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_slice_mul_limb_with_carry_in_place(&mut xs, 789, 10), 0);
/// assert_eq!(xs, &[97057, 359784]);
///
/// let mut xs = vec![u32::MAX];
/// assert_eq!(limbs_slice_mul_limb_with_carry_in_place(&mut xs, 2, 3), 2);
/// assert_eq!(xs, &[1]);
/// ```
///
/// This is mul_1c from gmp-impl.h, GMP 6.1.2, where the output is the same as the input.
pub fn limbs_slice_mul_limb_with_carry_in_place(xs: &mut [Limb], y: Limb, mut carry: Limb) -> Limb {
    let y = DoubleLimb::from(y);
    for x in xs.iter_mut() {
        let product = DoubleLimb::from(*x) * y + DoubleLimb::from(carry);
        *x = product.lower_half();
        carry = product.upper_half();
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
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_slice_mul_limb_in_place(&mut xs, 789), 0);
/// assert_eq!(xs, &[97047, 359784]);
///
/// let mut xs = vec![u32::MAX];
/// assert_eq!(limbs_slice_mul_limb_in_place(&mut xs, 2), 1);
/// assert_eq!(xs, &[u32::MAX - 1]);
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
/// let mut xs = vec![123, 456];
/// limbs_vec_mul_limb_in_place(&mut xs, 789);
/// assert_eq!(xs, &[97047, 359784]);
///
/// let mut xs = vec![u32::MAX];
/// limbs_vec_mul_limb_in_place(&mut xs, 2);
/// assert_eq!(xs, &[u32::MAX - 1, 1]);
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
            (_, 0) => *self = Natural::ZERO,
            (_, 1) | (&mut natural_zero!(), _) => {}
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
        match (self, other) {
            (_, 0) => Natural::ZERO,
            (_, 1) | (natural_zero!(), _) => self.clone(),
            (natural_one!(), _) => Natural::from(other),
            (Natural(Small(small)), other) => Natural({
                let (upper, lower) = Limb::x_mul_y_is_zz(*small, other);
                if upper == 0 {
                    Small(lower)
                } else {
                    Large(vec![lower, upper])
                }
            }),
            (Natural(Large(ref limbs)), other) => Natural(Large(limbs_mul_limb(limbs, other))),
        }
    }
}

use std::mem::swap;

use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::basic::integers::PrimitiveInteger;
use malachite_base::num::conversion::traits::{ExactFrom, SplitInHalf};

use natural::arithmetic::add::{
    limbs_add_greater, limbs_slice_add_greater_in_place_left, limbs_slice_add_limb_in_place,
};
use natural::arithmetic::mul::limb::{limbs_mul_limb_to_out, limbs_slice_mul_limb_in_place};
use natural::arithmetic::mul::limbs_mul_to_out;
use natural::InnerNatural::{Large, Small};
use natural::Natural;
use platform::{DoubleLimb, Limb};

/// Given the limbs of two `Natural`s x and y, and a limb `z`, returns the limbs of x + y * z. `xs`
/// and `ys` should be nonempty and have no trailing zeros, and `z` should be nonzero. The result
/// will have no trailing zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_mul::limbs_add_mul_limb;
///
/// assert_eq!(limbs_add_mul_limb(&[123, 456], &[123], 4), &[615, 456]);
/// assert_eq!(limbs_add_mul_limb(&[123], &[0, 123], 4), &[123, 492]);
/// assert_eq!(limbs_add_mul_limb(&[123, 456], &[0, 123], u32::MAX), &[123, 333, 123]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, GMP 6.1.2, where w and x are positive, sub is
/// positive, and w is returned instead of overwriting the first input.
pub fn limbs_add_mul_limb(xs: &[Limb], ys: &[Limb], limb: Limb) -> Vec<Limb> {
    let mut out;
    if xs.len() >= ys.len() {
        out = xs.to_vec();
        limbs_vec_add_mul_limb_greater_in_place_left(&mut out, ys, limb);
    } else {
        out = ys.to_vec();
        limbs_vec_add_mul_limb_smaller_in_place_right(xs, &mut out, limb);
    }
    out
}

/// Given the equal-length limbs of two `Natural`s x and y, and a limb `z`, computes x + y * z. The
/// lowest `xs.len()` limbs of the result are written to `xs`, and the highest limb of y * z, plus
/// the carry-out from the addition, is returned.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_mul::*;
///
/// let xs = &mut [123];
/// assert_eq!(limbs_slice_add_mul_limb_same_length_in_place_left(xs, &[123], 4), 0);
/// assert_eq!(xs, &[615]);
///
/// let xs = &mut [123];
/// assert_eq!(limbs_slice_add_mul_limb_same_length_in_place_left(xs, &[123], u32::MAX), 123);
/// assert_eq!(xs, &[0]);
///
/// let xs = &mut [123, 0];
/// assert_eq!(limbs_slice_add_mul_limb_same_length_in_place_left(xs, &[0, 123], 4), 0);
/// assert_eq!(xs, &[123, 492]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_slice_add_mul_limb_same_length_in_place_left(xs, &[0, 123], u32::MAX), 123);
/// assert_eq!(xs, &[123, 333]);
/// ```
///
/// This is mpn_addmul_1 from mpn/generic/addmul_1.c, GMP 6.1.2.
pub fn limbs_slice_add_mul_limb_same_length_in_place_left(
    xs: &mut [Limb],
    ys: &[Limb],
    z: Limb,
) -> Limb {
    let len = xs.len();
    assert_eq!(ys.len(), len);
    let mut carry = 0;
    let dz = DoubleLimb::from(z);
    for (x, &y) in xs.iter_mut().zip(ys.iter()) {
        let out = DoubleLimb::from(*x) + DoubleLimb::from(y) * dz + carry;
        *x = out.lower_half();
        carry = out >> Limb::WIDTH;
    }
    Limb::exact_from(carry)
}

/// Given the limbs of two `Natural`s x and y, and a limb `z`, computes x + y * z. The lowest limbs
/// of the result are written to `ys` and the highest limb is returned. `xs` must have the same
/// length as `ys`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()` = `ys.len()`
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_mul::*;
///
/// let ys = &mut [0, 123];
/// assert_eq!(limbs_slice_add_mul_limb_same_length_in_place_right(&[123, 0], ys, 4), 0);
/// assert_eq!(ys, &[123, 492]);
///
/// let ys = &mut [0, 123];
/// assert_eq!(limbs_slice_add_mul_limb_same_length_in_place_right(&[123, 456], ys, u32::MAX),
///         123);
/// assert_eq!(ys, &[123, 333]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, GMP 6.1.2, where w and x are positive and have the
/// same lengths, sub is positive, the lowest limbs of the result are written to the second input
/// rather than the first, and the highest limb is returned.
pub fn limbs_slice_add_mul_limb_same_length_in_place_right(
    xs: &[Limb],
    ys: &mut [Limb],
    z: Limb,
) -> Limb {
    let xs_len = xs.len();
    assert_eq!(ys.len(), xs_len);
    let mut carry = 0;
    let dz = DoubleLimb::from(z);
    for (&x, y) in xs.iter().zip(ys.iter_mut()) {
        let out = DoubleLimb::from(x) + DoubleLimb::from(*y) * dz + carry;
        *y = out.lower_half();
        carry = out >> Limb::WIDTH;
    }
    Limb::exact_from(carry)
}

/// Given the limbs of two `Natural`s a and b, and a limb c, writes the limbs of a + b * c to the
/// first (left) input, corresponding to the limbs of a. `xs` and `ys` should be nonempty and have
/// no trailing zeros, and `z` should be nonzero. The result will have no trailing zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = max(`xs.len()`, `ys.len()`)
///       m = max(1, `ys.len()` - `xs.len()`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_mul::limbs_vec_add_mul_limb_in_place_left;
///
/// let mut xs = vec![123, 456];
/// limbs_vec_add_mul_limb_in_place_left(&mut xs, &[123], 4);
/// assert_eq!(xs, &[615, 456]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_add_mul_limb_in_place_left(&mut xs, &[123], u32::MAX);
/// assert_eq!(xs, &[0, 579]);
///
/// let mut xs = vec![123];
/// limbs_vec_add_mul_limb_in_place_left(&mut xs, &[0, 123], 4);
/// assert_eq!(xs, &[123, 492]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_add_mul_limb_in_place_left(&mut xs, &[0, 123], u32::MAX);
/// assert_eq!(xs, &[123, 333, 123]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, GMP 6.1.2, where w and x are positive and sub is
/// positive.
pub fn limbs_vec_add_mul_limb_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], z: Limb) {
    let xs_len = xs.len();
    if xs_len >= ys.len() {
        limbs_vec_add_mul_limb_greater_in_place_left(xs, ys, z);
    } else {
        xs.resize(ys.len(), 0);
        let (xs_lo, xs_hi) = xs.split_at_mut(xs_len);
        let (ys_lo, ys_hi) = ys.split_at(xs_len);
        let mut carry = limbs_mul_limb_to_out(xs_hi, ys_hi, z);
        let inner_carry = limbs_slice_add_mul_limb_same_length_in_place_left(xs_lo, ys_lo, z);
        if inner_carry != 0 && limbs_slice_add_limb_in_place(xs_hi, inner_carry) {
            carry += 1;
        }
        if carry != 0 {
            xs.push(carry);
        }
    }
}

// ys.len() > 0, xs.len() >= ys.len(), z != 0
fn limbs_vec_add_mul_limb_greater_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], z: Limb) {
    let ys_len = ys.len();
    let carry = limbs_slice_add_mul_limb_same_length_in_place_left(&mut xs[..ys_len], ys, z);
    if carry != 0 {
        if xs.len() == ys_len {
            xs.push(carry);
        } else if limbs_slice_add_limb_in_place(&mut xs[ys_len..], carry) {
            xs.push(1);
        }
    }
}

/// Given the limbs of two `Natural`s x and y, and a limb `z`, writes the limbs of x + y * z to the
/// second (right) input, corresponding to the limbs of y. `xs` and `ys` should be nonempty and have
/// no trailing zeros, and `z` should be nonzero. The result will have no trailing zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = max(`xs.len()`, `ys.len()`)
///       m = max(1, `xs.len()` - `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_mul::limbs_vec_add_mul_limb_in_place_right;
///
/// let mut ys = vec![123];
/// limbs_vec_add_mul_limb_in_place_right(&[123, 456], &mut ys, 4);
/// assert_eq!(ys, &[615, 456]);
///
/// let mut ys = vec![123];
/// limbs_vec_add_mul_limb_in_place_right(&[123, 456], &mut ys, u32::MAX);
/// assert_eq!(ys, &[0, 579]);
///
/// let mut ys = vec![0, 123];
/// limbs_vec_add_mul_limb_in_place_right(&[123], &mut ys, 4);
/// assert_eq!(ys, &[123, 492]);
///
/// let mut ys = vec![0, 123];
/// limbs_vec_add_mul_limb_in_place_right(&[123, 456], &mut ys, u32::MAX);
/// assert_eq!(ys, &[123, 333, 123]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, GMP 6.1.2, where w and x are positive, sub is
/// positive, and the result is written to the second input rather than the first.
pub fn limbs_vec_add_mul_limb_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>, z: Limb) {
    let ys_len = ys.len();
    if xs.len() >= ys_len {
        let carry = limbs_slice_add_mul_limb_same_length_in_place_right(&xs[..ys_len], ys, z);
        ys.extend_from_slice(&xs[ys_len..]);
        if carry != 0 {
            if xs.len() == ys_len {
                ys.push(carry);
            } else if limbs_slice_add_limb_in_place(&mut ys[ys_len..], carry) {
                ys.push(1);
            }
        }
    } else {
        limbs_vec_add_mul_limb_smaller_in_place_right(xs, ys, z);
    }
}

// xs.len() > 0, xs.len() < ys.len(), z != 0
fn limbs_vec_add_mul_limb_smaller_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>, z: Limb) {
    let (ys_lo, ys_hi) = ys.split_at_mut(xs.len());
    let mut carry = limbs_slice_mul_limb_in_place(ys_hi, z);
    let inner_carry = limbs_slice_add_mul_limb_same_length_in_place_right(xs, ys_lo, z);
    if inner_carry != 0 && limbs_slice_add_limb_in_place(ys_hi, inner_carry) {
        carry += 1;
    }
    if carry != 0 {
        ys.push(carry);
    }
}

/// Given the limbs of two `Natural`s x and y, and a limb `z`, writes the limbs of x + y * z to
/// whichever input is longer. If the result is written to the first input, `false` is returned; if
/// to the second, `true` is returned. `xs` and `ys` should be nonempty and have no trailing zeros,
/// and `z` should be nonzero. The result will have no trailing zeros.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_mul::limbs_vec_add_mul_limb_in_place_either;
///
/// let mut xs = vec![123, 456];
/// let mut ys = vec![123];
/// assert_eq!(limbs_vec_add_mul_limb_in_place_either(&mut xs, &mut ys, 4), false);
/// assert_eq!(xs, &[615, 456]);
/// assert_eq!(ys, &[123]);
///
/// let mut xs = vec![123, 456];
/// let mut ys = vec![123];
/// assert_eq!(limbs_vec_add_mul_limb_in_place_either(&mut xs, &mut ys, u32::MAX), false);
/// assert_eq!(xs, &[0, 579]);
/// assert_eq!(ys, &[123]);
///
/// let mut xs = vec![123];
/// let mut ys = vec![0, 123];
/// assert_eq!(limbs_vec_add_mul_limb_in_place_either(&mut xs, &mut ys, 4), true);
/// assert_eq!(xs, &[123]);
/// assert_eq!(ys, &[123, 492]);
///
/// let mut xs = vec![123, 456];
/// let mut ys = vec![0, 123];
/// assert_eq!(limbs_vec_add_mul_limb_in_place_either(&mut xs, &mut ys, u32::MAX), false);
/// assert_eq!(xs, &[123, 333, 123]);
/// assert_eq!(ys, &[0, 123]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, GMP 6.1.2, where w and x are positive, sub is
/// positive, and the result is written to the longer input.
pub fn limbs_vec_add_mul_limb_in_place_either(
    xs: &mut Vec<Limb>,
    ys: &mut Vec<Limb>,
    z: Limb,
) -> bool {
    if xs.len() >= ys.len() {
        limbs_vec_add_mul_limb_greater_in_place_left(xs, ys, z);
        false
    } else {
        limbs_vec_add_mul_limb_smaller_in_place_right(xs, ys, z);
        true
    }
}

/// Given the limbs `xs`, `ys` and `zs` of three `Natural`s x, y, and z, returns the limbs of
/// x + y * z. `xs` should be nonempty and `ys` and `zs` should have length at least 2. None of the
/// slices should have any trailing zeros. The result will have no trailing zeros.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(m + n * log(n))
///
/// where n = max(`ys.len()`, `zs.len()`)
///       m = `xs.len()`
///
/// # Panics
/// Panics if `ys` or `zs` are empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_mul::limbs_add_mul;
///
/// assert_eq!(limbs_add_mul(&[123, 456], &[123, 789], &[321, 654]),
///         &[39606, 334167, 516006]);
/// assert_eq!(limbs_add_mul(&[123, 456, 789, 987, 654], &[123, 789], &[321, 654]),
///         &[39606, 334167, 516795, 987, 654]);
/// ```
///
/// This is mpz_aorsmul from mpz/aorsmul.c, GMP 6.1.2, where w, x, and y are positive, sub is
/// positive, and w is returned instead of overwriting the first input.
pub fn limbs_add_mul(xs: &[Limb], ys: &[Limb], zs: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let mut out_len = ys.len() + zs.len();
    let mut out = vec![0; out_len];
    if limbs_mul_to_out(&mut out, ys, zs) == 0 {
        out_len -= 1;
        out.pop();
    }
    assert_ne!(*out.last().unwrap(), 0);
    if xs_len >= out_len {
        limbs_add_greater(xs, &out)
    } else {
        if limbs_slice_add_greater_in_place_left(&mut out, xs) {
            out.push(1);
        }
        out
    }
}

/// Given the limbs `xs`, `ys` and `zs` of three `Natural`s x, y, and z, computes x + y * z. The
/// limbs of the result are written to `xs`. `xs` should be nonempty and `ys` and `zs` should have
/// length at least 2. None of the slices should have any trailing zeros. The result will have no
/// trailing zeros.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`ys.len()`, `zs.len()`)
///       m = `xs.len()`
///
/// # Panics
/// Panics if `ys` or `zs` are empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::add_mul::limbs_add_mul_in_place_left;
///
/// let mut xs = vec![123, 456];
/// limbs_add_mul_in_place_left(&mut xs, &[123, 789], &[321, 654]);
/// assert_eq!(xs, &[39606, 334167, 516006]);
///
/// let mut xs = vec![123, 456, 789, 987, 654];
/// limbs_add_mul_in_place_left(&mut xs, &[123, 789], &[321, 654]);
/// assert_eq!(xs, &[39606, 334167, 516795, 987, 654]);
/// ```
///
/// This is mpz_aorsmul from mpz/aorsmul.c, GMP 6.1.2, where w, x, and y are positive and sub is
/// positive.
pub fn limbs_add_mul_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], zs: &[Limb]) {
    let xs_len = xs.len();
    let mut out_len = ys.len() + zs.len();
    let mut out = vec![0; out_len];
    if limbs_mul_to_out(&mut out, ys, zs) == 0 {
        out_len -= 1;
        out.pop();
    }
    assert_ne!(*out.last().unwrap(), 0);
    if xs_len < out_len {
        swap(xs, &mut out);
    }
    if limbs_slice_add_greater_in_place_left(xs, &out) {
        xs.push(1);
    }
}

impl Natural {
    fn add_mul_limb_ref_ref(&self, y: &Natural, z: Limb) -> Natural {
        match (self, y, z) {
            (x, _, 0) | (x, natural_zero!(), _) => x.clone(),
            (x, y, 1) => x + y,
            (x, natural_one!(), z) => x + Natural::from(z),
            (Natural(Large(ref xs)), Natural(Large(ref ys)), z) => {
                Natural(Large(limbs_add_mul_limb(xs, ys, z)))
            }
            (x, y, z) => x + y * Natural::from(z),
        }
    }

    fn add_mul_assign_limb(&mut self, mut y: Natural, z: Limb) {
        match (&mut *self, &mut y, z) {
            (_, _, 0) | (_, natural_zero!(), _) => {}
            (x, _, 1) => *x += y,
            (x, natural_one!(), z) => *x += Natural::from(z),
            (Natural(Large(ref mut xs)), Natural(Large(ref mut ys)), z) => {
                if limbs_vec_add_mul_limb_in_place_either(xs, ys, z) {
                    *self = y;
                }
            }
            (x, _, z) => *x += y * Natural::from(z),
        }
    }

    fn add_mul_assign_limb_ref(&mut self, y: &Natural, z: Limb) {
        match (&mut *self, y, z) {
            (_, _, 0) | (_, natural_zero!(), _) => {}
            (x, y, 1) => *x += y,
            (x, natural_one!(), z) => *x += Natural::from(z),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys)), z) => {
                limbs_vec_add_mul_limb_in_place_left(xs, ys, z);
            }
            (x, y, z) => *x += y * Natural::from(z),
        }
    }
}

impl<'a> AddMul<Natural, Natural> for Natural {
    type Output = Natural;

    /// Adds the product of a `Natural` (y) and a `Natural` (z) to a `Natural` (self), taking
    /// `self`, y, and z by value.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///       m = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(10u32).add_mul(Natural::from(3u32), Natural::from(4u32)), 22);
    /// assert_eq!(Natural::trillion().add_mul(Natural::from(0x1_0000u32),
    ///     Natural::trillion()).to_string(), "65537000000000000");
    /// ```
    #[inline]
    fn add_mul(mut self, b: Natural, c: Natural) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

impl<'a> AddMul<Natural, &'a Natural> for Natural {
    type Output = Natural;

    /// Adds the product of a `Natural` (y) and a `Natural` (z) to a `Natural` (self), taking `self`
    /// and y by value and z by reference.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///       m = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(10u32).add_mul(Natural::from(3u32), &Natural::from(4u32)), 22);
    /// assert_eq!(Natural::trillion().add_mul(Natural::from(0x1_0000u32),
    ///     &Natural::trillion()).to_string(), "65537000000000000");
    /// ```
    #[inline]
    fn add_mul(mut self, b: Natural, c: &'a Natural) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

impl<'a> AddMul<&'a Natural, Natural> for Natural {
    type Output = Natural;

    /// Adds the product of a `Natural` (y) and a `Natural` (z) to a `Natural` (self), taking `self`
    /// and z by value and y by reference.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///       m = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(10u32).add_mul(&Natural::from(3u32), Natural::from(4u32)), 22);
    /// assert_eq!(Natural::trillion().add_mul(&Natural::from(0x1_0000u32),
    ///     Natural::trillion()).to_string(), "65537000000000000");
    /// ```
    #[inline]
    fn add_mul(mut self, b: &'a Natural, c: Natural) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

impl<'a, 'b> AddMul<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    /// Adds the product of a `Natural` (y) and a `Natural` (z) to a `Natural` (self), taking `self`
    /// by value and y and z by reference.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///       m = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(10u32).add_mul(&Natural::from(3u32), &Natural::from(4u32)), 22);
    /// assert_eq!(Natural::trillion().add_mul(&Natural::from(0x1_0000u32),
    ///     &Natural::trillion()).to_string(), "65537000000000000");
    /// ```
    #[inline]
    fn add_mul(mut self, b: &'a Natural, c: &'b Natural) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

impl<'a, 'b, 'c> AddMul<&'a Natural, &'b Natural> for &'c Natural {
    type Output = Natural;

    /// Adds the product of a `Natural` (y) and a `Natural` (z) to a `Natural` (self), taking
    /// `self`, y, and z by reference.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(m + n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///       m = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(10u32)).add_mul(&Natural::from(3u32), &Natural::from(4u32)), 22);
    /// assert_eq!((&Natural::trillion()).add_mul(&Natural::from(0x1_0000u32),
    ///     &Natural::trillion()).to_string(), "65537000000000000");
    /// ```
    fn add_mul(self, b: &'a Natural, c: &'b Natural) -> Natural {
        match (self, b, c) {
            (Natural(Small(small_a)), b, c) => (b * c).add_limb(*small_a),
            (a, Natural(Small(small_b)), c) => a.add_mul_limb_ref_ref(c, *small_b),
            (a, b, Natural(Small(small_c))) => a.add_mul_limb_ref_ref(b, *small_c),
            (
                Natural(Large(ref a_limbs)),
                Natural(Large(ref b_limbs)),
                Natural(Large(ref c_limbs)),
            ) => Natural(Large(limbs_add_mul(a_limbs, b_limbs, c_limbs))),
        }
    }
}

impl AddMulAssign<Natural, Natural> for Natural {
    /// Adds the product of a `Natural` (y) and a `Natural` (z) to a `Natural` (self), in place,
    /// taking y and z by value.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///       m = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.add_mul_assign(Natural::from(3u32), Natural::from(4u32));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = Natural::trillion();
    /// x.add_mul_assign(Natural::from(0x1_0000u32), Natural::trillion());
    /// assert_eq!(x.to_string(), "65537000000000000");
    /// ```
    fn add_mul_assign(&mut self, mut b: Natural, mut c: Natural) {
        match (&mut *self, &mut b, &mut c) {
            (Natural(Small(small_a)), _, _) => *self = (b * c).add_limb(*small_a),
            (_, Natural(Small(small_b)), _) => self.add_mul_assign_limb(c, *small_b),
            (_, _, Natural(Small(small_c))) => self.add_mul_assign_limb(b, *small_c),
            (
                Natural(Large(ref mut a_limbs)),
                Natural(Large(ref b_limbs)),
                Natural(Large(ref c_limbs)),
            ) => limbs_add_mul_in_place_left(a_limbs, b_limbs, c_limbs),
        }
    }
}

impl<'a> AddMulAssign<Natural, &'a Natural> for Natural {
    /// Adds the product of a `Natural` (y) and a `Natural` (z) to a `Natural` (self), in place,
    /// taking y by value and z by reference.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///       m = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.add_mul_assign(Natural::from(3u32), &Natural::from(4u32));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = Natural::trillion();
    /// x.add_mul_assign(Natural::from(0x1_0000u32), &Natural::trillion());
    /// assert_eq!(x.to_string(), "65537000000000000");
    /// ```
    fn add_mul_assign(&mut self, mut b: Natural, c: &'a Natural) {
        match (&mut *self, &mut b, c) {
            (Natural(Small(small_a)), _, _) => *self = (b * c).add_limb(*small_a),
            (_, Natural(Small(small_b)), _) => self.add_mul_assign_limb_ref(c, *small_b),
            (_, _, Natural(Small(small_c))) => self.add_mul_assign_limb(b, *small_c),
            (
                Natural(Large(ref mut a_limbs)),
                Natural(Large(ref b_limbs)),
                Natural(Large(ref c_limbs)),
            ) => limbs_add_mul_in_place_left(a_limbs, b_limbs, c_limbs),
        }
    }
}

impl<'a> AddMulAssign<&'a Natural, Natural> for Natural {
    /// Adds the product of a `Natural` (y) and a `Natural` (z) to a `Natural` (self), in place,
    /// taking y by reference and z by value.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///       m = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.add_mul_assign(&Natural::from(3u32), Natural::from(4u32));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = Natural::trillion();
    /// x.add_mul_assign(&Natural::from(0x1_0000u32), Natural::trillion());
    /// assert_eq!(x.to_string(), "65537000000000000");
    /// ```
    fn add_mul_assign(&mut self, b: &'a Natural, mut c: Natural) {
        match (&mut *self, b, &mut c) {
            (Natural(Small(small_a)), _, _) => *self = (b * c).add_limb(*small_a),
            (_, Natural(Small(small_b)), _) => self.add_mul_assign_limb(c, *small_b),
            (_, _, Natural(Small(small_c))) => self.add_mul_assign_limb_ref(b, *small_c),
            (
                Natural(Large(ref mut a_limbs)),
                Natural(Large(ref b_limbs)),
                Natural(Large(ref c_limbs)),
            ) => limbs_add_mul_in_place_left(a_limbs, b_limbs, c_limbs),
        }
    }
}

impl<'a, 'b> AddMulAssign<&'a Natural, &'b Natural> for Natural {
    /// Adds the product of a `Natural` (y) and a `Natural` (z) to a `Natural` (self), in place,
    /// taking y and z by reference.
    ///
    /// Time: O(m + n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///       m = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::AddMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.add_mul_assign(&Natural::from(3u32), &Natural::from(4u32));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = Natural::trillion();
    /// x.add_mul_assign(&Natural::from(0x1_0000u32), &Natural::trillion());
    /// assert_eq!(x.to_string(), "65537000000000000");
    /// ```
    fn add_mul_assign(&mut self, b: &'a Natural, c: &'b Natural) {
        match (&mut *self, b, c) {
            (Natural(Small(small_a)), _, _) => *self = (b * c).add_limb(*small_a),
            (_, Natural(Small(small_b)), _) => self.add_mul_assign_limb_ref(c, *small_b),
            (_, _, Natural(Small(small_c))) => self.add_mul_assign_limb_ref(b, *small_c),
            (
                Natural(Large(ref mut a_limbs)),
                Natural(Large(ref b_limbs)),
                Natural(Large(ref c_limbs)),
            ) => limbs_add_mul_in_place_left(a_limbs, b_limbs, c_limbs),
        }
    }
}

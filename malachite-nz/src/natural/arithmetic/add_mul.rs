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

/// Given the limbs of two `Natural`s a and b, and a limb c, returns the limbs of a + b * c. `xs`
/// and `ys` should be nonempty and have no trailing zeros, and `limb` should be nonzero. The result
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
/// assert_eq!(limbs_add_mul_limb(&[123, 456], &[0, 123], 0xffff_ffff), &[123, 333, 123]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive, sub is positive, and w
/// is returned instead of overwriting the first input.
pub fn limbs_add_mul_limb(xs: &[Limb], ys: &[Limb], limb: Limb) -> Vec<Limb> {
    let mut result;
    if xs.len() >= ys.len() {
        result = xs.to_vec();
        limbs_vec_add_mul_limb_greater_in_place_left(&mut result, ys, limb);
    } else {
        result = ys.to_vec();
        limbs_vec_add_mul_limb_smaller_in_place_right(xs, &mut result, limb);
    }
    result
}

/// Given the equal-length limbs of two `Natural`s a and b, and a limb c, computes a + b * c. The
/// lowest `xs.len()` limbs of the result are written to `xs`, and the highest limb of b * c, plus
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
/// assert_eq!(limbs_slice_add_mul_limb_same_length_in_place_left(xs, &[123], 0xffff_ffff), 123);
/// assert_eq!(xs, &[0]);
///
/// let xs = &mut [123, 0];
/// assert_eq!(limbs_slice_add_mul_limb_same_length_in_place_left(xs, &[0, 123], 4), 0);
/// assert_eq!(xs, &[123, 492]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_slice_add_mul_limb_same_length_in_place_left(xs, &[0, 123], 0xffff_ffff), 123);
/// assert_eq!(xs, &[123, 333]);
/// ```
///
/// This is mpn_addmul_1 from mpn/generic/addmul_1.c.
pub fn limbs_slice_add_mul_limb_same_length_in_place_left(
    xs: &mut [Limb],
    ys: &[Limb],
    limb: Limb,
) -> Limb {
    let len = xs.len();
    assert_eq!(ys.len(), len);
    let mut carry = 0;
    let limb_double = DoubleLimb::from(limb);
    for i in 0..len {
        let limb_result = DoubleLimb::from(xs[i]) + DoubleLimb::from(ys[i]) * limb_double + carry;
        xs[i] = limb_result.lower_half();
        carry = limb_result >> Limb::WIDTH;
    }
    Limb::exact_from(carry)
}

/// Given the limbs of two `Natural`s a and b, and a limb c, computes a + b * c. The lowest limbs of
/// the result are written to `ys` and the highest limb is returned. `xs` must have the same length
/// as `ys`.
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
/// assert_eq!(limbs_slice_add_mul_limb_same_length_in_place_right(&[123, 456], ys, 0xffff_ffff),
///         123);
/// assert_eq!(ys, &[123, 333]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive and have the same
/// lengths, sub is positive, the lowest limbs of the result are written to the second input rather
/// than the first, and the highest limb is returned.
pub fn limbs_slice_add_mul_limb_same_length_in_place_right(
    xs: &[Limb],
    ys: &mut [Limb],
    limb: Limb,
) -> Limb {
    let xs_len = xs.len();
    assert_eq!(ys.len(), xs_len);
    let mut carry = 0;
    let limb_double = DoubleLimb::from(limb);
    for i in 0..xs_len {
        let limb_result = DoubleLimb::from(xs[i]) + DoubleLimb::from(ys[i]) * limb_double + carry;
        ys[i] = limb_result.lower_half();
        carry = limb_result >> Limb::WIDTH;
    }
    Limb::exact_from(carry)
}

/// Given the limbs of two `Natural`s a and b, and a limb c, writes the limbs of a + b * c to the
/// first (left) input, corresponding to the limbs of a. `xs` and `ys` should be nonempty and have
/// no trailing zeros, and `limb` should be nonzero. The result will have no trailing zeros.
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
/// limbs_vec_add_mul_limb_in_place_left(&mut xs, &[123], 0xffff_ffff);
/// assert_eq!(xs, &[0, 579]);
///
/// let mut xs = vec![123];
/// limbs_vec_add_mul_limb_in_place_left(&mut xs, &[0, 123], 4);
/// assert_eq!(xs, &[123, 492]);
///
/// let mut xs = vec![123, 456];
/// limbs_vec_add_mul_limb_in_place_left(&mut xs, &[0, 123], 0xffff_ffff);
/// assert_eq!(xs, &[123, 333, 123]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive and sub is positive.
pub fn limbs_vec_add_mul_limb_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], limb: Limb) {
    let xs_len = xs.len();
    if xs_len >= ys.len() {
        limbs_vec_add_mul_limb_greater_in_place_left(xs, ys, limb);
    } else {
        let (ys_lo, ys_hi) = ys.split_at(xs_len);
        xs.resize(ys.len(), 0);
        let mut carry;
        {
            let (xs_lo, xs_hi) = xs.split_at_mut(xs_len);
            carry = limbs_mul_limb_to_out(xs_hi, ys_hi, limb);
            let inner_carry =
                limbs_slice_add_mul_limb_same_length_in_place_left(xs_lo, ys_lo, limb);
            if inner_carry != 0 && limbs_slice_add_limb_in_place(xs_hi, inner_carry) {
                carry += 1;
            }
        }
        if carry != 0 {
            xs.push(carry);
        }
    }
}

// ys.len() > 0, xs.len() >= ys.len(), limb != 0
fn limbs_vec_add_mul_limb_greater_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], limb: Limb) {
    let ys_len = ys.len();
    let carry = limbs_slice_add_mul_limb_same_length_in_place_left(&mut xs[..ys_len], ys, limb);
    if carry != 0 {
        if xs.len() == ys_len {
            xs.push(carry);
        } else if limbs_slice_add_limb_in_place(&mut xs[ys_len..], carry) {
            xs.push(1);
        }
    }
}

/// Given the limbs of two `Natural`s a and b, and a limb c, writes the limbs of a + b * c to the
/// second (right) input, corresponding to the limbs of b. `xs` and `ys` should be nonempty and have
/// no trailing zeros, and `limb` should be nonzero. The result will have no trailing zeros.
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
/// limbs_vec_add_mul_limb_in_place_right(&[123, 456], &mut ys, 0xffff_ffff);
/// assert_eq!(ys, &[0, 579]);
///
/// let mut ys = vec![0, 123];
/// limbs_vec_add_mul_limb_in_place_right(&[123], &mut ys, 4);
/// assert_eq!(ys, &[123, 492]);
///
/// let mut ys = vec![0, 123];
/// limbs_vec_add_mul_limb_in_place_right(&[123, 456], &mut ys, 0xffff_ffff);
/// assert_eq!(ys, &[123, 333, 123]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive, sub is positive, and the
/// result is written to the second input rather than the first.
pub fn limbs_vec_add_mul_limb_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>, limb: Limb) {
    let ys_len = ys.len();
    if xs.len() >= ys_len {
        let carry = limbs_slice_add_mul_limb_same_length_in_place_right(&xs[..ys_len], ys, limb);
        ys.extend_from_slice(&xs[ys_len..]);
        if carry != 0 {
            if xs.len() == ys_len {
                ys.push(carry);
            } else if limbs_slice_add_limb_in_place(&mut ys[ys_len..], carry) {
                ys.push(1);
            }
        }
    } else {
        limbs_vec_add_mul_limb_smaller_in_place_right(xs, ys, limb);
    }
}

// xs.len() > 0, xs.len() < ys.len(), limb != 0
fn limbs_vec_add_mul_limb_smaller_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>, limb: Limb) {
    let mut carry;
    {
        let (ys_lo, ys_hi) = ys.split_at_mut(xs.len());
        carry = limbs_slice_mul_limb_in_place(ys_hi, limb);
        let inner_carry = limbs_slice_add_mul_limb_same_length_in_place_right(xs, ys_lo, limb);
        if inner_carry != 0 && limbs_slice_add_limb_in_place(ys_hi, inner_carry) {
            carry += 1;
        }
    }
    if carry != 0 {
        ys.push(carry);
    }
}

/// Given the limbs of two `Natural`s a and b, and a limb c, writes the limbs of a + b * c to
/// whichever input is longer. If the result is written to the first input, `false` is returned; if
/// to the second, `true` is returned. `xs` and `ys` should be nonempty and have no trailing zeros,
/// and `limb` should be nonzero. The result will have no trailing zeros.
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
/// assert_eq!(limbs_vec_add_mul_limb_in_place_either(&mut xs, &mut ys, 0xffff_ffff), false);
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
/// assert_eq!(limbs_vec_add_mul_limb_in_place_either(&mut xs, &mut ys, 0xffff_ffff), false);
/// assert_eq!(xs, &[123, 333, 123]);
/// assert_eq!(ys, &[0, 123]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive, sub is positive, and the
/// result is written to the longer input.
pub fn limbs_vec_add_mul_limb_in_place_either(
    xs: &mut Vec<Limb>,
    ys: &mut Vec<Limb>,
    limb: Limb,
) -> bool {
    if xs.len() >= ys.len() {
        limbs_vec_add_mul_limb_greater_in_place_left(xs, ys, limb);
        false
    } else {
        limbs_vec_add_mul_limb_smaller_in_place_right(xs, ys, limb);
        true
    }
}

/// Given the limbs `xs`, `ys` and `zs` of three `Natural`s a, b, and c, returns the limbs of
/// a + b * c. `xs` should be nonempty and `ys` and `zs` should have length at least 2. None of the
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
/// This is mpz_aorsmul from mpz/aorsmul.c, where w, x, and y are positive, sub is positive, and w
/// is returned instead of overwriting the first input.
pub fn limbs_add_mul(xs: &[Limb], ys: &[Limb], zs: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let mut product_size = ys.len() + zs.len();
    let mut product = vec![0; product_size];
    if limbs_mul_to_out(&mut product, ys, zs) == 0 {
        product_size -= 1;
        product.pop();
    }
    assert_ne!(*product.last().unwrap(), 0);
    if xs_len >= product_size {
        limbs_add_greater(xs, &product)
    } else {
        if limbs_slice_add_greater_in_place_left(&mut product, xs) {
            product.push(1);
        }
        product
    }
}

/// Given the limbs `xs`, `ys` and `zs` of three `Natural`s a, b, and c, computes a + b * c. The
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
/// This is mpz_aorsmul from mpz/aorsmul.c, where w, x, and y are positive and sub is positive.
pub fn limbs_add_mul_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], zs: &[Limb]) {
    let xs_len = xs.len();
    let mut product_size = ys.len() + zs.len();
    let mut product = vec![0; product_size];
    if limbs_mul_to_out(&mut product, ys, zs) == 0 {
        product_size -= 1;
        product.pop();
    }
    assert_ne!(*product.last().unwrap(), 0);
    if xs_len < product_size {
        swap(xs, &mut product);
    }
    if limbs_slice_add_greater_in_place_left(xs, &product) {
        xs.push(1);
    }
}

impl Natural {
    fn add_mul_limb_ref_ref(&self, b: &Natural, c: Limb) -> Natural {
        if c == 0 || *b == 0 {
            return self.clone();
        }
        if c == 1 {
            return self + b;
        }
        match (self, b) {
            (Natural(Large(ref a_limbs)), Natural(Large(ref b_limbs))) => {
                Natural(Large(limbs_add_mul_limb(a_limbs, b_limbs, c)))
            }
            _ => self + b * Natural::from(c),
        }
    }

    fn add_mul_assign_limb(&mut self, mut b: Natural, c: Limb) {
        if c == 0 || b == 0 {
            return;
        }
        if c == 1 {
            *self += b;
            return;
        }
        let (fallback, right) = match (&mut *self, &mut b) {
            (&mut Natural(Large(ref mut a_limbs)), &mut Natural(Large(ref mut b_limbs))) => (
                false,
                limbs_vec_add_mul_limb_in_place_either(a_limbs, b_limbs, c),
            ),
            _ => (true, false),
        };
        if fallback {
            *self += b * Natural::from(c);
        } else if right {
            *self = b;
        }
    }

    fn add_mul_assign_limb_ref(&mut self, b: &Natural, c: Limb) {
        if c == 0 || *b == 0 {
            return;
        }
        if c == 1 {
            *self += b;
            return;
        }
        let fallback = match (&mut *self, b) {
            (&mut Natural(Large(ref mut a_limbs)), &Natural(Large(ref b_limbs))) => {
                limbs_vec_add_mul_limb_in_place_left(a_limbs, b_limbs, c);
                false
            }
            _ => true,
        };
        if fallback {
            *self += b * Natural::from(c);
        }
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), taking `self`, b,
/// and c by value.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
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
impl<'a> AddMul<Natural, Natural> for Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(mut self, b: Natural, c: Natural) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), taking `self` and
/// b by value and c by reference.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
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
impl<'a> AddMul<Natural, &'a Natural> for Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(mut self, b: Natural, c: &'a Natural) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), taking `self` and
/// c by value and b by reference.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
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
impl<'a> AddMul<&'a Natural, Natural> for Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(mut self, b: &'a Natural, c: Natural) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), taking `self` by
/// value and b and c by reference.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
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
impl<'a, 'b> AddMul<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    #[inline]
    fn add_mul(mut self, b: &'a Natural, c: &'b Natural) -> Natural {
        self.add_mul_assign(b, c);
        self
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), taking `self`, b,
/// and c by reference.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(m + n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
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
impl<'a, 'b, 'c> AddMul<&'a Natural, &'b Natural> for &'c Natural {
    type Output = Natural;

    fn add_mul(self, b: &'a Natural, c: &'b Natural) -> Natural {
        if let Natural(Small(small_a)) = *self {
            (b * c).add_limb(small_a)
        } else if let Natural(Small(small_b)) = *b {
            self.add_mul_limb_ref_ref(c, small_b)
        } else if let Natural(Small(small_c)) = *c {
            self.add_mul_limb_ref_ref(b, small_c)
        } else {
            if let Natural(Large(ref a_limbs)) = *self {
                if let Natural(Large(ref b_limbs)) = *b {
                    if let Natural(Large(ref c_limbs)) = *c {
                        return Natural(Large(limbs_add_mul(a_limbs, b_limbs, c_limbs)));
                    }
                }
            }
            unreachable!();
        }
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), in place, taking
/// b and c by value.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
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
impl AddMulAssign<Natural, Natural> for Natural {
    fn add_mul_assign(&mut self, b: Natural, c: Natural) {
        if let Natural(Small(small_b)) = b {
            self.add_mul_assign_limb(c, small_b);
        } else if let Natural(Small(small_c)) = c {
            self.add_mul_assign_limb(b, small_c);
        } else if *self == 0 {
            *self = b * c;
        } else {
            let self_limbs = self.promote_in_place();
            if let Natural(Large(ref b_limbs)) = b {
                if let Natural(Large(ref c_limbs)) = c {
                    limbs_add_mul_in_place_left(self_limbs, b_limbs, c_limbs);
                }
            }
        }
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), in place, taking
/// b by value and c by reference.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
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
impl<'a> AddMulAssign<Natural, &'a Natural> for Natural {
    fn add_mul_assign(&mut self, b: Natural, c: &'a Natural) {
        if let Natural(Small(small_b)) = b {
            self.add_mul_assign_limb_ref(c, small_b);
        } else if let Natural(Small(small_c)) = *c {
            self.add_mul_assign_limb(b, small_c);
        } else if *self == 0 {
            *self = b * c;
        } else {
            let self_limbs = self.promote_in_place();
            if let Natural(Large(ref b_limbs)) = b {
                if let Natural(Large(ref c_limbs)) = *c {
                    limbs_add_mul_in_place_left(self_limbs, b_limbs, c_limbs);
                }
            }
        }
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), in place, taking
/// b by reference and c by value.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
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
impl<'a> AddMulAssign<&'a Natural, Natural> for Natural {
    fn add_mul_assign(&mut self, b: &'a Natural, c: Natural) {
        if let Natural(Small(small_b)) = *b {
            self.add_mul_assign_limb(c, small_b);
        } else if let Natural(Small(small_c)) = c {
            self.add_mul_assign_limb_ref(b, small_c);
        } else if *self == 0 {
            *self = b * c;
        } else {
            let self_limbs = self.promote_in_place();
            if let Natural(Large(ref b_limbs)) = *b {
                if let Natural(Large(ref c_limbs)) = c {
                    limbs_add_mul_in_place_left(self_limbs, b_limbs, c_limbs);
                }
            }
        }
    }
}

/// Adds the product of a `Natural` (b) and a `Natural` (c) to a `Natural` (self), in place, taking
/// b and c by reference.
///
/// Time: O(m + n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`b.significant_bits()`, `c.significant_bits()`)
///       m = `a.significant_bits()`
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
impl<'a, 'b> AddMulAssign<&'a Natural, &'b Natural> for Natural {
    fn add_mul_assign(&mut self, b: &'a Natural, c: &'b Natural) {
        if let Natural(Small(small_b)) = *b {
            self.add_mul_assign_limb_ref(c, small_b);
        } else if let Natural(Small(small_c)) = *c {
            self.add_mul_assign_limb_ref(b, small_c);
        } else if *self == 0 {
            *self = b * c;
        } else {
            let self_limbs = self.promote_in_place();
            if let Natural(Large(ref b_limbs)) = *b {
                if let Natural(Large(ref c_limbs)) = *c {
                    limbs_add_mul_in_place_left(self_limbs, b_limbs, c_limbs);
                }
            }
        }
    }
}

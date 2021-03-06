use malachite_base::num::arithmetic::traits::{
    CheckedSubMul, SubMul, SubMulAssign, WrappingAddAssign,
};
use malachite_base::num::basic::traits::Iverson;
use malachite_base::num::conversion::traits::SplitInHalf;
use natural::arithmetic::mul::limbs_mul;
use natural::arithmetic::sub::{limbs_sub_in_place_left, limbs_sub_limb_in_place};
use natural::comparison::ord::limbs_cmp;
use natural::Natural;
use platform::{DoubleLimb, Limb};
use std::cmp::Ordering;
use std::fmt::Display;

/// Given the limbs of two `Natural`s x and y, and a limb z, returns the limbs of x - y * z. If
/// y * z > x, `None` is returned.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is shorter than `ys`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub_mul::limbs_sub_mul_limb_greater;
///
/// assert_eq!(limbs_sub_mul_limb_greater(&[123, 456], &[123], 4), Some(vec![4294966927, 455]));
/// assert_eq!(limbs_sub_mul_limb_greater(&[123, 456], &[123], u32::MAX), Some(vec![246, 333]));
/// assert_eq!(limbs_sub_mul_limb_greater(&[123, 456], &[0, 123], u32::MAX), None);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, GMP 6.2.1, where w and x are positive, sub is
/// negative, and w is returned instead of overwriting the first input.
pub fn limbs_sub_mul_limb_greater(xs: &[Limb], ys: &[Limb], z: Limb) -> Option<Vec<Limb>> {
    let ys_len = ys.len();
    let mut result = xs.to_vec();
    let borrow = limbs_sub_mul_limb_same_length_in_place_left(&mut result[..ys_len], ys, z);
    if borrow == 0 {
        Some(result)
    } else if xs.len() == ys_len || limbs_sub_limb_in_place(&mut result[ys_len..], borrow) {
        None
    } else {
        Some(result)
    }
}

/// Given the equal-length limbs of two `Natural`s x and y, and a limb z, calculates x - y * z and
/// writes the limbs of the result to the first (left) input slice. If y * z > x, a nonzero borrow
/// is returned.
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
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub_mul::*;
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_mul_limb_same_length_in_place_left(xs, &[123, 0], 4), 0);
/// assert_eq!(xs, &[4294966927, 455]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_mul_limb_same_length_in_place_left(xs, &[123, 0], u32::MAX), 0);
/// assert_eq!(xs, &[246, 333]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_mul_limb_same_length_in_place_left(xs, &[0, 123], u32::MAX), 123);
/// assert_eq!(xs, &[123, 579]);
/// ```
///
/// This is mpn_submul_1 from mpn/generic/submul_1.c, GMP 6.1.2.
pub fn limbs_sub_mul_limb_same_length_in_place_left(xs: &mut [Limb], ys: &[Limb], z: Limb) -> Limb {
    assert_eq!(xs.len(), ys.len());
    let mut borrow = 0;
    let z = DoubleLimb::from(z);
    for (x, &y) in xs.iter_mut().zip(ys.iter()) {
        let (upper, mut lower) = (DoubleLimb::from(y) * z).split_in_half();
        lower.wrapping_add_assign(borrow);
        if lower < borrow {
            borrow = upper.wrapping_add(1);
        } else {
            borrow = upper;
        }
        lower = x.wrapping_sub(lower);
        if lower > *x {
            borrow.wrapping_add_assign(1);
        }
        *x = lower;
    }
    borrow
}

/// Given the limbs of two `Natural`s x and y, and a limb z, calculates x - y * z and writes the
/// limbs of the result to the first (left) input slice. If y * z > x, a nonzero borrow is returned.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `xs.len()`
///
/// # Panics
/// Panics if `xs` is shorter than `ys`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub_mul::limbs_sub_mul_limb_greater_in_place_left;
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_mul_limb_greater_in_place_left(xs, &[123], 4), 0);
/// assert_eq!(xs, &[4294966927, 455]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_mul_limb_greater_in_place_left(xs, &[123], u32::MAX), 0);
/// assert_eq!(xs, &[246, 333]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_mul_limb_greater_in_place_left(xs, &[0, 123], u32::MAX), 123);
/// assert_eq!(xs, &[123, 579]);
/// ```
///
/// This is mpn_submul_1 from mpn/generic/submul_1.c, GMP 6.1.2, but where the first input may be
/// longer than the second.
pub fn limbs_sub_mul_limb_greater_in_place_left(xs: &mut [Limb], ys: &[Limb], limb: Limb) -> Limb {
    let (xs_lo, xs_hi) = xs.split_at_mut(ys.len());
    let borrow = limbs_sub_mul_limb_same_length_in_place_left(xs_lo, ys, limb);
    if borrow == 0 || xs_hi.is_empty() {
        borrow
    } else {
        Limb::iverson(limbs_sub_limb_in_place(xs_hi, borrow))
    }
}

/// Given the equal-length limbs of two `Natural`s x and y, and a limb z, calculates x - y * z and
/// writes the limbs of the result to the second (right) input slice. If y * z > x, a nonzero borrow
/// is returned.
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
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub_mul::*;
///
/// let ys = &mut [123, 0];
/// assert_eq!(limbs_sub_mul_limb_same_length_in_place_right(&[123, 456], ys, 4), 0);
/// assert_eq!(ys, &[4294966927, 455]);
///
/// let ys = &mut [123, 0];
/// assert_eq!(limbs_sub_mul_limb_same_length_in_place_right(&[123, 456], ys, u32::MAX), 0);
/// assert_eq!(ys, &[246, 333]);
///
/// let ys = &mut [0, 123];
/// assert_eq!(limbs_sub_mul_limb_same_length_in_place_right(&[123, 456], ys, u32::MAX), 123);
/// assert_eq!(ys, &[123, 579]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, GMP 6.2.1, where w and x are positive and have the
/// same lengths, sub is negative, and the lowest limbs of the result are written to the second
/// input rather than the first.
pub fn limbs_sub_mul_limb_same_length_in_place_right(
    xs: &[Limb],
    ys: &mut [Limb],
    z: Limb,
) -> Limb {
    assert_eq!(xs.len(), ys.len());
    let mut borrow = 0;
    let z = DoubleLimb::from(z);
    for (&x, y) in xs.iter().zip(ys.iter_mut()) {
        let (upper, mut lower) = (DoubleLimb::from(*y) * z).split_in_half();
        lower.wrapping_add_assign(borrow);
        if lower < borrow {
            borrow = upper.wrapping_add(1);
        } else {
            borrow = upper;
        }
        lower = x.wrapping_sub(lower);
        if lower > x {
            borrow.wrapping_add_assign(1);
        }
        *y = lower;
    }
    borrow
}

/// Given the limbs of two `Natural`s x and y, and a limb z, calculates x - y * z and writes the
/// limbs of the result to the second (right) input `Vec`. If y * z > x, a nonzero borrow is
/// returned.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = `xs.len()`
///       m = `xs.len() - ys.len()`
///
/// # Panics
/// Panics if `xs` is shorter than `ys`.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub_mul::limbs_sub_mul_limb_greater_in_place_right;
///
/// let mut ys = vec![123];
/// assert_eq!(limbs_sub_mul_limb_greater_in_place_right(&[123, 456], &mut ys, 4), 0);
/// assert_eq!(ys, &[4294966927, 455]);
///
/// let mut ys = vec![123];
/// assert_eq!(limbs_sub_mul_limb_greater_in_place_right(&[123, 456], &mut ys, u32::MAX), 0);
/// assert_eq!(ys, &[246, 333]);
///
/// let mut ys = vec![0, 123];
/// assert_eq!(limbs_sub_mul_limb_greater_in_place_right(&[123, 456], &mut ys, u32::MAX), 123);
/// assert_eq!(ys, &[123, 579]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, GMP 6.2.1, where w and x are positive, sub is
/// negative, and the result is written to the second input rather than the first.
pub fn limbs_sub_mul_limb_greater_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>, z: Limb) -> Limb {
    let ys_len = ys.len();
    let (xs_lo, xs_hi) = xs.split_at(ys_len);
    let borrow = limbs_sub_mul_limb_same_length_in_place_right(xs_lo, ys, z);
    if xs_hi.is_empty() {
        borrow
    } else {
        ys.extend(&xs[ys_len..]);
        if borrow == 0 {
            0
        } else if limbs_sub_limb_in_place(&mut ys[ys_len..], borrow) {
            1
        } else {
            0
        }
    }
}

/// Given the limbs `xs`, `ys` and `zs` of three `Natural`s x, y, and z, returns the limbs of
/// x - y * z. If x < y * z, `None` is returned. `ys` and `zs` should have length at least 2, and
/// the length of `xs` should be at least `ys.len()` + `zs.len()` - 1 (if the latter condition is
/// false, the result would be `None` and there's no point in calling this function). None of the
/// slices should have any trailing zeros. The result, if it exists, will have no trailing zeros.
///
/// Time: O(n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`ys.len()`, `zs.len()`)
///
/// # Panics
/// Panics if `ys` or `zs` have fewer than two elements each, or if `xs.len()` < `ys.len()` +
/// `zs.len()` - 1.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub_mul::limbs_sub_mul;
///
/// assert_eq!(limbs_sub_mul(&[123, 456, 789], &[123, 789], &[321, 654]), None);
/// assert_eq!(limbs_sub_mul(&[123, 456, 789, 1], &[123, 789], &[321, 654]),
///         Some(vec![4294927936, 4294634040, 4294452078, 0]));
/// ```
///
/// This is mpz_aorsmul from mpz/aorsmul.c, GMP 6.2.1, where w, x, and y are positive, sub is
/// negative, negative results are converted to `None`, and w is returned instead of overwriting the
/// first input.
pub fn limbs_sub_mul(xs: &[Limb], ys: &[Limb], zs: &[Limb]) -> Option<Vec<Limb>> {
    let mut xs = xs.to_vec();
    if limbs_sub_mul_in_place_left(&mut xs, ys, zs) {
        None
    } else {
        Some(xs)
    }
}

/// Given the limbs `xs`, `ys` and `zs` of three `Natural`s x, y, and z, computes x - y * z. The
/// limbs of the result are written to `xs`. Returns whether a borrow (overflow) occurred: if
/// x < y * z, `true` is returned and the value of `xs` should be ignored. `ys` and `zs` should have
/// length at least 2, and the length of `xs` should be at least `ys.len()` + `zs.len()` - 1 (if the
/// latter condition is false, the result would be negative and there would be no point in calling
/// this function). None of the slices should have any trailing zeros. The result, if it exists,
/// will have no trailing zeros.
///
/// Time: O(n * log(n) * log(log(n)))
///
/// Additional memory: O(n * log(n))
///
/// where n = max(`ys.len()`, `zs.len()`)
///
/// # Panics
/// Panics if `ys` or `zs` have fewer than two elements each, or if `xs.len()` < `ys.len()` +
/// `zs.len()` - 1.
///
/// # Examples
/// ```
/// use malachite_nz::natural::arithmetic::sub_mul::limbs_sub_mul_in_place_left;
///
/// let mut xs = vec![123, 456, 789];
/// assert_eq!(limbs_sub_mul_in_place_left(&mut xs, &[123, 789], &[321, 654]), true);
///
/// let mut xs = vec![123, 456, 789, 1];
/// assert_eq!(limbs_sub_mul_in_place_left(&mut xs, &[123, 789], &[321, 654]), false);
/// assert_eq!(xs, &[4294927936, 4294634040, 4294452078, 0]);
/// ```
///
/// This is mpz_aorsmul from mpz/aorsmul.c, GMP 6.2.1, where w, x, and y are positive, sub is
/// negative and negative results are discarded.
pub fn limbs_sub_mul_in_place_left(xs: &mut [Limb], ys: &[Limb], zs: &[Limb]) -> bool {
    assert!(ys.len() > 1);
    assert!(zs.len() > 1);
    let mut scratch = limbs_mul(ys, zs);
    assert!(xs.len() >= scratch.len() - 1);
    if *scratch.last().unwrap() == 0 {
        scratch.pop();
    }
    let borrow = limbs_cmp(xs, &scratch) == Ordering::Less;
    if !borrow {
        assert!(!limbs_sub_in_place_left(xs, &scratch));
    }
    borrow
}

fn sub_mul_panic<S: Display, T: Display, U: Display>(a: S, b: T, c: U) -> ! {
    panic!("Cannot perform sub_mul. a: {}, b: {}, c: {}", a, b, c);
}

impl SubMul<Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (`self`),
    /// taking `self`, y, and z by value.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(20u32).sub_mul(Natural::from(3u32), Natural::from(4u32)).to_string(),
    ///     "8"
    /// );
    /// assert_eq!(Natural::trillion().sub_mul(
    ///     Natural::from(0x10000u32), Natural::from(0x10000u32)).to_string(),
    ///     "995705032704"
    /// );
    /// ```
    fn sub_mul(self, y: Natural, z: Natural) -> Natural {
        self.checked_sub_mul(y, z)
            .expect("Natural sub_mul_assign cannot have a negative result")
    }
}

impl<'a> SubMul<Natural, &'a Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (`self`),
    /// taking `self` and y by value and z by reference.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(20u32).sub_mul(Natural::from(3u32), &Natural::from(4u32)).to_string(),
    ///     "8"
    /// );
    /// assert_eq!(
    ///     Natural::trillion().sub_mul(Natural::from(0x10000u32), &Natural::from(0x10000u32))
    ///     .to_string(),
    ///     "995705032704"
    /// );
    /// ```
    fn sub_mul(self, y: Natural, z: &'a Natural) -> Natural {
        self.checked_sub_mul(y, z)
            .expect("Natural sub_mul_assign cannot have a negative result")
    }
}

impl<'a> SubMul<&'a Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (`self`),
    /// taking `self` and z value and y by reference.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(20u32).sub_mul(&Natural::from(3u32), Natural::from(4u32)).to_string(),
    ///     "8"
    /// );
    /// assert_eq!(Natural::trillion().sub_mul(
    ///     &Natural::from(0x10000u32), Natural::from(0x10000u32)).to_string(),
    ///     "995705032704"
    /// );
    /// ```
    fn sub_mul(self, y: &'a Natural, z: Natural) -> Natural {
        self.checked_sub_mul(y, z)
            .expect("Natural sub_mul_assign cannot have a negative result")
    }
}

impl<'a, 'b> SubMul<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (`self`),
    /// taking `self` by value and y and z by reference.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(20u32).sub_mul(&Natural::from(3u32), &Natural::from(4u32)).to_string(),
    ///     "8"
    /// );
    /// assert_eq!(
    ///     Natural::trillion().sub_mul(&Natural::from(0x10000u32), &Natural::from(0x10000u32))
    ///     .to_string(),
    ///     "995705032704"
    /// );
    /// ```
    fn sub_mul(self, y: &'a Natural, z: &'b Natural) -> Natural {
        self.checked_sub_mul(y, z)
            .expect("Natural sub_mul_assign cannot have a negative result")
    }
}

impl<'a, 'b, 'c> SubMul<&'a Natural, &'b Natural> for &'c Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (`self`),
    /// taking `self`, y, and z by reference.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(20u32)).sub_mul(&Natural::from(3u32), &Natural::from(4u32)).to_string(),
    ///     "8"
    /// );
    /// assert_eq!(
    ///     (&Natural::trillion()).sub_mul(&Natural::from(0x10000u32), &Natural::from(0x10000u32))
    ///         .to_string(),
    ///     "995705032704"
    /// );
    /// ```
    fn sub_mul(self, y: &'a Natural, z: &'b Natural) -> Natural {
        self.checked_sub_mul(y, z).unwrap_or_else(|| {
            sub_mul_panic(self, y, z);
        })
    }
}

impl SubMulAssign<Natural, Natural> for Natural {
    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (`self`), in
    /// place, taking y and z by value.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(20u32);
    /// x.sub_mul_assign(Natural::from(3u32), Natural::from(4u32));
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::trillion();
    /// x.sub_mul_assign(Natural::from(0x10000u32), Natural::from(0x10000u32));
    /// assert_eq!(x.to_string(), "995705032704");
    /// ```
    fn sub_mul_assign(&mut self, y: Natural, z: Natural) {
        if self.sub_mul_assign_no_panic(y, z) {
            panic!("Natural sub_mul_assign cannot have a negative result");
        }
    }
}

impl<'a> SubMulAssign<Natural, &'a Natural> for Natural {
    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (`self`), in
    /// place, taking y by value and z by reference.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(20u32);
    /// x.sub_mul_assign(Natural::from(3u32), &Natural::from(4u32));
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::trillion();
    /// x.sub_mul_assign(Natural::from(0x10000u32), &Natural::from(0x10000u32));
    /// assert_eq!(x.to_string(), "995705032704");
    /// ```
    fn sub_mul_assign(&mut self, y: Natural, z: &'a Natural) {
        if self.sub_mul_assign_val_ref_no_panic(y, z) {
            panic!("Natural sub_mul_assign cannot have a negative result");
        }
    }
}

impl<'a> SubMulAssign<&'a Natural, Natural> for Natural {
    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (`self`), in
    /// place, taking y by reference and z by value.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(20u32);
    /// x.sub_mul_assign(&Natural::from(3u32), Natural::from(4u32));
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::trillion();
    /// x.sub_mul_assign(&Natural::from(0x10000u32), Natural::from(0x10000u32));
    /// assert_eq!(x.to_string(), "995705032704");
    /// ```
    fn sub_mul_assign(&mut self, y: &'a Natural, z: Natural) {
        if self.sub_mul_assign_ref_val_no_panic(y, z) {
            panic!("Natural sub_mul_assign cannot have a negative result");
        }
    }
}

impl<'a, 'b> SubMulAssign<&'a Natural, &'b Natural> for Natural {
    /// Subtracts the product of a `Natural` (y) and a `Natural` (z) from a `Natural` (`self`), in
    /// place, taking y and z by reference.
    ///
    /// Time: O(n * log(n) * log(log(n)))
    ///
    /// Additional memory: O(n * log(n))
    ///
    /// where n = max(`y.significant_bits()`, `z.significant_bits()`)
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(20u32);
    /// x.sub_mul_assign(&Natural::from(3u32), &Natural::from(4u32));
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::trillion();
    /// x.sub_mul_assign(&Natural::from(0x10000u32), &Natural::from(0x10000u32));
    /// assert_eq!(x.to_string(), "995705032704");
    /// ```
    fn sub_mul_assign(&mut self, y: &'a Natural, z: &'b Natural) {
        if self.sub_mul_assign_ref_ref_no_panic(y, z) {
            panic!("Natural sub_mul_assign cannot have a negative result");
        }
    }
}

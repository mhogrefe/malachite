use malachite_base::num::traits::{
    CheckedSubMul, SplitInHalf, SubMul, SubMulAssign, WrappingAddAssign,
};

use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use natural::arithmetic::sub_mul::sub_mul_panic;
use natural::Natural;
use platform::{DoubleLimb, Limb};

/// Given the limbs of two `Natural`s a and b, and a limb c, returns the limbs of a - b * c. If
/// b * c > a, `None` is returned.
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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub_mul_limb::limbs_sub_mul_limb_greater;
///
/// assert_eq!(limbs_sub_mul_limb_greater(&[123, 456], &[123], 4), Some(vec![4294966927, 455]));
/// assert_eq!(limbs_sub_mul_limb_greater(&[123, 456], &[123], 0xffff_ffff), Some(vec![246, 333]));
/// assert_eq!(limbs_sub_mul_limb_greater(&[123, 456], &[0, 123], 0xffff_ffff), None);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive, sub is negative, and w
/// is returned instead of overwriting the first input.
pub fn limbs_sub_mul_limb_greater(xs: &[Limb], ys: &[Limb], limb: Limb) -> Option<Vec<Limb>> {
    let ys_len = ys.len();
    let mut result = xs.to_vec();
    let borrow = limbs_sub_mul_limb_same_length_in_place_left(&mut result[..ys_len], ys, limb);
    if borrow == 0 {
        Some(result)
    } else if xs.len() == ys_len {
        None
    } else if limbs_sub_limb_in_place(&mut result[ys_len..], borrow) {
        None
    } else {
        Some(result)
    }
}

/// Given the equal-length limbs of two `Natural`s a and b, and a limb c, calculates a - b * c and
/// writes the limbs of the result to the first (left) input slice. If b * c > a, a nonzero borrow
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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub_mul_limb::*;
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_mul_limb_same_length_in_place_left(xs, &[123, 0], 4), 0);
/// assert_eq!(xs, &[4294966927, 455]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_mul_limb_same_length_in_place_left(xs, &[123, 0], 0xffff_ffff), 0);
/// assert_eq!(xs, &[246, 333]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_mul_limb_same_length_in_place_left(xs, &[0, 123], 0xffff_ffff), 123);
/// assert_eq!(xs, &[123, 579]);
/// ```
///
/// This is mpn_submul_1 from mpn/generic/submul_1.c.
pub fn limbs_sub_mul_limb_same_length_in_place_left(
    xs: &mut [Limb],
    ys: &[Limb],
    limb: Limb,
) -> Limb {
    assert_eq!(xs.len(), ys.len());
    let mut borrow = 0;
    let double_limb = DoubleLimb::from(limb);
    for (x, &y) in xs.iter_mut().zip(ys.iter()) {
        let product = DoubleLimb::from(y) * double_limb;
        let (upper, mut lower) = product.split_in_half();
        lower.wrapping_add_assign(borrow);
        if lower < borrow {
            borrow = upper.wrapping_add(1);
        } else {
            borrow = upper;
        }
        let limb = *x;
        lower = limb.wrapping_sub(lower);
        if lower > limb {
            borrow.wrapping_add_assign(1);
        }
        *x = lower;
    }
    borrow
}

/// Given the limbs of two `Natural`s a and b, and a limb c, calculates a - b * c and writes the
/// limbs of the result to the first (left) input slice. If b * c > a, a nonzero borrow is returned.
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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub_mul_limb::limbs_sub_mul_limb_greater_in_place_left;
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_mul_limb_greater_in_place_left(xs, &[123], 4), 0);
/// assert_eq!(xs, &[4294966927, 455]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_mul_limb_greater_in_place_left(xs, &[123], 0xffff_ffff), 0);
/// assert_eq!(xs, &[246, 333]);
///
/// let xs = &mut [123, 456];
/// assert_eq!(limbs_sub_mul_limb_greater_in_place_left(xs, &[0, 123], 0xffff_ffff), 123);
/// assert_eq!(xs, &[123, 579]);
/// ```
///
/// This is mpn_submul_1 from mpn/generic/submul_1.c, but where the first input may be longer than
/// the second.
pub fn limbs_sub_mul_limb_greater_in_place_left(xs: &mut [Limb], ys: &[Limb], limb: Limb) -> Limb {
    let (xs_lo, xs_hi) = xs.split_at_mut(ys.len());
    let borrow = limbs_sub_mul_limb_same_length_in_place_left(xs_lo, ys, limb);
    if borrow == 0 || xs_hi.is_empty() {
        borrow
    } else if limbs_sub_limb_in_place(xs_hi, borrow) {
        1
    } else {
        0
    }
}

/// Given the equal-length limbs of two `Natural`s a and b, and a limb c, calculates a - b * c and
/// writes the limbs of the result to the second (right) input slice. If b * c > a, a nonzero borrow
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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub_mul_limb::*;
///
/// let ys = &mut [123, 0];
/// assert_eq!(limbs_sub_mul_limb_same_length_in_place_right(&[123, 456], ys, 4), 0);
/// assert_eq!(ys, &[4294966927, 455]);
///
/// let ys = &mut [123, 0];
/// assert_eq!(limbs_sub_mul_limb_same_length_in_place_right(&[123, 456], ys, 0xffff_ffff), 0);
/// assert_eq!(ys, &[246, 333]);
///
/// let ys = &mut [0, 123];
/// assert_eq!(limbs_sub_mul_limb_same_length_in_place_right(&[123, 456], ys, 0xffff_ffff), 123);
/// assert_eq!(ys, &[123, 579]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive and have the same
/// lengths, sub is negative, and the lowest limbs of the result are written to the second input
/// rather than the first.
pub fn limbs_sub_mul_limb_same_length_in_place_right(
    xs: &[Limb],
    ys: &mut [Limb],
    limb: Limb,
) -> Limb {
    assert_eq!(xs.len(), ys.len());
    let mut borrow = 0;
    let double_limb = DoubleLimb::from(limb);
    for (&x, y) in xs.iter().zip(ys.iter_mut()) {
        let product = DoubleLimb::from(*y) * double_limb;
        let (upper, mut lower) = product.split_in_half();
        lower.wrapping_add_assign(borrow);
        if lower < borrow {
            borrow = upper.wrapping_add(1);
        } else {
            borrow = upper;
        }
        let limb = x;
        lower = limb.wrapping_sub(lower);
        if lower > limb {
            borrow.wrapping_add_assign(1);
        }
        *y = lower;
    }
    borrow
}

/// Given the limbs of two `Natural`s a and b, and a limb c, calculates a - b * c and writes the
/// limbs of the result to the second (right) input `Vec`. If b * c > a, a nonzero borrow is
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
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::sub_mul_limb::limbs_sub_mul_limb_greater_in_place_right;
///
/// let mut ys = vec![123];
/// assert_eq!(limbs_sub_mul_limb_greater_in_place_right(&[123, 456], &mut ys, 4), 0);
/// assert_eq!(ys, &[4294966927, 455]);
///
/// let mut ys = vec![123];
/// assert_eq!(limbs_sub_mul_limb_greater_in_place_right(&[123, 456], &mut ys, 0xffff_ffff), 0);
/// assert_eq!(ys, &[246, 333]);
///
/// let mut ys = vec![0, 123];
/// assert_eq!(limbs_sub_mul_limb_greater_in_place_right(&[123, 456], &mut ys, 0xffff_ffff), 123);
/// assert_eq!(ys, &[123, 579]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive, sub is negative, and the
/// result is written to the second input rather than the first.
pub fn limbs_sub_mul_limb_greater_in_place_right(
    xs: &[Limb],
    ys: &mut Vec<Limb>,
    limb: Limb,
) -> Limb {
    let ys_len = ys.len();
    let (xs_lo, xs_hi) = xs.split_at(ys_len);
    let borrow = limbs_sub_mul_limb_same_length_in_place_right(xs_lo, ys, limb);
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

impl SubMul<Natural, Limb> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` and b by value.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::from(15u32).sub_mul(Natural::from(3u32), 4).to_string(), "3");
    ///     assert_eq!(Natural::trillion().sub_mul(Natural::from(0x1_0000u32),
    ///         0x1_0000u32).to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: Natural, c: Limb) -> Natural {
        self.checked_sub_mul(b, c).expect("Cannot perform sub_mul")
    }
}

#[cfg(feature = "64_bit_limbs")]
impl SubMul<Natural, u32> for Natural {
    type Output = Natural;

    #[inline]
    fn sub_mul(self, b: Natural, c: u32) -> Natural {
        self.sub_mul(b, Limb::from(c))
    }
}

impl<'a> SubMul<&'a Natural, Limb> for Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` by value and b by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::from(15u32).sub_mul(&Natural::from(3u32), 4).to_string(), "3");
    ///     assert_eq!(Natural::trillion().sub_mul(&Natural::from(0x1_0000u32),
    ///         0x1_0000u32).to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: &'a Natural, c: Limb) -> Natural {
        self.checked_sub_mul(b, c).expect("Cannot perform sub_mul")
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SubMul<&'a Natural, u32> for Natural {
    type Output = Natural;

    #[inline]
    fn sub_mul(self, b: &'a Natural, c: u32) -> Natural {
        self.sub_mul(b, Limb::from(c))
    }
}

impl<'a> SubMul<Natural, Limb> for &'a Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` by reference and b by value.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!((&Natural::from(15u32)).sub_mul(Natural::from(3u32), 4).to_string(), "3");
    ///     assert_eq!((&Natural::trillion()).sub_mul(Natural::from(0x1_0000u32),
    ///         0x1_0000u32).to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: Natural, c: Limb) -> Natural {
        self.checked_sub_mul(b, c).expect("Cannot perform sub_mul")
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SubMul<Natural, u32> for &'a Natural {
    type Output = Natural;

    #[inline]
    fn sub_mul(self, b: Natural, c: u32) -> Natural {
        self.sub_mul(b, Limb::from(c))
    }
}

impl<'a, 'b> SubMul<&'a Natural, Limb> for &'b Natural {
    type Output = Natural;

    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), taking
    /// `self` and b by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::SubMul;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     assert_eq!((&Natural::from(15u32)).sub_mul(&Natural::from(3u32), 4).to_string(), "3");
    ///     assert_eq!((&Natural::trillion()).sub_mul(&Natural::from(0x1_0000u32),
    ///         0x1_0000u32).to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul(self, b: &'a Natural, c: Limb) -> Natural {
        self.checked_sub_mul(b, c).unwrap_or_else(|| {
            sub_mul_panic(self, b, c);
        })
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a, 'b> SubMul<&'a Natural, u32> for &'b Natural {
    type Output = Natural;

    #[inline]
    fn sub_mul(self, b: &'a Natural, c: u32) -> Natural {
        self.sub_mul(b, Limb::from(c))
    }
}

impl SubMulAssign<Natural, Limb> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), in place,
    /// taking b by value.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::SubMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = Natural::from(15u32);
    ///     x.sub_mul_assign(Natural::from(3u32), 4);
    ///     assert_eq!(x, 3);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.sub_mul_assign(Natural::from(0x1_0000u32), 0x1_0000u32);
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul_assign(&mut self, b: Natural, c: Limb) {
        if self.sub_mul_assign_limb_ref_no_panic(&b, c) {
            panic!("Natural sub_mul_assign cannot have a negative result");
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl SubMulAssign<Natural, u32> for Natural {
    #[inline]
    fn sub_mul_assign(&mut self, b: Natural, c: u32) {
        self.sub_mul_assign(b, Limb::from(c));
    }
}

impl<'a> SubMulAssign<&'a Natural, Limb> for Natural {
    /// Subtracts the product of a `Natural` (b) and a `Limb` (c) from a `Natural` (self), in place,
    /// taking b by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Panics
    /// Panics if `b * c` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::traits::SubMulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// fn main() {
    ///     let mut x = Natural::from(15u32);
    ///     x.sub_mul_assign(&Natural::from(3u32), 4);
    ///     assert_eq!(x, 3);
    ///
    ///     let mut x = Natural::trillion();
    ///     x.sub_mul_assign(&Natural::from(0x1_0000u32), 0x1_0000u32);
    ///     assert_eq!(x.to_string(), "995705032704");
    /// }
    /// ```
    fn sub_mul_assign(&mut self, b: &'a Natural, c: Limb) {
        if self.sub_mul_assign_limb_ref_no_panic(b, c) {
            panic!("Natural sub_mul_assign cannot have a negative result");
        }
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> SubMulAssign<&'a Natural, u32> for Natural {
    #[inline]
    fn sub_mul_assign(&mut self, b: &'a Natural, c: u32) {
        self.sub_mul_assign(b, Limb::from(c));
    }
}

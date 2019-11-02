use malachite_base::comparison::Max;
use malachite_base::num::arithmetic::traits::{
    AddMul, AddMulAssign, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::num::conversion::traits::Assign;
use malachite_base::num::logic::traits::NotAssign;

use integer::Integer;
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::mul_limb::{
    limbs_mul_limb_with_carry_to_out, limbs_slice_mul_limb_with_carry_in_place,
};
use natural::arithmetic::sub_limb::{limbs_sub_limb_in_place, limbs_sub_limb_to_out};
use natural::arithmetic::sub_mul_limb::{
    limbs_sub_mul_limb_same_length_in_place_left, limbs_sub_mul_limb_same_length_in_place_right,
};
use natural::logic::not::limbs_not_in_place;
use natural::Natural::{self, Large};
use platform::Limb;

/// Given the limbs of two `Natural`s a and b, and a limb c, calculates a - b * c, returning the
/// limbs of the absolute value and the sign (true means non-negative). `xs` and `ys` should be
/// nonempty and have no trailing zeros, and `limb` should be nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::integer::arithmetic::add_mul_limb::limbs_overflowing_sub_mul_limb;
///
/// assert_eq!(limbs_overflowing_sub_mul_limb(&[123, 456], &[123], 4),
///         (vec![4294966927, 455, 0], true));
/// assert_eq!(limbs_overflowing_sub_mul_limb(&[123], &[123], 4),
///         (vec![369, 0], false));
/// assert_eq!(limbs_overflowing_sub_mul_limb(&[123], &[123, 456], 0xffff_ffff),
///         (vec![4294967050, 4294966962, 455], false));
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive, sub is negative, and w
/// is returned instead of overwriting the first input. w_sign is also returned.
pub fn limbs_overflowing_sub_mul_limb(xs: &[Limb], ys: &[Limb], limb: Limb) -> (Vec<Limb>, bool) {
    let mut result;
    let sign = if xs.len() >= ys.len() {
        result = xs.to_vec();
        limbs_overflowing_sub_mul_limb_greater_in_place_left(&mut result, ys, limb)
    } else {
        result = ys.to_vec();
        limbs_overflowing_sub_mul_limb_smaller_in_place_right(xs, &mut result, limb)
    };
    (result, sign)
}

/// Given the limbs of two `Natural`s a and b, and a limb c, calculates a - b * c, writing the limbs
/// of the absolute value to the first (left) slice and returning the sign (true means
/// non-negative). `xs` and `ys` should be nonempty and have no trailing zeros, and `limb` should be
/// nonzero.
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
/// use malachite_nz::integer::arithmetic::add_mul_limb::*;
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_overflowing_sub_mul_limb_in_place_left(&mut xs, &[123], 4), true);
/// assert_eq!(xs, &[4294966927, 455, 0]);
///
/// let mut xs = vec![123];
/// assert_eq!(limbs_overflowing_sub_mul_limb_in_place_left(&mut xs, &[123], 4), false);
/// assert_eq!(xs, &[369, 0]);
///
/// let mut xs = vec![123];
/// assert_eq!(limbs_overflowing_sub_mul_limb_in_place_left(&mut xs, &[123, 456], 0xffff_ffff),
///         false);
/// assert_eq!(xs, &[4294967050, 4294966962, 455]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive, sub is negative, and
/// w_sign is returned.
pub fn limbs_overflowing_sub_mul_limb_in_place_left(
    xs: &mut Vec<Limb>,
    ys: &[Limb],
    limb: Limb,
) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len >= ys_len {
        limbs_overflowing_sub_mul_limb_greater_in_place_left(xs, ys, limb)
    } else {
        let (ys_lo, ys_hi) = ys.split_at(xs_len);
        // submul of absolute values
        let mut borrow = limbs_sub_mul_limb_same_length_in_place_left(xs, ys_lo, limb);
        // ys bigger than xs, so want ys * limb - xs. Submul has given xs - ys * limb, so take twos'
        // complement and use an limbs_mul_limb_with_carry_to_out for the rest.
        // -(-borrow * b ^ n + xs - ys * limb) = (borrow - 1) * b ^ n + ~(xs - ys * limb) + 1
        limbs_not_in_place(xs);
        if !limbs_slice_add_limb_in_place(xs, 1) {
            borrow.wrapping_sub_assign(1);
        }
        // If borrow - 1 == -1, then hold that -1 for later.
        // limbs_sub_mul_limb_same_length_in_place_left never returns borrow == Limb::MAX, so that
        // value always indicates a -1.
        let negative_one = borrow == Limb::MAX;
        if negative_one {
            borrow.wrapping_add_assign(1);
        }
        xs.resize(ys_len + 1, 0);
        let (xs_hi_last, xs_hi_init) = xs[xs_len..].split_last_mut().unwrap();
        *xs_hi_last = limbs_mul_limb_with_carry_to_out(xs_hi_init, ys_hi, limb, borrow);
        // Apply any -1 from above. The value at xs_hi is non-zero because limb != 0 and the high
        // limb of ys will be non-zero.
        if negative_one {
            limbs_sub_limb_in_place(xs_hi_init, 1);
        }
        false
    }
}

// xs.len() >= ys.len()
fn limbs_overflowing_sub_mul_limb_greater_in_place_left(
    xs: &mut Vec<Limb>,
    ys: &[Limb],
    limb: Limb,
) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    xs.push(0);
    // submul of absolute values
    let mut borrow;
    {
        let (xs_lo, xs_hi) = xs.split_at_mut(ys_len);
        borrow = limbs_sub_mul_limb_same_length_in_place_left(xs_lo, ys, limb);
        // If xs bigger than ys, then propagate borrow through it.
        if xs_len != ys_len {
            borrow = if limbs_sub_limb_in_place(xs_hi, borrow) {
                1
            } else {
                0
            };
        }
    }
    if borrow == 0 {
        true
    } else {
        // Borrow out of xs, take twos' complement negative to get absolute value, flip sign of
        // xs.
        {
            let (xs_last, xs_init) = xs.split_last_mut().unwrap();
            *xs_last = borrow.wrapping_sub(1);
            limbs_not_in_place(xs_init);
        }
        limbs_slice_add_limb_in_place(xs, 1);
        false
    }
}

/// Given the limbs of two `Natural`s a and b, and a limb c, calculates a - b * c, writing the limbs
/// of the absolute value to the second (right) slice and returning the sign (true means
/// non-negative). `xs` and `ys` should be nonempty and have no trailing zeros, and `limb` should be
/// nonzero.
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
/// use malachite_nz::integer::arithmetic::add_mul_limb::*;
///
/// let mut ys = vec![123];
/// assert_eq!(limbs_overflowing_sub_mul_limb_in_place_right(&[123, 456], &mut ys, 4), true);
/// assert_eq!(ys, &[4294966927, 455, 0]);
///
/// let mut ys = vec![123];
/// assert_eq!(limbs_overflowing_sub_mul_limb_in_place_right(&[123], &mut ys, 4), false);
/// assert_eq!(ys, &[369, 0]);
///
/// let mut ys = vec![123, 456];
/// assert_eq!(limbs_overflowing_sub_mul_limb_in_place_right(&[123], &mut ys, 0xffff_ffff), false);
/// assert_eq!(ys, &[4294967050, 4294966962, 455]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive, sub is negative, the
/// limbs of the result are written to the second input rather than the first, and w_sign is
/// returned.
pub fn limbs_overflowing_sub_mul_limb_in_place_right(
    xs: &[Limb],
    ys: &mut Vec<Limb>,
    limb: Limb,
) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len >= ys_len {
        ys.resize(xs_len + 1, 0);
        // submul of absolute values
        let mut borrow;
        {
            let (xs_lo, xs_hi) = xs.split_at(ys_len);
            let (ys_lo, ys_hi) = ys.split_at_mut(ys_len);
            borrow = limbs_sub_mul_limb_same_length_in_place_right(xs_lo, ys_lo, limb);
            // If xs bigger than ys, then propagate borrow through it.
            if xs_len != ys_len {
                borrow = if limbs_sub_limb_to_out(ys_hi, xs_hi, borrow) {
                    1
                } else {
                    0
                };
            }
        }
        if borrow == 0 {
            true
        } else {
            // Borrow out of ys, take twos' complement negative to get absolute value, flip sign of
            // ys.
            {
                let (ys_last, ys_init) = ys.split_last_mut().unwrap();
                *ys_last = borrow.wrapping_sub(1);
                limbs_not_in_place(ys_init);
            }
            limbs_slice_add_limb_in_place(ys, 1);
            false
        }
    } else {
        limbs_overflowing_sub_mul_limb_smaller_in_place_right(xs, ys, limb)
    }
}

// xs.len() < ys.len()
fn limbs_overflowing_sub_mul_limb_smaller_in_place_right(
    xs: &[Limb],
    ys: &mut Vec<Limb>,
    limb: Limb,
) -> bool {
    ys.push(0);
    let (ys_lo, ys_hi) = ys.split_at_mut(xs.len());
    // submul of absolute values
    let mut borrow = limbs_sub_mul_limb_same_length_in_place_right(xs, ys_lo, limb);
    // ys bigger than xs, so want ys * limb - xs. Submul has given xs - ys * limb, so take twos'
    // complement and use an limbs_mul_limb_with_carry_to_out for the rest.
    // -(-borrow * b ^ n + xs - ys * limb) = (borrow - 1) * b ^ n + ~(xs - ys * limb) + 1
    limbs_not_in_place(ys_lo);
    if !limbs_slice_add_limb_in_place(ys_lo, 1) {
        borrow.wrapping_sub_assign(1);
    }
    // If borrow - 1 == -1, then hold that -1 for later.
    // limbs_sub_mul_limb_same_length_in_place_left never returns borrow == Limb::MAX, so that
    // value always indicates a -1.
    let negative_one = borrow == Limb::MAX;
    if negative_one {
        borrow.wrapping_add_assign(1);
    }
    let (ys_hi_last, ys_hi_init) = ys_hi.split_last_mut().unwrap();
    *ys_hi_last = limbs_slice_mul_limb_with_carry_in_place(ys_hi_init, limb, borrow);
    if negative_one {
        limbs_sub_limb_in_place(ys_hi_init, 1);
    }
    false
}

/// Given the limbs of two `Natural`s a and b, and a limb c, calculates a - b * c, writing the limbs
/// of the absolute value to whichever input is longer. The first `bool` returned is the sign of the
/// result (true means non-negative). The second `bool` is `false` if the result is written to the
/// first input, and `true` if it is written to the second. `xs` and `ys` should be nonempty and
/// have no trailing zeros, and `limb` should be nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::integer::arithmetic::add_mul_limb::*;
///
/// let mut xs = vec![123, 456];
/// let mut ys = vec![123];
/// assert_eq!(limbs_overflowing_sub_mul_limb_in_place_either(&mut xs, &mut ys, 4), (false, true));
/// assert_eq!(xs, &[4294966927, 455, 0]);
/// assert_eq!(ys, &[123]);
///
/// let mut xs = vec![123];
/// let mut ys = vec![123];
/// assert_eq!(limbs_overflowing_sub_mul_limb_in_place_either(&mut xs, &mut ys, 4), (false, false));
/// assert_eq!(xs, &[369, 0]);
/// assert_eq!(ys, &[123]);
///
/// let mut xs = vec![123];
/// let mut ys = vec![123, 456];
/// assert_eq!(limbs_overflowing_sub_mul_limb_in_place_either(&mut xs, &mut ys, 0xffff_ffff),
///         (true, false));
/// assert_eq!(xs, &[123]);
/// assert_eq!(ys, &[4294967050, 4294966962, 455]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, where w and x are positive, sub is negative, the
/// result is written to the longer input, and w_sign is returned.
pub fn limbs_overflowing_sub_mul_limb_in_place_either(
    xs: &mut Vec<Limb>,
    ys: &mut Vec<Limb>,
    limb: Limb,
) -> (bool, bool) {
    if xs.len() >= ys.len() {
        (
            false,
            limbs_overflowing_sub_mul_limb_greater_in_place_left(xs, ys, limb),
        )
    } else {
        (
            true,
            limbs_overflowing_sub_mul_limb_smaller_in_place_right(xs, ys, limb),
        )
    }
}

/// Adds the product of an `Integer` (b) and a `Limb` (c) to an `Integer` (self), taking `self` and
/// b by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = max(`a.significant_bits()`, `b.significant_bits()`)
///       m = `min(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(Integer::from(3u32), 4u32), 22);
///     assert_eq!((-Integer::trillion()).add_mul(Integer::from(0x1_0000), 0x1_0000u32).to_string(),
///                "-995705032704");
/// }
/// ```
impl AddMul<Integer, Limb> for Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(mut self, b: Integer, c: Limb) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl AddMul<Integer, u32> for Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(self, b: Integer, c: u32) -> Integer {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of an `Integer` (b) and a `Limb` (c) to an `Integer` (self), taking `self` by
/// value and b by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = max(`a.significant_bits()`, `b.significant_bits()`)
///       m = `b.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!(Integer::from(10u32).add_mul(&Integer::from(3u32), 4u32), 22);
///     assert_eq!((-Integer::trillion()).add_mul(&Integer::from(0x1_0000),
///         0x1_0000u32).to_string(), "-995705032704");
/// }
/// ```
impl<'a> AddMul<&'a Integer, Limb> for Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(mut self, b: &'a Integer, c: Limb) -> Integer {
        self.add_mul_assign(b, c);
        self
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> AddMul<&'a Integer, u32> for Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(self, b: &'a Integer, c: u32) -> Integer {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of an `Integer` (b) and a `Limb` (c) to an `Integer` (self), taking `self` by
/// reference and b by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = max(`a.significant_bits()`, `b.significant_bits()`)
///       m = `a.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).add_mul(Integer::from(3u32), 4u32), 22);
///     assert_eq!((&(-Integer::trillion())).add_mul(Integer::from(0x1_0000),
///         0x1_0000u32).to_string(), "-995705032704");
/// }
/// ```
impl<'a> AddMul<Integer, Limb> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(self, mut b: Integer, c: Limb) -> Integer {
        if self.sign == b.sign {
            Integer {
                sign: self.sign,
                abs: (&self.abs).add_mul(b.abs, c),
            }
        } else {
            let abs_result_sign = b.abs.add_mul_right_assign_limb_neg(&self.abs, c);
            Integer {
                sign: b.sign != abs_result_sign || b.abs == 0 as Limb,
                abs: b.abs,
            }
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> AddMul<Integer, u32> for &'a Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(self, b: Integer, c: u32) -> Integer {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of an `Integer` (b) and a `Limb` (c) to an `Integer` (self), taking `self` and
/// b by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = max(`self.significant_bits()`, `b.significant_bits()`)
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMul;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     assert_eq!((&Integer::from(10u32)).add_mul(&Integer::from(3u32), 4u32), 22);
///     assert_eq!((&(-Integer::trillion())).add_mul(&Integer::from(0x1_0000),
///         0x1_0000u32).to_string(), "-995705032704");
/// }
/// ```
impl<'a, 'b> AddMul<&'a Integer, Limb> for &'b Integer {
    type Output = Integer;

    fn add_mul(self, b: &'a Integer, c: Limb) -> Integer {
        if self.sign == b.sign {
            Integer {
                sign: self.sign,
                abs: (&self.abs).add_mul(&b.abs, c),
            }
        } else {
            let (abs, abs_result_sign) = self.abs.add_mul_limb_neg(&b.abs, c);
            Integer {
                sign: self.sign == abs_result_sign || abs == 0 as Limb,
                abs,
            }
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a, 'b> AddMul<&'a Integer, u32> for &'b Integer {
    type Output = Integer;

    #[inline]
    fn add_mul(self, b: &'a Integer, c: u32) -> Integer {
        self.add_mul(b, Limb::from(c))
    }
}

/// Adds the product of an `Integer` (b) and a `Limb` (c) to an `Integer` (self), in place, taking b
/// by value.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = max(`a.significant_bits()`, `b.significant_bits()`)
///       m = `min(self.significant_bits(), b.significant_bits())`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(Integer::from(3u32), 4u32);
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.add_mul_assign(Integer::from(0x1_0000), 0x1_0000u32);
///     assert_eq!(x.to_string(), "-995705032704");
/// }
/// ```
impl AddMulAssign<Integer, Limb> for Integer {
    fn add_mul_assign(&mut self, b: Integer, c: Limb) {
        if self.sign == b.sign {
            self.abs.add_mul_assign(b.abs, c);
        } else {
            if !self.abs.add_mul_assign_limb_neg(b.abs, c) {
                self.sign.not_assign();
            }
            if self.abs == 0 as Limb {
                self.sign = true;
            }
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl AddMulAssign<Integer, u32> for Integer {
    #[inline]
    fn add_mul_assign(&mut self, b: Integer, c: u32) {
        self.add_mul_assign(b, Limb::from(c));
    }
}

/// Adds the product of an `Integer` (b) and a `Limb` (c) to an `Integer` (self), in place, taking b
/// by reference.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(m)
///
/// where n = max(`a.significant_bits()`, `b.significant_bits()`)
///       m = `b.significant_bits()`
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::arithmetic::traits::AddMulAssign;
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::from(10u32);
///     x.add_mul_assign(&Integer::from(3u32), 4u32);
///     assert_eq!(x, 22);
///
///     let mut x = -Integer::trillion();
///     x.add_mul_assign(&Integer::from(0x1_0000), 0x1_0000u32);
///     assert_eq!(x.to_string(), "-995705032704");
/// }
/// ```
impl<'a> AddMulAssign<&'a Integer, Limb> for Integer {
    fn add_mul_assign(&mut self, b: &'a Integer, c: Limb) {
        if self.sign == b.sign {
            self.abs.add_mul_assign(&b.abs, c);
        } else {
            if !self.abs.add_mul_assign_limb_neg_ref(&b.abs, c) {
                self.sign.not_assign();
            }
            if self.abs == 0 as Limb {
                self.sign = true;
            }
        }
    }
}

#[cfg(not(feature = "32_bit_limbs"))]
impl<'a> AddMulAssign<&'a Integer, u32> for Integer {
    #[inline]
    fn add_mul_assign(&mut self, b: &'a Integer, c: u32) {
        self.add_mul_assign(b, Limb::from(c));
    }
}

impl Natural {
    // self - b * c, returns sign (true means non-negative)
    pub(crate) fn add_mul_limb_neg(&self, b: &Natural, c: Limb) -> (Natural, bool) {
        if c == 0 || *b == 0 as Limb {
            return (self.clone(), true);
        }
        if c == 1 {
            return if self >= b {
                (self - b, true)
            } else {
                (b - self, false)
            };
        }
        match (self, b) {
            (Large(ref a_limbs), Large(ref b_limbs)) => {
                let (limbs, sign) = limbs_overflowing_sub_mul_limb(a_limbs, b_limbs, c);
                let mut result = Large(limbs);
                result.trim();
                (result, sign)
            }
            _ => {
                let bc = b * c;
                if *self >= bc {
                    (self - bc, true)
                } else {
                    (bc - self, false)
                }
            }
        }
    }

    // self -= b * c, returns sign (true means non-negative)
    pub(crate) fn add_mul_assign_limb_neg(&mut self, mut b: Natural, c: Limb) -> bool {
        if c == 0 || b == 0 as Limb {
            return true;
        }
        if c == 1 {
            let sign = *self >= b;
            if sign {
                self.sub_assign_no_panic(b);
            } else {
                self.sub_right_assign_no_panic(&b);
            }
            return sign;
        }
        let (fallback, (right, mut sign)) = match (&mut *self, &mut b) {
            (&mut Large(ref mut a_limbs), &mut Large(ref mut b_limbs)) => (
                false,
                limbs_overflowing_sub_mul_limb_in_place_either(a_limbs, b_limbs, c),
            ),
            _ => (true, (false, false)),
        };
        if fallback {
            let bc = b * c;
            sign = *self >= bc;
            if sign {
                self.sub_assign_no_panic(bc);
            } else {
                self.sub_right_assign_no_panic(&bc);
            }
        } else if right {
            b.trim();
            *self = b;
        } else {
            self.trim();
        }
        sign
    }

    // b = a - b * c, returns sign (true means non-negative). self is b
    fn add_mul_right_assign_limb_neg(&mut self, a: &Natural, c: Limb) -> bool {
        if c == 0 || *self == 0 as Limb {
            self.assign(a);
            return true;
        }
        if c == 1 {
            let sign = a >= self;
            if sign {
                self.sub_right_assign_no_panic(a);
            } else {
                self.sub_assign_ref_no_panic(a);
            }
            return sign;
        }
        let (fallback, mut sign) = match (&a, &mut *self) {
            (&Large(ref a_limbs), &mut Large(ref mut b_limbs)) => (
                false,
                limbs_overflowing_sub_mul_limb_in_place_right(a_limbs, b_limbs, c),
            ),
            _ => (true, false),
        };
        if fallback {
            *self *= c;
            // self is now b * c
            sign = *a >= *self;
            if sign {
                self.sub_right_assign_no_panic(a);
            } else {
                self.sub_assign_ref_no_panic(a);
            }
        } else {
            self.trim();
        }
        sign
    }

    // self -= &b * c, returns sign (true means non-negative)
    pub(crate) fn add_mul_assign_limb_neg_ref(&mut self, b: &Natural, c: Limb) -> bool {
        if c == 0 || *b == 0 as Limb {
            return true;
        }
        if c == 1 {
            let sign = *self >= *b;
            if sign {
                self.sub_assign_ref_no_panic(b);
            } else {
                self.sub_right_assign_no_panic(b);
            }
            return sign;
        }
        let (mut sign, fallback) = match (&mut *self, b) {
            (&mut Large(ref mut a_limbs), &Large(ref b_limbs)) => (
                limbs_overflowing_sub_mul_limb_in_place_left(a_limbs, b_limbs, c),
                false,
            ),
            _ => (false, true),
        };
        if fallback {
            let bc = b * c;
            sign = *self >= bc;
            if sign {
                self.sub_assign_no_panic(bc);
            } else {
                self.sub_right_assign_no_panic(&bc);
            }
        } else {
            self.trim();
        }
        sign
    }
}

use std::cmp::Ordering;

use malachite_base::num::arithmetic::traits::{
    AddMul, AddMulAssign, NegAssign, SubMul, SubMulAssign, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::num::basic::traits::Iverson;
use malachite_base::slices::slice_test_zero;

use integer::Integer;
use natural::arithmetic::add::limbs_slice_add_limb_in_place;
use natural::arithmetic::mul::limb::{
    limbs_mul_limb_with_carry_to_out, limbs_slice_mul_limb_with_carry_in_place,
};
use natural::arithmetic::mul::limbs_mul_greater_to_out;
use natural::arithmetic::sub::{
    limbs_slice_sub_in_place_right, limbs_sub_in_place_left, limbs_sub_limb_in_place,
    limbs_sub_limb_to_out,
};
use natural::arithmetic::sub_mul::{
    limbs_sub_mul_limb_same_length_in_place_left, limbs_sub_mul_limb_same_length_in_place_right,
};
use natural::comparison::ord::limbs_cmp;
use natural::logic::not::limbs_not_in_place;
use platform::Limb;

/// Given the limbs of two `Natural`s x and y, and a limb `z`, calculates x - y * z, returning the
/// limbs of the absolute value and the sign (true means non-negative). `xs` and `ys` should be
/// nonempty and have no trailing zeros, and `z` should be nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::integer::arithmetic::sub_mul::limbs_overflowing_sub_mul_limb;
///
/// assert_eq!(limbs_overflowing_sub_mul_limb(&[123, 456], &[123], 4),
///         (vec![4294966927, 455, 0], true));
/// assert_eq!(limbs_overflowing_sub_mul_limb(&[123], &[123], 4),
///         (vec![369, 0], false));
/// assert_eq!(limbs_overflowing_sub_mul_limb(&[123], &[123, 456], u32::MAX),
///         (vec![4294967050, 4294966962, 455], false));
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, GMP 6.1.2, where w and x are positive, sub is
/// negative, and w is returned instead of overwriting the first input. w_sign is also returned.
pub fn limbs_overflowing_sub_mul_limb(xs: &[Limb], ys: &[Limb], z: Limb) -> (Vec<Limb>, bool) {
    let mut result;
    let sign = if xs.len() >= ys.len() {
        result = xs.to_vec();
        limbs_overflowing_sub_mul_limb_greater_in_place_left(&mut result, ys, z)
    } else {
        result = ys.to_vec();
        limbs_overflowing_sub_mul_limb_smaller_in_place_right(xs, &mut result, z)
    };
    (result, sign)
}

/// Given the limbs of two `Natural`s x and y, and a limb `z`, calculates x - y * z, writing the
/// limbs of the absolute value to the first (left) slice and returning the sign (true means non-
/// negative). `xs` and `ys` should be nonempty and have no trailing zeros, and `z` should be
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
/// use malachite_nz::integer::arithmetic::sub_mul::limbs_overflowing_sub_mul_limb_in_place_left;
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
/// assert_eq!(limbs_overflowing_sub_mul_limb_in_place_left(&mut xs, &[123, 456], u32::MAX),
///         false);
/// assert_eq!(xs, &[4294967050, 4294966962, 455]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, GMP 6.1.2, where w and x are positive, sub is
/// negative, and w_sign is returned.
pub fn limbs_overflowing_sub_mul_limb_in_place_left(
    xs: &mut Vec<Limb>,
    ys: &[Limb],
    z: Limb,
) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len >= ys_len {
        limbs_overflowing_sub_mul_limb_greater_in_place_left(xs, ys, z)
    } else {
        let (ys_lo, ys_hi) = ys.split_at(xs_len);
        // submul of absolute values
        let mut borrow = limbs_sub_mul_limb_same_length_in_place_left(xs, ys_lo, z);
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
        *xs_hi_last = limbs_mul_limb_with_carry_to_out(xs_hi_init, ys_hi, z, borrow);
        // Apply any -1 from above. The value at xs_hi is non-zero because z != 0 and the high limb
        // of ys will be non-zero.
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
    z: Limb,
) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    xs.push(0);
    // submul of absolute values
    let (xs_lo, xs_hi) = xs.split_at_mut(ys_len);
    let mut borrow = limbs_sub_mul_limb_same_length_in_place_left(xs_lo, ys, z);
    // If xs bigger than ys, then propagate borrow through it.
    if xs_len != ys_len {
        borrow = Limb::iverson(limbs_sub_limb_in_place(xs_hi, borrow));
    }
    if borrow == 0 {
        true
    } else {
        // Borrow out of xs, take twos' complement negative to get absolute value, flip sign of xs.
        let (xs_last, xs_init) = xs.split_last_mut().unwrap();
        *xs_last = borrow.wrapping_sub(1);
        limbs_not_in_place(xs_init);
        limbs_slice_add_limb_in_place(xs, 1);
        false
    }
}

/// Given the limbs of two `Natural`s x and y, and a limb `z`, calculates x - y * z, writing the
/// limbs of the absolute value to the second (right) slice and returning the sign (true means non-
/// negative). `xs` and `ys` should be nonempty and have no trailing zeros, and `z` should be
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
/// use malachite_nz::integer::arithmetic::sub_mul::limbs_overflowing_sub_mul_limb_in_place_right;
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
/// assert_eq!(limbs_overflowing_sub_mul_limb_in_place_right(&[123], &mut ys, u32::MAX), false);
/// assert_eq!(ys, &[4294967050, 4294966962, 455]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, GMP 6.1.2, where w and x are positive, sub is
/// negative, the limbs of the result are written to the second input rather than the first, and
/// w_sign is returned.
pub fn limbs_overflowing_sub_mul_limb_in_place_right(
    xs: &[Limb],
    ys: &mut Vec<Limb>,
    z: Limb,
) -> bool {
    let xs_len = xs.len();
    let ys_len = ys.len();
    if xs_len >= ys_len {
        ys.resize(xs_len + 1, 0);
        // submul of absolute values
        let (xs_lo, xs_hi) = xs.split_at(ys_len);
        let (ys_lo, ys_hi) = ys.split_at_mut(ys_len);
        let mut borrow = limbs_sub_mul_limb_same_length_in_place_right(xs_lo, ys_lo, z);
        // If xs bigger than ys, then propagate borrow through it.
        if xs_len != ys_len {
            borrow = Limb::iverson(limbs_sub_limb_to_out(ys_hi, xs_hi, borrow));
        }
        if borrow == 0 {
            true
        } else {
            // Borrow out of ys, take twos' complement negative to get absolute value, flip sign of
            // ys.
            let (ys_last, ys_init) = ys.split_last_mut().unwrap();
            *ys_last = borrow.wrapping_sub(1);
            limbs_not_in_place(ys_init);
            limbs_slice_add_limb_in_place(ys, 1);
            false
        }
    } else {
        limbs_overflowing_sub_mul_limb_smaller_in_place_right(xs, ys, z)
    }
}

// xs.len() < ys.len()
fn limbs_overflowing_sub_mul_limb_smaller_in_place_right(
    xs: &[Limb],
    ys: &mut Vec<Limb>,
    z: Limb,
) -> bool {
    ys.push(0);
    let (ys_lo, ys_hi) = ys.split_at_mut(xs.len());
    // submul of absolute values
    let mut borrow = limbs_sub_mul_limb_same_length_in_place_right(xs, ys_lo, z);
    // ys bigger than xs, so want ys * z - xs. Submul has given xs - ys * z, so take twos'
    // complement and use an limbs_mul_limb_with_carry_to_out for the rest.
    // -(-borrow * b ^ n + xs - ys * z) = (borrow - 1) * b ^ n + ~(xs - ys * z) + 1
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
    *ys_hi_last = limbs_slice_mul_limb_with_carry_in_place(ys_hi_init, z, borrow);
    if negative_one {
        limbs_sub_limb_in_place(ys_hi_init, 1);
    }
    false
}

/// Given the limbs of two `Natural`s x and y, and a limb `z`, calculates x - y * z, writing the
/// limbs of the absolute value to whichever input is longer. The first `bool` returned is `false`
/// if the result is written to the first input, and `true` if it is written to the second. The
/// second `bool` is the sign of the result (true means non-negative). `xs` and `ys` should be
/// nonempty and have no trailing zeros, and `z` should be nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = max(`xs.len()`, `ys.len()`)
///
/// # Example
/// ```
/// use malachite_nz::integer::arithmetic::sub_mul::limbs_overflowing_sub_mul_limb_in_place_either;
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
/// assert_eq!(limbs_overflowing_sub_mul_limb_in_place_either(&mut xs, &mut ys, u32::MAX),
///         (true, false));
/// assert_eq!(xs, &[123]);
/// assert_eq!(ys, &[4294967050, 4294966962, 455]);
/// ```
///
/// This is mpz_aorsmul_1 from mpz/aorsmul_i.c, GMP 6.1.2, where w and x are positive, sub is
/// negative, the result is written to the longer input, and w_sign is returned.
pub fn limbs_overflowing_sub_mul_limb_in_place_either(
    xs: &mut Vec<Limb>,
    ys: &mut Vec<Limb>,
    z: Limb,
) -> (bool, bool) {
    if xs.len() >= ys.len() {
        (
            false,
            limbs_overflowing_sub_mul_limb_greater_in_place_left(xs, ys, z),
        )
    } else {
        (
            true,
            limbs_overflowing_sub_mul_limb_smaller_in_place_right(xs, ys, z),
        )
    }
}

/// Given the limbs of three `Natural`s x, y, and z, calculates x - y * z, returning the limbs of
/// the absolute value and the sign (true means non-negative). All of the input slices should be
/// non-empty and have no trailing zeros.
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
/// use malachite_nz::integer::arithmetic::sub_mul::limbs_overflowing_sub_mul;
///
/// assert_eq!(limbs_overflowing_sub_mul(&[123, 456], &[123, 789], &[321, 654]),
///         (vec![39360, 333255, 516006], false));
/// assert_eq!(limbs_overflowing_sub_mul(&[123, 456, 789, 987, 654], &[123, 789], &[321, 654]),
///         (vec![4294927936, 4294634040, 4294452078, 986, 654], true));
/// ```
///
/// This is mpz_aorsmul from mpz/aorsmul.c, GMP 6.1.2, where w, x, and y are positive, sub is
/// negative, and w is returned instead of overwriting the first input. w_sign is also returned.
pub fn limbs_overflowing_sub_mul(xs: &[Limb], ys: &[Limb], zs: &[Limb]) -> (Vec<Limb>, bool) {
    let mut xs = xs.to_vec();
    let sign = limbs_overflowing_sub_mul_in_place_left(&mut xs, ys, zs);
    (xs, sign)
}

/// Given the limbs of three `Natural`s x, y, and z, calculates x - y * z, writing the limbs of the
/// absolute value to the first (left) slice and returning the sign (true means non-negative). All
/// of the input slices should be non-empty and have no trailing zeros.
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
/// use malachite_nz::integer::arithmetic::sub_mul::limbs_overflowing_sub_mul_in_place_left;
///
/// let mut xs = vec![123, 456];
/// assert_eq!(limbs_overflowing_sub_mul_in_place_left(&mut xs, &[123, 789], &[321, 654]), false);
/// assert_eq!(xs, &[39360, 333255, 516006]);
///
/// let mut xs = vec![123, 456, 789, 987, 654];
/// assert_eq!(limbs_overflowing_sub_mul_in_place_left(&mut xs, &[123, 789], &[321, 654]), true);
/// assert_eq!(xs, &[4294927936, 4294634040, 4294452078, 986, 654]);
/// ```
///
/// This is mpz_aorsmul from mpz/aorsmul.c, GMP 6.1.2, where w, x, and y are positive, sub is
/// negative, and w_sign is returned.
pub fn limbs_overflowing_sub_mul_in_place_left(
    xs: &mut Vec<Limb>,
    ys: &[Limb],
    zs: &[Limb],
) -> bool {
    if ys.len() >= zs.len() {
        limbs_overflowing_sub_mul_greater_in_place_left(xs, ys, zs)
    } else {
        limbs_overflowing_sub_mul_greater_in_place_left(xs, zs, ys)
    }
}

// zs.len() >= ys.len()
fn limbs_overflowing_sub_mul_greater_in_place_left(
    xs: &mut Vec<Limb>,
    ys: &[Limb],
    zs: &[Limb],
) -> bool {
    let xs_len = xs.len();
    let product_len = ys.len() + zs.len();
    let mut product = vec![0; product_len];
    if limbs_mul_greater_to_out(&mut product, ys, zs) == 0 {
        product.pop();
    }
    assert_ne!(*product.last().unwrap(), 0);
    if limbs_cmp(xs, &product) == Ordering::Less {
        if xs_len < product_len {
            xs.resize(product.len(), 0);
        }
        assert!(!limbs_slice_sub_in_place_right(
            &product,
            &mut xs[..product.len()],
            xs_len,
        ));
        false
    } else {
        assert!(!limbs_sub_in_place_left(xs, &product));
        !slice_test_zero(xs)
    }
}

impl<'a> SubMul<Integer, Integer> for Integer {
    type Output = Integer;

    /// Adds the product of an `Integer` (y) and an `Integer` (z) to an `Integer` (self), taking
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
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(10u32).sub_mul(Integer::from(3u32), Integer::from(-4)), 22);
    /// assert_eq!((-Integer::trillion()).sub_mul(Integer::from(-0x10000),
    ///     -Integer::trillion()).to_string(), "-65537000000000000");
    /// ```
    #[inline]
    fn sub_mul(mut self, y: Integer, z: Integer) -> Integer {
        self.sub_mul_assign(y, z);
        self
    }
}

impl<'a> SubMul<Integer, &'a Integer> for Integer {
    type Output = Integer;

    /// Adds the product of an `Integer` (y) and an `Integer` (z) to an `Integer` (self), taking
    /// `self` and y by value and z by reference.
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
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(10u32).sub_mul(Integer::from(3u32), &Integer::from(-4)), 22);
    /// assert_eq!((-Integer::trillion()).sub_mul(Integer::from(-0x10000),
    ///     &(-Integer::trillion())).to_string(), "-65537000000000000");
    /// ```
    #[inline]
    fn sub_mul(mut self, y: Integer, z: &'a Integer) -> Integer {
        self.sub_mul_assign(y, z);
        self
    }
}

impl<'a> SubMul<&'a Integer, Integer> for Integer {
    type Output = Integer;

    /// Adds the product of an `Integer` (y) and an `Integer` (z) to an `Integer` (self), taking
    /// `self` and z by value and y by reference.
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
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(10u32).sub_mul(&Integer::from(3u32), Integer::from(-4)), 22);
    /// assert_eq!((-Integer::trillion()).sub_mul(&Integer::from(-0x10000),
    ///     -Integer::trillion()).to_string(), "-65537000000000000");
    /// ```
    #[inline]
    fn sub_mul(mut self, y: &'a Integer, z: Integer) -> Integer {
        self.sub_mul_assign(y, z);
        self
    }
}

impl<'a, 'b> SubMul<&'a Integer, &'b Integer> for Integer {
    type Output = Integer;

    /// Adds the product of an `Integer` (y) and an `Integer` (z) to an `Integer` (self), taking
    /// `self` by value and y and z by reference.
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
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(10u32).sub_mul(&Integer::from(3u32), &Integer::from(-4)), 22);
    /// assert_eq!((-Integer::trillion()).sub_mul(&Integer::from(-0x10000),
    ///                     &(-Integer::trillion())).to_string(), "-65537000000000000");
    /// ```
    #[inline]
    fn sub_mul(mut self, y: &'a Integer, z: &'b Integer) -> Integer {
        self.sub_mul_assign(y, z);
        self
    }
}

impl<'a, 'b, 'c> SubMul<&'a Integer, &'b Integer> for &'c Integer {
    type Output = Integer;

    /// Adds the product of an `Integer` (b) and an `Integer` (c) to an `Integer` (self), taking
    /// `self`, b, and c by reference.
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
    /// use malachite_base::num::arithmetic::traits::SubMul;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(10u32)).sub_mul(&Integer::from(3u32), &Integer::from(-4)), 22);
    /// assert_eq!((&(-Integer::trillion())).sub_mul(&Integer::from(-0x10000),
    ///                     &(-Integer::trillion())).to_string(), "-65537000000000000");
    /// ```
    fn sub_mul(self, y: &'a Integer, z: &'b Integer) -> Integer {
        if self.sign == (y.sign != z.sign) {
            Integer {
                sign: self.sign,
                abs: (&self.abs).add_mul(&y.abs, &z.abs),
            }
        } else {
            let (abs, abs_result_sign) = self.abs.add_mul_neg(&y.abs, &z.abs);
            Integer {
                sign: (self.sign == abs_result_sign) || abs == 0,
                abs,
            }
        }
    }
}

impl<'a> SubMulAssign<Integer, Integer> for Integer {
    /// Adds the product of an `Integer` (y) and an `Integer` (z) to an `Integer` (self), in place,
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
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10u32);
    /// x.sub_mul_assign(Integer::from(3u32), Integer::from(-4));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = -Integer::trillion();
    /// x.sub_mul_assign(Integer::from(-0x10000), -Integer::trillion());
    /// assert_eq!(x.to_string(), "-65537000000000000");
    /// ```
    fn sub_mul_assign(&mut self, y: Integer, z: Integer) {
        self.add_mul_assign(-y, z);
    }
}

impl<'a> SubMulAssign<Integer, &'a Integer> for Integer {
    /// Adds the product of an `Integer` (y) and an `Integer` (z) to an `Integer` (self), in place,
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
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10u32);
    /// x.sub_mul_assign(Integer::from(3u32), &Integer::from(-4));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = -Integer::trillion();
    /// x.sub_mul_assign(Integer::from(-0x10000), &(-Integer::trillion()));
    /// assert_eq!(x.to_string(), "-65537000000000000");
    /// ```
    fn sub_mul_assign(&mut self, y: Integer, z: &'a Integer) {
        self.add_mul_assign(-y, z);
    }
}

impl<'a> SubMulAssign<&'a Integer, Integer> for Integer {
    /// Adds the product of an `Integer` (y) and an `Integer` (z) to an `Integer` (self), in place,
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
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10u32);
    /// x.sub_mul_assign(&Integer::from(3u32), Integer::from(-4));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = -Integer::trillion();
    /// x.sub_mul_assign(&Integer::from(-0x10000), -Integer::trillion());
    /// assert_eq!(x.to_string(), "-65537000000000000");
    /// ```
    fn sub_mul_assign(&mut self, y: &'a Integer, z: Integer) {
        self.add_mul_assign(y, -z);
    }
}

impl<'a, 'b> SubMulAssign<&'a Integer, &'b Integer> for Integer {
    /// Adds the product of an `Integer` (y) and an `Integer` (z) to an `Integer` (self), in place,
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
    /// use malachite_base::num::arithmetic::traits::SubMulAssign;
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10u32);
    /// x.sub_mul_assign(&Integer::from(3u32), &Integer::from(-4));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = -Integer::trillion();
    /// x.sub_mul_assign(&Integer::from(-0x10000), &(-Integer::trillion()));
    /// assert_eq!(x.to_string(), "-65537000000000000");
    /// ```
    fn sub_mul_assign(&mut self, y: &'a Integer, z: &'b Integer) {
        self.neg_assign();
        self.add_mul_assign(y, z);
        self.neg_assign();
    }
}

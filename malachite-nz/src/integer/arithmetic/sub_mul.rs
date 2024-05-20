// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 2001, 2004, 2005, 2012 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::arithmetic::add::limbs_slice_add_limb_in_place;
use crate::natural::arithmetic::mul::limb::{
    limbs_mul_limb_with_carry_to_out, limbs_slice_mul_limb_with_carry_in_place,
};
use crate::natural::arithmetic::mul::{
    limbs_mul_greater_to_out, limbs_mul_greater_to_out_scratch_len,
};
use crate::natural::arithmetic::sub::{
    limbs_slice_sub_in_place_right, limbs_sub_greater_in_place_left, limbs_sub_limb_in_place,
    limbs_sub_limb_to_out,
};
use crate::natural::arithmetic::sub_mul::{
    limbs_sub_mul_limb_same_length_in_place_left, limbs_sub_mul_limb_same_length_in_place_right,
};
use crate::natural::comparison::cmp::limbs_cmp;
use crate::natural::logic::not::limbs_not_in_place;
use crate::platform::Limb;
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use malachite_base::num::arithmetic::traits::{
    AddMul, AddMulAssign, NegAssign, SubMul, SubMulAssign, WrappingAddAssign, WrappingSubAssign,
};
use malachite_base::slices::slice_test_zero;

// Given the limbs of two `Natural`s x and y, and a limb `z`, calculates x - y * z, returning the
// limbs of the absolute value and the sign (true means non-negative). `xs` and `ys` should be
// nonempty and have no trailing zeros, and `z` should be nonzero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// This is equivalent to `mpz_aorsmul_1` from `mpz/aorsmul_i.c`, GMP 6.2.1, where `w` and `x` are
// positive, `sub` is negative, and `w` is returned instead of overwriting the first input. `w_sign`
// is also returned.
pub_crate_test! {limbs_overflowing_sub_mul_limb(
    xs: &[Limb],
    ys: &[Limb],
    z: Limb
) -> (Vec<Limb>, bool) {
    let mut result;
    let sign = if xs.len() >= ys.len() {
        result = xs.to_vec();
        limbs_overflowing_sub_mul_limb_greater_in_place_left(&mut result, ys, z)
    } else {
        result = ys.to_vec();
        limbs_overflowing_sub_mul_limb_smaller_in_place_right(xs, &mut result, z)
    };
    (result, sign)
}}

// Given the limbs of two `Natural`s x and y, and a limb `z`, calculates x - y * z, writing the
// limbs of the absolute value to the first (left) slice and returning the sign (true means non-
// negative). `xs` and `ys` should be nonempty and have no trailing zeros, and `z` should be
// nonzero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(xs.len(), ys.len())`, and $m$ is `max(1,
// ys.len() - xs.len())`.
//
// This is equivalent to `mpz_aorsmul_1` from `mpz/aorsmul_i.c`, GMP 6.2.1, where `w` and `x` are
// positive, `sub` is negative, and `w_sign` is returned.
pub_crate_test! {limbs_overflowing_sub_mul_limb_in_place_left(
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
        // complement and use an limbs_mul_limb_with_carry_to_out for the rest. -(-borrow * b ^ n +
        // xs - ys * limb) = (borrow - 1) * b ^ n + ~(xs - ys * limb) + 1
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
        let xs_hi = &mut xs[xs_len..];
        let (xs_hi_last, xs_hi_init) = xs_hi.split_last_mut().unwrap();
        *xs_hi_last = limbs_mul_limb_with_carry_to_out(xs_hi_init, ys_hi, z, borrow);
        // Apply any -1 from above. The value at xs_hi is non-zero because z != 0 and the high limb
        // of ys will be non-zero.
        if negative_one {
            assert!(!limbs_sub_limb_in_place(xs_hi, 1));
        }
        false
    }
}}

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
        borrow = Limb::from(limbs_sub_limb_in_place(xs_hi, borrow));
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

// Given the limbs of two `Natural`s x and y, and a limb `z`, calculates x - y * z, writing the
// limbs of the absolute value to the second (right) slice and returning the sign (true means non-
// negative). `xs` and `ys` should be nonempty and have no trailing zeros, and `z` should be
// nonzero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(xs.len(), ys.len())`, and $m$ is `max(1,
// ys.len() - xs.len())`.
//
// This is equivalent to `mpz_aorsmul_1` from `mpz/aorsmul_i.c`, GMP 6.2.1, where `w` and `x` are
// positive, `sub` is negative, the limbs of the result are written to the second input rather than
// the first, and `w_sign` is returned.
pub_test! {limbs_overflowing_sub_mul_limb_in_place_right(
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
            borrow = Limb::from(limbs_sub_limb_to_out(ys_hi, xs_hi, borrow));
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
}}

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
    // complement and use an limbs_mul_limb_with_carry_to_out for the rest. -(-borrow * b ^ n + xs
    // - ys * z) = (borrow - 1) * b ^ n + ~(xs - ys * z) + 1
    limbs_not_in_place(ys_lo);
    if !limbs_slice_add_limb_in_place(ys_lo, 1) {
        borrow.wrapping_sub_assign(1);
    }
    // If borrow - 1 == -1, then hold that -1 for later.
    // limbs_sub_mul_limb_same_length_in_place_left never returns borrow == Limb::MAX, so that value
    // always indicates a -1.
    let negative_one = borrow == Limb::MAX;
    if negative_one {
        borrow.wrapping_add_assign(1);
    }
    let (ys_hi_last, ys_hi_init) = ys_hi.split_last_mut().unwrap();
    *ys_hi_last = limbs_slice_mul_limb_with_carry_in_place(ys_hi_init, z, borrow);
    if negative_one {
        assert!(!limbs_sub_limb_in_place(ys_hi, 1));
    }
    false
}

// Given the limbs of two `Natural`s x and y, and a limb `z`, calculates x - y * z, writing the
// limbs of the absolute value to whichever input is longer. The first `bool` returned is `false` if
// the result is written to the first input, and `true` if it is written to the second. The second
// `bool` is the sign of the result (true means non-negative). `xs` and `ys` should be nonempty and
// have no trailing zeros, and `z` should be nonzero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// This is equivalent to `mpz_aorsmul_1` from `mpz/aorsmul_i.c`, GMP 6.2.1, where `w` and `x` are
// positive, `sub` is negative, the result is written to the longer input, and `w_sign` is returned.
pub_crate_test! {limbs_overflowing_sub_mul_limb_in_place_either(
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
}}

// Given the limbs of three `Natural`s x, y, and z, calculates x - y * z, returning the limbs of the
// absolute value and the sign (true means non-negative). All of the input slices should be
// non-empty and have no trailing zeros.
//
// # Worst-case complexity
// $T(n, m) = O(m + n \log n \log\log n)$
//
// $M(n, m) = O(m + n \log n)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(ys.len(), zs.len())`, and $m$ is
// `xs.len()`.
//
// # Panics
// Panics if `ys` or `zs` are empty.
//
// This is equivalent to `mpz_aorsmul` from `mpz/aorsmul.c`, GMP 6.2.1, where `w`, `x`, and `y` are
// positive, `sub` is negative, and `w` is returned instead of overwriting the first input. `w_sign`
// is also returned.
pub_crate_test! {limbs_overflowing_sub_mul(
    xs: &[Limb],
    ys: &[Limb],
    zs: &[Limb]
) -> (Vec<Limb>, bool) {
    let mut xs = xs.to_vec();
    let sign = limbs_overflowing_sub_mul_in_place_left(&mut xs, ys, zs);
    (xs, sign)
}}

// Given the limbs of three `Natural`s x, y, and z, calculates x - y * z, writing the limbs of the
// absolute value to the first (left) slice and returning the sign (true means non-negative). All of
// the input slices should be non-empty and have no trailing zeros.
//
// # Worst-case complexity
// $T(n, m) = O(m + n \log n \log\log n)$
//
// $M(n, m) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(ys.len(), zs.len())`, and $m$ is
// `xs.len()`.
//
// # Panics
// Panics if `ys` or `zs` are empty.
//
// This is equivalent to `mpz_aorsmul` from `mpz/aorsmul.c`, GMP 6.2.1, where `w`, `x`, and `y` are
// positive, `sub` is negative, and `w_sign` is returned.
pub_crate_test! {limbs_overflowing_sub_mul_in_place_left(
    xs: &mut Vec<Limb>,
    ys: &[Limb],
    zs: &[Limb],
) -> bool {
    if ys.len() >= zs.len() {
        limbs_overflowing_sub_mul_greater_in_place_left(xs, ys, zs)
    } else {
        limbs_overflowing_sub_mul_greater_in_place_left(xs, zs, ys)
    }
}}

// zs.len() >= ys.len()
fn limbs_overflowing_sub_mul_greater_in_place_left(
    xs: &mut Vec<Limb>,
    ys: &[Limb],
    zs: &[Limb],
) -> bool {
    let xs_len = xs.len();
    let product_len = ys.len() + zs.len();
    let mut product = vec![0; product_len];
    let mut mul_scratch = vec![0; limbs_mul_greater_to_out_scratch_len(ys.len(), zs.len())];
    if limbs_mul_greater_to_out(&mut product, ys, zs, &mut mul_scratch) == 0 {
        product.pop();
    }
    assert_ne!(*product.last().unwrap(), 0);
    if limbs_cmp(xs, &product) == Less {
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
        assert!(!limbs_sub_greater_in_place_left(xs, &product));
        !slice_test_zero(xs)
    }
}

impl SubMul<Integer, Integer> for Integer {
    type Output = Integer;

    /// Subtracts an [`Integer`] by the product of two other [`Integer`]s, taking all three by
    /// value.
    ///
    /// $f(x, y, z) = x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMul};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10u32).sub_mul(Integer::from(3u32), Integer::from(-4)),
    ///     22
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12))
    ///         .sub_mul(Integer::from(-0x10000), -Integer::from(10u32).pow(12)),
    ///     -65537000000000000i64
    /// );
    /// ```
    #[inline]
    fn sub_mul(mut self, y: Integer, z: Integer) -> Integer {
        self.sub_mul_assign(y, z);
        self
    }
}

impl<'a> SubMul<Integer, &'a Integer> for Integer {
    type Output = Integer;

    /// Subtracts an [`Integer`] by the product of two other [`Integer`]s, taking the first two by
    /// value and the third by reference.
    ///
    /// $f(x, y, z) = x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMul};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10u32).sub_mul(Integer::from(3u32), &Integer::from(-4)),
    ///     22
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12))
    ///         .sub_mul(Integer::from(-0x10000), &-Integer::from(10u32).pow(12)),
    ///     -65537000000000000i64
    /// );
    /// ```
    #[inline]
    fn sub_mul(mut self, y: Integer, z: &'a Integer) -> Integer {
        self.sub_mul_assign(y, z);
        self
    }
}

impl<'a> SubMul<&'a Integer, Integer> for Integer {
    type Output = Integer;

    /// Subtracts an [`Integer`] by the product of two other [`Integer`]s, taking the first and
    /// third by value and the second by reference.
    ///
    /// $f(x, y, z) = x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMul};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10u32).sub_mul(&Integer::from(3u32), Integer::from(-4)),
    ///     22
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12))
    ///         .sub_mul(&Integer::from(-0x10000), -Integer::from(10u32).pow(12)),
    ///     -65537000000000000i64
    /// );
    /// ```
    #[inline]
    fn sub_mul(mut self, y: &'a Integer, z: Integer) -> Integer {
        self.sub_mul_assign(y, z);
        self
    }
}

impl<'a, 'b> SubMul<&'a Integer, &'b Integer> for Integer {
    type Output = Integer;

    /// Subtracts an [`Integer`] by the product of two other [`Integer`]s, taking the first by value
    /// and the second and third by reference.
    ///
    /// $f(x, y, z) = x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMul};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     Integer::from(10u32).sub_mul(&Integer::from(3u32), &Integer::from(-4)),
    ///     22
    /// );
    /// assert_eq!(
    ///     (-Integer::from(10u32).pow(12))
    ///         .sub_mul(&Integer::from(-0x10000), &-Integer::from(10u32).pow(12)),
    ///     -65537000000000000i64
    /// );
    /// ```
    #[inline]
    fn sub_mul(mut self, y: &'a Integer, z: &'b Integer) -> Integer {
        self.sub_mul_assign(y, z);
        self
    }
}

impl<'a, 'b, 'c> SubMul<&'a Integer, &'b Integer> for &'c Integer {
    type Output = Integer;

    /// Subtracts an [`Integer`] by the product of two other [`Integer`]s, taking all three by
    /// reference.
    ///
    /// $f(x, y, z) = x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMul};
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(
    ///     (&Integer::from(10u32)).sub_mul(&Integer::from(3u32), &Integer::from(-4)),
    ///     22
    /// );
    /// assert_eq!(
    ///     (&-Integer::from(10u32).pow(12))
    ///         .sub_mul(&Integer::from(-0x10000), &-Integer::from(10u32).pow(12)),
    ///     -65537000000000000i64
    /// );
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

impl SubMulAssign<Integer, Integer> for Integer {
    /// Subtracts the product of two other [`Integer`]s from an [`Integer`] in place, taking both
    /// [`Integer`]s on the right-hand side by value.
    ///
    /// $x \gets x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulAssign};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10u32);
    /// x.sub_mul_assign(Integer::from(3u32), Integer::from(-4));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = -Integer::from(10u32).pow(12);
    /// x.sub_mul_assign(Integer::from(-0x10000), -Integer::from(10u32).pow(12));
    /// assert_eq!(x, -65537000000000000i64);
    /// ```
    fn sub_mul_assign(&mut self, y: Integer, z: Integer) {
        self.add_mul_assign(-y, z);
    }
}

impl<'a> SubMulAssign<Integer, &'a Integer> for Integer {
    /// Subtracts the product of two other [`Integer`]s from an [`Integer`] in place, taking the
    /// first [`Integer`] on the right-hand side by value and the second by reference.
    ///
    /// $x \gets x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulAssign};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10u32);
    /// x.sub_mul_assign(Integer::from(3u32), &Integer::from(-4));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = -Integer::from(10u32).pow(12);
    /// x.sub_mul_assign(Integer::from(-0x10000), &(-Integer::from(10u32).pow(12)));
    /// assert_eq!(x, -65537000000000000i64);
    /// ```
    fn sub_mul_assign(&mut self, y: Integer, z: &'a Integer) {
        self.add_mul_assign(-y, z);
    }
}

impl<'a> SubMulAssign<&'a Integer, Integer> for Integer {
    /// Subtracts the product of two other [`Integer`]s from an [`Integer`] in place, taking the
    /// first [`Integer`] on the right-hand side by reference and the second by value.
    ///
    /// $x \gets x + yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulAssign};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10u32);
    /// x.sub_mul_assign(&Integer::from(3u32), Integer::from(-4));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = -Integer::from(10u32).pow(12);
    /// x.sub_mul_assign(&Integer::from(-0x10000), -Integer::from(10u32).pow(12));
    /// assert_eq!(x, -65537000000000000i64);
    /// ```
    fn sub_mul_assign(&mut self, y: &'a Integer, z: Integer) {
        self.add_mul_assign(y, -z);
    }
}

impl<'a, 'b> SubMulAssign<&'a Integer, &'b Integer> for Integer {
    /// Subtracts the product of two other [`Integer`]s from an [`Integer`] in place, taking both
    /// [`Integer`]s on the right-hand side by reference.
    ///
    /// $x \gets x - yz$.
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `self.significant_bits()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulAssign};
    /// use malachite_nz::integer::Integer;
    ///
    /// let mut x = Integer::from(10u32);
    /// x.sub_mul_assign(&Integer::from(3u32), &Integer::from(-4));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = -Integer::from(10u32).pow(12);
    /// x.sub_mul_assign(&Integer::from(-0x10000), &(-Integer::from(10u32).pow(12)));
    /// assert_eq!(x, -65537000000000000i64);
    /// ```
    fn sub_mul_assign(&mut self, y: &'a Integer, z: &'b Integer) {
        self.neg_assign();
        self.add_mul_assign(y, z);
        self.neg_assign();
    }
}

// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 2000-2002, 2005, 2010-2012 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::arithmetic::div_mod::limbs_div_mod_to_out;
use crate::natural::arithmetic::eq_mod::limbs_mod_exact_odd_limb;
use crate::natural::arithmetic::kronecker_symbol::{
    limbs_jacobi_symbol_init, limbs_jacobi_symbol_same_length,
};
use crate::natural::arithmetic::mod_op::limbs_mod_limb_alt_2;
use crate::natural::arithmetic::shr::limbs_shr_to_out;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{Limb, BMOD_1_TO_MOD_1_THRESHOLD};
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{
    JacobiSymbol, KroneckerSymbol, LegendreSymbol, Parity,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::{BitAccess, NotAssign, TrailingZeros};
use malachite_base::slices::slice_leading_zeros;

// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpz_jacobi` from `mpz/jacobi.c`, GMP 6.2.1, where the absolute values of
// both `a` and `b` fit in a limb.
pub_crate_test! {limbs_kronecker_symbol_single(
    x_sign: bool,
    x: Limb,
    y_sign: bool,
    mut y: Limb,
) -> i8 {
    // Common factor of 2 => (a/b) = 0
    if (x | y).even() {
        return 0;
    }
    // (a/-1) = -1 if a < 0, +1 if a >= 0
    let mut negate = !x_sign && !y_sign;
    let y_twos = TrailingZeros::trailing_zeros(y);
    y >>= y_twos;
    // (-1/b) = -1 iff b = 3 (mod 4)
    if !x_sign && y.get_bit(1) {
        negate.not_assign();
    }
    if y_twos.odd() & ((x >> 1) ^ x).get_bit(1) {
        negate.not_assign();
    }
    let j = if y == 1 { 1 } else { x.jacobi_symbol(y) };
    if negate {
        -j
    } else {
        j
    }
}}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// This is equivalent to `mpz_jacobi` from `mpz/jacobi.c`, GMP 6.2.1.
pub_crate_test! {
    limbs_kronecker_symbol(x_sign: bool, xs: &[Limb], y_sign: bool, ys: &[Limb]) -> i8 {
    let mut xs_len = xs.len();
    let mut ys_len = ys.len();
    // The `limbs_jacobi_symbol_same_length` function requires positive x and y, and x odd. So we
    // must handle the cases of x or y zero, then signs, and then the case of even y.
    //
    // (x / 0) = (x = 1 or x = -1)
    if ys_len == 0 {
        return i8::from(xs == [1]);
    }
    // (0 / y) = (y = 1 or y = -1)
    if xs_len == 0 {
        return i8::from(ys == [1]);
    }
    assert_ne!(xs[xs_len - 1], 0);
    assert_ne!(ys[ys_len - 1], 0);
    let mut xs = xs;
    let mut ys = ys;
    // Common factor of 2 => (x / y) = 0
    if (xs[0] | ys[0]).even() {
        return 0;
    }
    // (x / -1) = -1 if x < 0, 1 if x >= 0
    let mut negate = !x_sign && !y_sign;
    ys = &ys[slice_leading_zeros(ys)..];
    ys_len = ys.len();
    let mut y_lo = ys[0];
    let mut y_twos = TrailingZeros::trailing_zeros(y_lo);
    y_lo >>= y_twos;
    if ys_len > 1 && y_twos != 0 {
        let y_1 = ys[1];
        y_lo |= y_1 << (Limb::WIDTH - y_twos);
        if ys_len == 2 && y_1 >> y_twos == 0 {
            ys_len = 1;
        }
    }
    // (-1 / y) = -1 iff y ≡ 3 mod 4
    if !x_sign && y_lo.get_bit(1) {
        negate.not_assign();
    }
    xs = &xs[slice_leading_zeros(xs)..];
    xs_len = xs.len();
    let mut x_lo = xs[0];
    // Ensure xs_len >= ys_len. Take advantage of the generalized reciprocity law: (x / y * 2 ^ n) =
    // (y * 2 ^ n / x) * recip(x, y)
    if xs_len < ys_len {
        swap(&mut xs, &mut ys);
        swap(&mut xs_len, &mut ys_len);
        swap(&mut x_lo, &mut y_lo);
        // The value of x_lo (old y_lo) is a bit subtle. For this code path, we get x_lo as the low,
        // always odd, limb of shifted x. Which is what we need for the reciprocity update below.
        //
        // However, all other uses of x_lo assumes that it is *not* shifted. Luckily, x_lo matters
        // only when either
        // - y_twos > 0, in which case x is always odd
        // - xs_len = ys_len = 1, in which case this code path is never taken.
        y_twos = TrailingZeros::trailing_zeros(y_lo);
        y_lo >>= y_twos;
        if ys_len > 1 && y_twos != 0 {
            let y_1 = ys[1];
            y_lo |= y_1 << (Limb::WIDTH - y_twos);
            if ys_len == 2 && y_1 >> y_twos == 0 {
                ys_len = 1;
            }
        }
        if (x_lo & y_lo).get_bit(1) {
            negate.not_assign();
        }
    }
    if ys_len == 1 {
        if y_twos.odd() & ((x_lo >> 1) ^ x_lo).get_bit(1) {
            negate.not_assign();
        }
        if y_lo == 1 {
            return if negate { -1 } else { 1 };
        }
        if xs_len > 1 {
            assert!(y_lo.odd());
            x_lo = if xs.len() >= BMOD_1_TO_MOD_1_THRESHOLD {
                limbs_mod_limb_alt_2(xs, y_lo)
            } else {
                if y_lo.get_bit(1) {
                    negate.not_assign();
                }
                limbs_mod_exact_odd_limb(xs, y_lo, 0)
            };
        }
        let j = x_lo.jacobi_symbol(y_lo);
        return if negate { -j } else { j };
    }
    // Allocation strategy: For x, we allocate a working copy only for x % y, but when x is much
    // larger than y, we have to allocate space for the large quotient. We use the same area,
    // pointed to by ys_alt, for both the quotient x / y and the working copy of y.
    let mut scratch = vec![
        0;
        if xs_len >= ys_len << 1 {
            xs_len + 1
        } else {
            ys_len << 1
        }
    ];
    let (mut xs_alt, mut ys_alt) = scratch.split_at_mut(ys_len);
    // In the case of even y, we conceptually shift out the powers of two first, and then divide x %
    // y. Hence, when taking those powers of two into account, we must use alow *before* the
    // division. Doing the actual division first is ok, because the point is to remove multiples of
    // y from x, and multiples of 2 ^ k y are good enough.
    if xs_len > ys_len {
        limbs_div_mod_to_out(ys_alt, xs_alt, xs, ys);
        ys_alt = &mut ys_alt[..ys_len];
    } else {
        xs_alt.copy_from_slice(xs);
    }
    if y_twos != 0 {
        if y_twos.odd() & ((x_lo >> 1) ^ x_lo).get_bit(1) {
            negate.not_assign();
        }
        limbs_shr_to_out(ys_alt, ys, y_twos);
        if xs_alt[ys_len - 1] == 0 && ys_alt[ys_len - 1] == 0 {
            xs_alt = &mut xs_alt[..ys_len - 1];
            ys_alt = &mut ys_alt[..ys_len - 1];
        }
    } else {
        ys_alt.copy_from_slice(ys);
    }
    assert_eq!(y_lo, ys_alt[0]);
    let bits = limbs_jacobi_symbol_init(xs_alt[0], y_lo, u8::from(negate));
    limbs_jacobi_symbol_same_length(xs_alt, ys_alt, bits)
}}

impl LegendreSymbol<Integer> for Integer {
    /// Computes the Legendre symbol of two [`Integer`]s, taking both by value.
    ///
    /// This implementation is identical to that of [`JacobiSymbol`], since there is no
    /// computational benefit to requiring that the denominator be prime.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is negative or if `other` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LegendreSymbol;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(10).legendre_symbol(Integer::from(5)), 0);
    /// assert_eq!(Integer::from(7).legendre_symbol(Integer::from(5)), -1);
    /// assert_eq!(Integer::from(11).legendre_symbol(Integer::from(5)), 1);
    /// assert_eq!(Integer::from(-7).legendre_symbol(Integer::from(5)), -1);
    /// assert_eq!(Integer::from(-11).legendre_symbol(Integer::from(5)), 1);
    /// ```
    #[inline]
    fn legendre_symbol(self, other: Integer) -> i8 {
        assert!(other > 0u32);
        assert!(other.odd());
        (&self).kronecker_symbol(&other)
    }
}

impl LegendreSymbol<&Integer> for Integer {
    /// Computes the Legendre symbol of two [`Integer`]s, taking the first by value and the second
    /// by reference.
    ///
    /// This implementation is identical to that of [`JacobiSymbol`], since there is no
    /// computational benefit to requiring that the denominator be prime.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is negative or if `other` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LegendreSymbol;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(10).legendre_symbol(&Integer::from(5)), 0);
    /// assert_eq!(Integer::from(7).legendre_symbol(&Integer::from(5)), -1);
    /// assert_eq!(Integer::from(11).legendre_symbol(&Integer::from(5)), 1);
    /// assert_eq!(Integer::from(-7).legendre_symbol(&Integer::from(5)), -1);
    /// assert_eq!(Integer::from(-11).legendre_symbol(&Integer::from(5)), 1);
    /// ```
    #[inline]
    fn legendre_symbol(self, other: &Integer) -> i8 {
        assert!(*other > 0u32);
        assert!(other.odd());
        (&self).kronecker_symbol(other)
    }
}

impl LegendreSymbol<Integer> for &Integer {
    /// Computes the Legendre symbol of two [`Integer`]s, taking the first by reference and the
    /// second by value.
    ///
    /// This implementation is identical to that of [`JacobiSymbol`], since there is no
    /// computational benefit to requiring that the denominator be prime.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is negative or if `other` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LegendreSymbol;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(10)).legendre_symbol(Integer::from(5)), 0);
    /// assert_eq!((&Integer::from(7)).legendre_symbol(Integer::from(5)), -1);
    /// assert_eq!((&Integer::from(11)).legendre_symbol(Integer::from(5)), 1);
    /// assert_eq!((&Integer::from(-7)).legendre_symbol(Integer::from(5)), -1);
    /// assert_eq!((&Integer::from(-11)).legendre_symbol(Integer::from(5)), 1);
    /// ```
    #[inline]
    fn legendre_symbol(self, other: Integer) -> i8 {
        assert!(other > 0u32);
        assert!(other.odd());
        self.kronecker_symbol(&other)
    }
}

impl LegendreSymbol<&Integer> for &Integer {
    /// Computes the Legendre symbol of two [`Integer`]s, taking both by reference.
    ///
    /// This implementation is identical to that of [`JacobiSymbol`], since there is no
    /// computational benefit to requiring that the denominator be prime.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is negative or if `other` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::LegendreSymbol;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(10)).legendre_symbol(&Integer::from(5)), 0);
    /// assert_eq!((&Integer::from(7)).legendre_symbol(&Integer::from(5)), -1);
    /// assert_eq!((&Integer::from(11)).legendre_symbol(&Integer::from(5)), 1);
    /// assert_eq!((&Integer::from(-7)).legendre_symbol(&Integer::from(5)), -1);
    /// assert_eq!((&Integer::from(-11)).legendre_symbol(&Integer::from(5)), 1);
    /// ```
    #[inline]
    fn legendre_symbol(self, other: &Integer) -> i8 {
        assert!(*other > 0u32);
        assert!(other.odd());
        self.kronecker_symbol(other)
    }
}

impl JacobiSymbol<Integer> for Integer {
    /// Computes the Jacobi symbol of two [`Integer`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is negative or if `other` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::JacobiSymbol;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(10).jacobi_symbol(Integer::from(5)), 0);
    /// assert_eq!(Integer::from(7).jacobi_symbol(Integer::from(5)), -1);
    /// assert_eq!(Integer::from(11).jacobi_symbol(Integer::from(5)), 1);
    /// assert_eq!(Integer::from(11).jacobi_symbol(Integer::from(9)), 1);
    /// assert_eq!(Integer::from(-7).jacobi_symbol(Integer::from(5)), -1);
    /// assert_eq!(Integer::from(-11).jacobi_symbol(Integer::from(5)), 1);
    /// assert_eq!(Integer::from(-11).jacobi_symbol(Integer::from(9)), 1);
    /// ```
    #[inline]
    fn jacobi_symbol(self, other: Integer) -> i8 {
        assert!(other > 0u32);
        assert!(other.odd());
        (&self).kronecker_symbol(&other)
    }
}

impl JacobiSymbol<&Integer> for Integer {
    /// Computes the Jacobi symbol of two [`Integer`]s, taking the first by value and the second by
    /// reference.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is negative or if `other` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::JacobiSymbol;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(10).jacobi_symbol(&Integer::from(5)), 0);
    /// assert_eq!(Integer::from(7).jacobi_symbol(&Integer::from(5)), -1);
    /// assert_eq!(Integer::from(11).jacobi_symbol(&Integer::from(5)), 1);
    /// assert_eq!(Integer::from(11).jacobi_symbol(&Integer::from(9)), 1);
    /// assert_eq!(Integer::from(-7).jacobi_symbol(&Integer::from(5)), -1);
    /// assert_eq!(Integer::from(-11).jacobi_symbol(&Integer::from(5)), 1);
    /// assert_eq!(Integer::from(-11).jacobi_symbol(&Integer::from(9)), 1);
    /// ```
    #[inline]
    fn jacobi_symbol(self, other: &Integer) -> i8 {
        assert!(*other > 0u32);
        assert!(other.odd());
        (&self).kronecker_symbol(other)
    }
}

impl JacobiSymbol<Integer> for &Integer {
    /// Computes the Jacobi symbol of two [`Integer`]s, taking the first by reference and the second
    /// by value.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is negative or if `other` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::JacobiSymbol;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(10)).jacobi_symbol(Integer::from(5)), 0);
    /// assert_eq!((&Integer::from(7)).jacobi_symbol(Integer::from(5)), -1);
    /// assert_eq!((&Integer::from(11)).jacobi_symbol(Integer::from(5)), 1);
    /// assert_eq!((&Integer::from(11)).jacobi_symbol(Integer::from(9)), 1);
    /// assert_eq!((&Integer::from(-7)).jacobi_symbol(Integer::from(5)), -1);
    /// assert_eq!((&Integer::from(-11)).jacobi_symbol(Integer::from(5)), 1);
    /// assert_eq!((&Integer::from(-11)).jacobi_symbol(Integer::from(9)), 1);
    /// ```
    #[inline]
    fn jacobi_symbol(self, other: Integer) -> i8 {
        assert!(other > 0u32);
        assert!(other.odd());
        self.kronecker_symbol(&other)
    }
}

impl JacobiSymbol<&Integer> for &Integer {
    /// Computes the Jacobi symbol of two [`Integer`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Panics
    /// Panics if `self` is negative or if `other` is even.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::JacobiSymbol;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(10)).jacobi_symbol(&Integer::from(5)), 0);
    /// assert_eq!((&Integer::from(7)).jacobi_symbol(&Integer::from(5)), -1);
    /// assert_eq!((&Integer::from(11)).jacobi_symbol(&Integer::from(5)), 1);
    /// assert_eq!((&Integer::from(11)).jacobi_symbol(&Integer::from(9)), 1);
    /// assert_eq!((&Integer::from(-7)).jacobi_symbol(&Integer::from(5)), -1);
    /// assert_eq!((&Integer::from(-11)).jacobi_symbol(&Integer::from(5)), 1);
    /// assert_eq!((&Integer::from(-11)).jacobi_symbol(&Integer::from(9)), 1);
    /// ```
    #[inline]
    fn jacobi_symbol(self, other: &Integer) -> i8 {
        assert!(*other > 0u32);
        assert!(other.odd());
        self.kronecker_symbol(other)
    }
}

impl KroneckerSymbol<Integer> for Integer {
    /// Computes the Kronecker symbol of two [`Integer`]s, taking both by value.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::KroneckerSymbol;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(10).kronecker_symbol(Integer::from(5)), 0);
    /// assert_eq!(Integer::from(7).kronecker_symbol(Integer::from(5)), -1);
    /// assert_eq!(Integer::from(11).kronecker_symbol(Integer::from(5)), 1);
    /// assert_eq!(Integer::from(11).kronecker_symbol(Integer::from(9)), 1);
    /// assert_eq!(Integer::from(11).kronecker_symbol(Integer::from(8)), -1);
    /// assert_eq!(Integer::from(-7).kronecker_symbol(Integer::from(5)), -1);
    /// assert_eq!(Integer::from(-11).kronecker_symbol(Integer::from(5)), 1);
    /// assert_eq!(Integer::from(-11).kronecker_symbol(Integer::from(9)), 1);
    /// assert_eq!(Integer::from(-11).kronecker_symbol(Integer::from(8)), -1);
    /// assert_eq!(Integer::from(-11).kronecker_symbol(Integer::from(-8)), 1);
    /// ```
    #[inline]
    fn kronecker_symbol(self, other: Integer) -> i8 {
        (&self).kronecker_symbol(&other)
    }
}

impl KroneckerSymbol<&Integer> for Integer {
    /// Computes the Kronecker symbol of two [`Integer`]s, taking the first by value and the second
    /// by reference.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::KroneckerSymbol;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!(Integer::from(10).kronecker_symbol(&Integer::from(5)), 0);
    /// assert_eq!(Integer::from(7).kronecker_symbol(&Integer::from(5)), -1);
    /// assert_eq!(Integer::from(11).kronecker_symbol(&Integer::from(5)), 1);
    /// assert_eq!(Integer::from(11).kronecker_symbol(&Integer::from(9)), 1);
    /// assert_eq!(Integer::from(11).kronecker_symbol(&Integer::from(8)), -1);
    /// assert_eq!(Integer::from(-7).kronecker_symbol(&Integer::from(5)), -1);
    /// assert_eq!(Integer::from(-11).kronecker_symbol(&Integer::from(5)), 1);
    /// assert_eq!(Integer::from(-11).kronecker_symbol(&Integer::from(9)), 1);
    /// assert_eq!(Integer::from(-11).kronecker_symbol(&Integer::from(8)), -1);
    /// assert_eq!(Integer::from(-11).kronecker_symbol(&Integer::from(-8)), 1);
    /// ```
    #[inline]
    fn kronecker_symbol(self, other: &Integer) -> i8 {
        (&self).kronecker_symbol(other)
    }
}

impl KroneckerSymbol<Integer> for &Integer {
    /// Computes the Kronecker symbol of two [`Integer`]s, taking the first by reference and the
    /// second value.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::KroneckerSymbol;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(10)).kronecker_symbol(Integer::from(5)), 0);
    /// assert_eq!((&Integer::from(7)).kronecker_symbol(Integer::from(5)), -1);
    /// assert_eq!((&Integer::from(11)).kronecker_symbol(Integer::from(5)), 1);
    /// assert_eq!((&Integer::from(11)).kronecker_symbol(Integer::from(9)), 1);
    /// assert_eq!((&Integer::from(11)).kronecker_symbol(Integer::from(8)), -1);
    /// assert_eq!((&Integer::from(-7)).kronecker_symbol(Integer::from(5)), -1);
    /// assert_eq!((&Integer::from(-11)).kronecker_symbol(Integer::from(5)), 1);
    /// assert_eq!((&Integer::from(-11)).kronecker_symbol(Integer::from(9)), 1);
    /// assert_eq!((&Integer::from(-11)).kronecker_symbol(Integer::from(8)), -1);
    /// assert_eq!((&Integer::from(-11)).kronecker_symbol(Integer::from(-8)), 1);
    /// ```
    #[inline]
    fn kronecker_symbol(self, other: Integer) -> i8 {
        self.kronecker_symbol(&other)
    }
}

impl KroneckerSymbol<&Integer> for &Integer {
    /// Computes the Kronecker symbol of two [`Integer`]s, taking both by reference.
    ///
    /// $$
    /// f(x, y) = \left ( \frac{x}{y} \right ).
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n (\log n)^2 \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::KroneckerSymbol;
    /// use malachite_nz::integer::Integer;
    ///
    /// assert_eq!((&Integer::from(10)).kronecker_symbol(&Integer::from(5)), 0);
    /// assert_eq!((&Integer::from(7)).kronecker_symbol(&Integer::from(5)), -1);
    /// assert_eq!((&Integer::from(11)).kronecker_symbol(&Integer::from(5)), 1);
    /// assert_eq!((&Integer::from(11)).kronecker_symbol(&Integer::from(9)), 1);
    /// assert_eq!((&Integer::from(11)).kronecker_symbol(&Integer::from(8)), -1);
    /// assert_eq!((&Integer::from(-7)).kronecker_symbol(&Integer::from(5)), -1);
    /// assert_eq!((&Integer::from(-11)).kronecker_symbol(&Integer::from(5)), 1);
    /// assert_eq!((&Integer::from(-11)).kronecker_symbol(&Integer::from(9)), 1);
    /// assert_eq!(
    ///     (&Integer::from(-11)).kronecker_symbol(&Integer::from(8)),
    ///     -1
    /// );
    /// assert_eq!(
    ///     (&Integer::from(-11)).kronecker_symbol(&Integer::from(-8)),
    ///     1
    /// );
    /// ```
    fn kronecker_symbol(self, other: &Integer) -> i8 {
        match (self, other) {
            (x, integer_zero!()) => i8::from(*x.unsigned_abs_ref() == 1u32),
            (integer_zero!(), y) => i8::from(*y.unsigned_abs_ref() == 1u32),
            (
                Integer {
                    sign: x_sign,
                    abs: Natural(Small(x_abs)),
                },
                Integer {
                    sign: y_sign,
                    abs: Natural(Small(y_abs)),
                },
            ) => limbs_kronecker_symbol_single(*x_sign, *x_abs, *y_sign, *y_abs),
            (
                Integer {
                    sign: x_sign,
                    abs: Natural(Small(x_abs)),
                },
                Integer {
                    sign: y_sign,
                    abs: Natural(Large(ys)),
                },
            ) => limbs_kronecker_symbol(*x_sign, &[*x_abs], *y_sign, ys),
            (
                Integer {
                    sign: x_sign,
                    abs: Natural(Large(xs)),
                },
                Integer {
                    sign: y_sign,
                    abs: Natural(Small(y_abs)),
                },
            ) => limbs_kronecker_symbol(*x_sign, xs, *y_sign, &[*y_abs]),
            (
                Integer {
                    sign: x_sign,
                    abs: Natural(Large(xs)),
                },
                Integer {
                    sign: y_sign,
                    abs: Natural(Large(ys)),
                },
            ) => limbs_kronecker_symbol(*x_sign, xs, *y_sign, ys),
        }
    }
}

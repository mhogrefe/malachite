// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1992-1994, 1996, 2000, 2001, 2002, 2004, 2005, 2012 Free Software Foundation,
//      Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::mul::limbs_mul;
use crate::natural::arithmetic::sub::{limbs_sub_greater_in_place_left, limbs_sub_limb_in_place};
use crate::natural::comparison::cmp::limbs_cmp;
use crate::natural::Natural;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use core::cmp::Ordering::*;
use core::fmt::Display;
use malachite_base::num::arithmetic::traits::{
    CheckedSubMul, SubMul, SubMulAssign, WrappingAddAssign,
};
use malachite_base::num::conversion::traits::SplitInHalf;

// Given the limbs of two `Natural`s x and y, and a limb z, returns the limbs of x - y * z. If y * z
// > x, `None` is returned.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is shorter than `ys`.
//
// This is equivalent to `mpz_aorsmul_1` from `mpz/aorsmul_i.c`, GMP 6.2.1, where `w` and `x` are
// positive, `sub` is negative, and `w` is returned instead of overwriting the first input.
pub_crate_test! {limbs_sub_mul_limb_greater(
    xs: &[Limb],
    ys: &[Limb],
    z: Limb
) -> Option<Vec<Limb>> {
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
}}

// Given the equal-length limbs of two `Natural`s x and y, and a limb z, calculates x - y * z and
// writes the limbs of the result to the first (left) input slice. If y * z > x, a nonzero borrow is
// returned.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` and `ys` have different lengths.
//
// This is equivalent to `mpn_submul_1` from `mpn/generic/submul_1.c`, GMP 6.2.1.
pub_crate_test! {limbs_sub_mul_limb_same_length_in_place_left(
    xs: &mut [Limb],
    ys: &[Limb],
    z: Limb
) -> Limb {
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
}}

// Given the limbs of two `Natural`s x and y, and a limb z, calculates x - y * z and writes the
// limbs of the result to the first (left) input slice. If y * z > x, a nonzero borrow is returned.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` is shorter than `ys`.
//
// This is equivalent to `mpn_submul_1` from `mpn/generic/submul_1.c`, GMP 6.2.1, but where the
// first input may be longer than the second.
pub_crate_test! {limbs_sub_mul_limb_greater_in_place_left(
    xs: &mut [Limb],
    ys: &[Limb],
    limb: Limb
) -> Limb {
    let (xs_lo, xs_hi) = xs.split_at_mut(ys.len());
    let borrow = limbs_sub_mul_limb_same_length_in_place_left(xs_lo, ys, limb);
    if borrow == 0 || xs_hi.is_empty() {
        borrow
    } else {
        Limb::from(limbs_sub_limb_in_place(xs_hi, borrow))
    }
}}

// Given the equal-length limbs of two `Natural`s x and y, and a limb z, calculates x - y * z and
// writes the limbs of the result to the second (right) input slice. If y * z > x, a nonzero borrow
// is returned.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if `xs` and `ys` have different lengths.
//
// This is equivalent to `mpz_aorsmul_1` from `mpz/aorsmul_i.c`, GMP 6.2.1, where `w` and `x` are
// positive and have the same lengths, sub is negative, and the lowest limbs of the result are
// written to the second input rather than the first.
pub_crate_test! {limbs_sub_mul_limb_same_length_in_place_right(
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
}}

// Given the limbs of two `Natural`s x and y, and a limb z, calculates x - y * z and writes the
// limbs of the result to the second (right) input `Vec`. If y * z > x, a nonzero borrow is
// returned.

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `xs.len()`, and $m$ is `xs.len() - ys.len()`.
//
// # Panics
// Panics if `xs` is shorter than `ys`.
//
// This is equivalent to `mpz_aorsmul_1` from `mpz/aorsmul_i.c`, GMP 6.2.1, where `w` and `x` are
// positive, `sub` is negative, and the result is written to the second input rather than the first.
pub_test! {limbs_sub_mul_limb_greater_in_place_right(
    xs: &[Limb],
    ys: &mut Vec<Limb>,
    z: Limb
) -> Limb {
    let ys_len = ys.len();
    let (xs_lo, xs_hi) = xs.split_at(ys_len);
    let borrow = limbs_sub_mul_limb_same_length_in_place_right(xs_lo, ys, z);
    if xs_hi.is_empty() {
        borrow
    } else {
        ys.extend(&xs[ys_len..]);
        if borrow == 0 {
            0
        } else {
            Limb::from(limbs_sub_limb_in_place(&mut ys[ys_len..], borrow))
        }
    }
}}

// Given the limbs `xs`, `ys` and `zs` of three `Natural`s x, y, and z, returns the limbs of x - y
// * z. If x < y * z, `None` is returned. `ys` and `zs` should have length at least 2, and the
// length of `xs` should be at least `ys.len()` + `zs.len()` - 1 (if the latter condition is false,
// the result would be `None` and there's no point in calling this function). None of the slices
// should have any trailing zeros. The result, if it exists, will have no trailing zeros.
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
// Panics if `ys` or `zs` have fewer than two elements each, or if `xs.len()` < `ys.len()` +
// `zs.len()` - 1.
//
// This is equivalent to `mpz_aorsmul` from `mpz/aorsmul.c`, GMP 6.2.1, where `w`, `x`, and `y` are
// positive, `sub` is negative, negative results are converted to `None`, and `w` is returned
// instead of overwriting the first input.
pub_crate_test! {limbs_sub_mul(xs: &[Limb], ys: &[Limb], zs: &[Limb]) -> Option<Vec<Limb>> {
    let mut xs = xs.to_vec();
    if limbs_sub_mul_in_place_left(&mut xs, ys, zs) {
        None
    } else {
        Some(xs)
    }
}}

// Given the limbs `xs`, `ys` and `zs` of three `Natural`s x, y, and z, computes x - y * z. The
// limbs of the result are written to `xs`. Returns whether a borrow (overflow) occurred: if x < y
// * z, `true` is returned and the value of `xs` should be ignored. `ys` and `zs` should have
// length at least 2, and the length of `xs` should be at least `ys.len()` + `zs.len()` - 1 (if the
// latter condition is false, the result would be negative and there would be no point in calling
// this function). None of the slices should have any trailing zeros. The result, if it exists, will
// have no trailing zeros.
//
// # Worst-case complexity
// $T(n, m) = O(m + n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(ys.len(), zs.len())`, and $m$ is
// `xs.len()`.
//
// # Panics
// Panics if `ys` or `zs` have fewer than two elements each, or if `xs.len() < ys.len() + zs.len()
// - 1`.
//
// This is equivalent to `mpz_aorsmul` from `mpz/aorsmul.c`, GMP 6.2.1, where `w`, `x`, and `y` are
// positive, `sub` is negative and negative results are discarded.
pub_crate_test! {limbs_sub_mul_in_place_left(xs: &mut [Limb], ys: &[Limb], zs: &[Limb]) -> bool {
    assert!(ys.len() > 1);
    assert!(zs.len() > 1);
    let mut scratch = limbs_mul(ys, zs);
    assert!(xs.len() >= scratch.len() - 1);
    if *scratch.last().unwrap() == 0 {
        scratch.pop();
    }
    let borrow = limbs_cmp(xs, &scratch) == Less;
    if !borrow {
        assert!(!limbs_sub_greater_in_place_left(xs, &scratch));
    }
    borrow
}}

fn sub_mul_panic<S: Display, T: Display, U: Display>(a: S, b: T, c: U) -> ! {
    panic!("Cannot perform sub_mul. a: {a}, b: {b}, c: {c}");
}

impl SubMul<Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s, taking all three by value.
    ///
    /// $$
    /// f(x, y, z) = x - yz.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(20u32).sub_mul(Natural::from(3u32), Natural::from(4u32)),
    ///     8
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .sub_mul(Natural::from(0x10000u32), Natural::from(0x10000u32)),
    ///     995705032704u64
    /// );
    /// ```
    fn sub_mul(self, y: Natural, z: Natural) -> Natural {
        self.checked_sub_mul(y, z)
            .expect("Natural sub_mul_assign cannot have a negative result")
    }
}

impl<'a> SubMul<Natural, &'a Natural> for Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s, taking the first two by
    /// value and the third by reference.
    ///
    /// $$
    /// f(x, y, z) = x - yz.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(20u32).sub_mul(Natural::from(3u32), &Natural::from(4u32)),
    ///     8
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .sub_mul(Natural::from(0x10000u32), &Natural::from(0x10000u32)),
    ///     995705032704u64
    /// );
    /// ```
    fn sub_mul(self, y: Natural, z: &'a Natural) -> Natural {
        self.checked_sub_mul(y, z)
            .expect("Natural sub_mul_assign cannot have a negative result")
    }
}

impl<'a> SubMul<&'a Natural, Natural> for Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s, taking the first and third
    /// by value and the second by reference.
    ///
    /// $$
    /// f(x, y, z) = x - yz.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(20u32).sub_mul(&Natural::from(3u32), Natural::from(4u32)),
    ///     8
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .sub_mul(&Natural::from(0x10000u32), Natural::from(0x10000u32)),
    ///     995705032704u64
    /// );
    /// ```
    fn sub_mul(self, y: &'a Natural, z: Natural) -> Natural {
        self.checked_sub_mul(y, z)
            .expect("Natural sub_mul_assign cannot have a negative result")
    }
}

impl<'a, 'b> SubMul<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s, taking the first by value
    /// and the second and third by reference.
    ///
    /// $$
    /// f(x, y, z) = x - yz.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(20u32).sub_mul(&Natural::from(3u32), &Natural::from(4u32)),
    ///     8
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .sub_mul(&Natural::from(0x10000u32), &Natural::from(0x10000u32)),
    ///     995705032704u64
    /// );
    /// ```
    fn sub_mul(self, y: &'a Natural, z: &'b Natural) -> Natural {
        self.checked_sub_mul(y, z)
            .expect("Natural sub_mul_assign cannot have a negative result")
    }
}

impl SubMul<&Natural, &Natural> for &Natural {
    type Output = Natural;

    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s, taking all three by
    /// reference.
    ///
    /// $$
    /// f(x, y, z) = x - yz.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n, m) = O(m + n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMul};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(20u32)).sub_mul(&Natural::from(3u32), &Natural::from(4u32)),
    ///     8
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12))
    ///         .sub_mul(&Natural::from(0x10000u32), &Natural::from(0x10000u32)),
    ///     995705032704u64
    /// );
    /// ```
    fn sub_mul(self, y: &Natural, z: &Natural) -> Natural {
        self.checked_sub_mul(y, z).unwrap_or_else(|| {
            sub_mul_panic(self, y, z);
        })
    }
}

impl SubMulAssign<Natural, Natural> for Natural {
    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s in place, taking both
    /// [`Natural`]s on the right-hand side by value.
    ///
    /// $$
    /// x \gets x - yz.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(20u32);
    /// x.sub_mul_assign(Natural::from(3u32), Natural::from(4u32));
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.sub_mul_assign(Natural::from(0x10000u32), Natural::from(0x10000u32));
    /// assert_eq!(x, 995705032704u64);
    /// ```
    fn sub_mul_assign(&mut self, y: Natural, z: Natural) {
        assert!(
            !self.sub_mul_assign_no_panic(y, z),
            "Natural sub_mul_assign cannot have a negative result"
        );
    }
}

impl<'a> SubMulAssign<Natural, &'a Natural> for Natural {
    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s in place, taking the first
    /// [`Natural`] on the right-hand side by value and the second by reference.
    ///
    /// $$
    /// x \gets x - yz.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(20u32);
    /// x.sub_mul_assign(Natural::from(3u32), &Natural::from(4u32));
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.sub_mul_assign(Natural::from(0x10000u32), &Natural::from(0x10000u32));
    /// assert_eq!(x, 995705032704u64);
    /// ```
    fn sub_mul_assign(&mut self, y: Natural, z: &'a Natural) {
        assert!(
            !self.sub_mul_assign_val_ref_no_panic(y, z),
            "Natural sub_mul_assign cannot have a negative result"
        );
    }
}

impl<'a> SubMulAssign<&'a Natural, Natural> for Natural {
    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s in place, taking the first
    /// [`Natural`] on the right-hand side by reference and the second by value.
    ///
    /// $$
    /// x \gets x - yz.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(20u32);
    /// x.sub_mul_assign(&Natural::from(3u32), Natural::from(4u32));
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.sub_mul_assign(&Natural::from(0x10000u32), Natural::from(0x10000u32));
    /// assert_eq!(x, 995705032704u64);
    /// ```
    fn sub_mul_assign(&mut self, y: &'a Natural, z: Natural) {
        assert!(
            !self.sub_mul_assign_ref_val_no_panic(y, z),
            "Natural sub_mul_assign cannot have a negative result"
        );
    }
}

impl<'a, 'b> SubMulAssign<&'a Natural, &'b Natural> for Natural {
    /// Subtracts a [`Natural`] by the product of two other [`Natural`]s in place, taking both
    /// [`Natural`]s on the right-hand side by reference.
    ///
    /// $$
    /// x \gets x - yz.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n, m) = O(m + n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, $n$ is `max(y.significant_bits(),
    /// z.significant_bits())`, and $m$ is `x.significant_bits()`.
    ///
    /// # Panics
    /// Panics if `y * z` is greater than `self`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::{Pow, SubMulAssign};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(20u32);
    /// x.sub_mul_assign(&Natural::from(3u32), &Natural::from(4u32));
    /// assert_eq!(x, 8);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.sub_mul_assign(&Natural::from(0x10000u32), &Natural::from(0x10000u32));
    /// assert_eq!(x, 995705032704u64);
    /// ```
    fn sub_mul_assign(&mut self, y: &'a Natural, z: &'b Natural) {
        assert!(
            !self.sub_mul_assign_ref_ref_no_panic(y, z),
            "Natural sub_mul_assign cannot have a negative result"
        );
    }
}

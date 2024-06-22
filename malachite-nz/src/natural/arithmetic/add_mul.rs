// Copyright © 2024 Mikhail Hogrefe
//
// Some optimizations contributed by florian1345.
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1992-1994, 1996, 2000, 2001, 2002, 2004, 2005, 2012, 2016 Free Software
//      Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::add::{
    limbs_add_greater, limbs_slice_add_greater_in_place_left, limbs_slice_add_limb_in_place,
};
use crate::natural::arithmetic::mul::limb::{limbs_mul_limb_to_out, limbs_slice_mul_limb_in_place};
use crate::natural::arithmetic::mul::{limbs_mul_to_out, limbs_mul_to_out_scratch_len};
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign, XMulYToZZ};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::{SplitInHalf, WrappingFrom};

// Given the limbs of two `Natural`s x and y, and a limb `z`, returns the limbs of x + y * z. `xs`
// and `ys` should be nonempty and have no trailing zeros, and `z` should be nonzero. The result
// will have no trailing zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// This is equivalent to `mpz_aorsmul_1` from `mpz/aorsmul_i.c`, GMP 6.2.1, where `w` and `x` are
// positive, `sub` is positive, and `w` is returned instead of overwriting the first input.
pub_test! {limbs_add_mul_limb(xs: &[Limb], ys: &[Limb], limb: Limb) -> Vec<Limb> {
    let mut out;
    if xs.len() >= ys.len() {
        out = xs.to_vec();
        limbs_vec_add_mul_limb_greater_in_place_left(&mut out, ys, limb);
    } else {
        out = ys.to_vec();
        limbs_vec_add_mul_limb_smaller_in_place_right(xs, &mut out, limb);
    }
    out
}}

// Given the equal-length limbs of two `Natural`s x and y, and a limb `z`, computes x + y * z. The
// lowest `xs.len()` limbs of the result are written to `xs`, and the highest limb of y * z, plus
// the carry-out from the addition, is returned.
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
// This is equivalent to `mpn_addmul_1` from `mpn/generic/addmul_1.c`, GMP 6.2.1.
pub_crate_test! {limbs_slice_add_mul_limb_same_length_in_place_left(
    xs: &mut [Limb],
    ys: &[Limb],
    z: Limb,
) -> Limb {
    let len = xs.len();
    assert_eq!(ys.len(), len);
    let mut carry = 0;
    for (x, &y) in xs.iter_mut().zip(ys.iter()) {
        let (product_hi, mut product_lo) = XMulYToZZ::x_mul_y_to_zz(y, z);
        product_lo = (*x).wrapping_add(product_lo);
        let mut add_carry = Limb::from(*x > product_lo);
        *x = product_lo.wrapping_add(carry);
        add_carry += Limb::from(product_lo > *x);
        carry = product_hi.wrapping_add(add_carry);
    }
    carry
}}

pub(crate) fn limbs_slice_add_mul_two_limbs_matching_length_in_place_left(
    xs: &mut [Limb],
    ys: &[Limb],
    zs: [Limb; 2],
) -> Limb {
    let len = ys.len();
    assert_eq!(xs.len(), len + 1);
    let mut carry_hi: Limb = 0;
    let mut carry_lo: Limb = 0;

    for (x, &y) in xs.iter_mut().zip(ys.iter()) {
        let (mut product_hi, mut product_lo) = XMulYToZZ::x_mul_y_to_zz(y, zs[0]);

        product_lo = (*x).wrapping_add(product_lo);
        let mut add_carry = Limb::from(*x > product_lo);

        *x = product_lo.wrapping_add(carry_lo);
        add_carry += Limb::from(product_lo > *x);

        carry_lo = product_hi.wrapping_add(add_carry);
        carry_lo = carry_hi.wrapping_add(carry_lo);
        add_carry = Limb::from(carry_hi > carry_lo);

        (product_hi, product_lo) = XMulYToZZ::x_mul_y_to_zz(y, zs[1]);
        carry_lo = product_lo.wrapping_add(carry_lo);
        add_carry += Limb::from(product_lo > carry_lo);
        carry_hi = product_hi.wrapping_add(add_carry);
    }

    xs[len] = carry_lo;

    carry_hi
}

// Given the limbs of two `Natural`s x and y, and a limb `z`, computes x + y * z. The lowest limbs
// of the result are written to `ys` and the highest limb is returned. `xs` must have the same
// length as `ys`.
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
// positive and have the same lengths, `sub` is positive, the lowest limbs of the result are written
// to the second input rather than the first, and the highest limb is returned.
pub_test! {limbs_slice_add_mul_limb_same_length_in_place_right(
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
    Limb::wrapping_from(carry)
}}

// Given the limbs of two `Natural`s a and b, and a limb c, writes the limbs of a + b * c to the
// first (left) input, corresponding to the limbs of a. `xs` and `ys` should be nonempty and have no
// trailing zeros, and `z` should be nonzero. The result will have no trailing zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(xs.len(), ys.len())` and $m$ is `max(1,
// ys.len() - xs.len())`.
//
// This is equivalent to `mpz_aorsmul_1` from `mpz/aorsmul_i.c`, GMP 6.2.1, where `w` and `x` are
// positive and sub is positive.
pub_test! {limbs_vec_add_mul_limb_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], z: Limb) {
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
}}

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

// Given the limbs of two `Natural`s x and y, and a limb `z`, writes the limbs of x + y * z to the
// second (right) input, corresponding to the limbs of y. `xs` and `ys` should be nonempty and have
// no trailing zeros, and `z` should be nonzero. The result will have no trailing zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(m) = O(m)$
//
// where $T$ is time, $M$ is additional memory, $n$ is `max(xs.len(), ys.len())` and $m$ is `max(1,
// ys.len() - xs.len())`.
//
// This is equivalent to `mpz_aorsmul_1` from `mpz/aorsmul_i.c`, GMP 6.2.1, where `w` and `x` are
// positive, `sub` is positive, and the result is written to the second input rather than the first.
pub_test! {limbs_vec_add_mul_limb_in_place_right(xs: &[Limb], ys: &mut Vec<Limb>, z: Limb) {
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
}}

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

// Given the limbs of two `Natural`s x and y, and a limb `z`, writes the limbs of x + y * z to
// whichever input is longer. If the result is written to the first input, `false` is returned; if
// to the second, `true` is returned. `xs` and `ys` should be nonempty and have no trailing zeros,
// and `z` should be nonzero. The result will have no trailing zeros.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory and $n$ is `max(xs.len(), ys.len())`.
//
// This is equivalent to `mpz_aorsmul_1` from `mpz/aorsmul_i.c`, GMP 6.2.1, where `w` and `x` are
// positive, `sub` is positive, and the result is written to the longer input.
pub_test! {limbs_vec_add_mul_limb_in_place_either(
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
}}

// Given the limbs `xs`, `ys` and `zs` of three `Natural`s x, y, and z, returns the limbs of x + y
// * z. `xs` should be nonempty and `ys` and `zs` should have length at least 2. None of the slices
// should have any trailing zeros. The result will have no trailing zeros.
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
// positive, `sub` is positive, and `w` is returned instead of overwriting the first input.
pub_test! {limbs_add_mul(xs: &[Limb], ys: &[Limb], zs: &[Limb]) -> Vec<Limb> {
    let xs_len = xs.len();
    let mut out_len = ys.len() + zs.len();
    let mut out = vec![0; out_len];
    let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(ys.len(), zs.len())];
    if limbs_mul_to_out(&mut out, ys, zs, &mut mul_scratch) == 0 {
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
}}

// Given the limbs `xs`, `ys` and `zs` of three `Natural`s x, y, and z, computes x + y * z. The
// limbs of the result are written to `xs`. `xs` should be nonempty and `ys` and `zs` should have
// length at least 2. None of the slices should have any trailing zeros. The result will have no
// trailing zeros.
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
// Panics if `ys` or `zs` are empty.
//
// This is equivalent to `mpz_aorsmul` from `mpz/aorsmul.c`, GMP 6.2.1, where `w`, `x`, and `y` are
// positive and `sub` is positive.
pub_test! {limbs_add_mul_in_place_left(xs: &mut Vec<Limb>, ys: &[Limb], zs: &[Limb]) {
    let xs_len = xs.len();
    let mut out_len = ys.len() + zs.len();
    let mut out = vec![0; out_len];
    let mut mul_scratch = vec![0; limbs_mul_to_out_scratch_len(ys.len(), zs.len())];
    if limbs_mul_to_out(&mut out, ys, zs, &mut mul_scratch) == 0 {
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
}}

impl Natural {
    fn add_mul_limb_ref_ref(&self, y: &Natural, z: Limb) -> Natural {
        match (self, y, z) {
            (x, _, 0) | (x, &Natural::ZERO, _) => x.clone(),
            (x, y, 1) => x + y,
            (x, &Natural::ONE, z) => x + Natural::from(z),
            (Natural(Large(ref xs)), Natural(Large(ref ys)), z) => {
                Natural(Large(limbs_add_mul_limb(xs, ys, z)))
            }
            (x, y, z) => x + y * Natural::from(z),
        }
    }

    fn add_mul_assign_limb(&mut self, mut y: Natural, z: Limb) {
        match (&mut *self, &mut y, z) {
            (_, _, 0) | (_, &mut Natural::ZERO, _) => {}
            (x, _, 1) => *x += y,
            (x, &mut Natural::ONE, z) => *x += Natural::from(z),
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
            (_, _, 0) | (_, &Natural::ZERO, _) => {}
            (x, y, 1) => *x += y,
            (x, &Natural::ONE, z) => *x += Natural::from(z),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys)), z) => {
                limbs_vec_add_mul_limb_in_place_left(xs, ys, z);
            }
            (x, y, z) => *x += y * Natural::from(z),
        }
    }
}

impl AddMul<Natural, Natural> for Natural {
    type Output = Natural;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, taking all three by value.
    ///
    /// $f(x, y, z) = x + yz$.
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
    /// use malachite_base::num::arithmetic::traits::{AddMul, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).add_mul(Natural::from(3u32), Natural::from(4u32)),
    ///     22
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .add_mul(Natural::from(0x10000u32), Natural::from(10u32).pow(12)),
    ///     65537000000000000u64
    /// );
    /// ```
    #[inline]
    fn add_mul(mut self, y: Natural, z: Natural) -> Natural {
        self.add_mul_assign(y, z);
        self
    }
}

impl<'a> AddMul<Natural, &'a Natural> for Natural {
    type Output = Natural;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, taking the first two by value
    /// and the third by reference.
    ///
    /// $f(x, y, z) = x + yz$.
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
    /// use malachite_base::num::arithmetic::traits::{AddMul, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).add_mul(Natural::from(3u32), &Natural::from(4u32)),
    ///     22
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .add_mul(Natural::from(0x10000u32), &Natural::from(10u32).pow(12)),
    ///     65537000000000000u64
    /// );
    /// ```
    #[inline]
    fn add_mul(mut self, y: Natural, z: &'a Natural) -> Natural {
        self.add_mul_assign(y, z);
        self
    }
}

impl<'a> AddMul<&'a Natural, Natural> for Natural {
    type Output = Natural;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, taking the first and third by
    /// value and the second by reference.
    ///
    /// $f(x, y, z) = x + yz$.
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
    /// use malachite_base::num::arithmetic::traits::{AddMul, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).add_mul(&Natural::from(3u32), Natural::from(4u32)),
    ///     22
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .add_mul(&Natural::from(0x10000u32), Natural::from(10u32).pow(12)),
    ///     65537000000000000u64
    /// );
    /// ```
    #[inline]
    fn add_mul(mut self, y: &'a Natural, z: Natural) -> Natural {
        self.add_mul_assign(y, z);
        self
    }
}

impl<'a, 'b> AddMul<&'a Natural, &'b Natural> for Natural {
    type Output = Natural;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, taking the first by value and
    /// the second and third by reference.
    ///
    /// $f(x, y, z) = x + yz$.
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
    /// use malachite_base::num::arithmetic::traits::{AddMul, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(10u32).add_mul(&Natural::from(3u32), &Natural::from(4u32)),
    ///     22
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32)
    ///         .pow(12)
    ///         .add_mul(&Natural::from(0x10000u32), &Natural::from(10u32).pow(12)),
    ///     65537000000000000u64
    /// );
    /// ```
    #[inline]
    fn add_mul(mut self, y: &'a Natural, z: &'b Natural) -> Natural {
        self.add_mul_assign(y, z);
        self
    }
}

impl<'a, 'b, 'c> AddMul<&'a Natural, &'b Natural> for &'c Natural {
    type Output = Natural;

    /// Adds a [`Natural`] and the product of two other [`Natural`]s, taking all three by reference.
    ///
    /// $f(x, y, z) = x + yz$.
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
    /// use malachite_base::num::arithmetic::traits::{AddMul, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(10u32)).add_mul(&Natural::from(3u32), &Natural::from(4u32)),
    ///     22
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32).pow(12))
    ///         .add_mul(&Natural::from(0x10000u32), &Natural::from(10u32).pow(12)),
    ///     65537000000000000u64
    /// );
    /// ```
    fn add_mul(self, y: &'a Natural, z: &'b Natural) -> Natural {
        match (self, y, z) {
            (Natural(Small(x)), y, z) => (y * z).add_limb(*x),
            (x, Natural(Small(y)), z) => x.add_mul_limb_ref_ref(z, *y),
            (x, y, Natural(Small(z))) => x.add_mul_limb_ref_ref(y, *z),
            (Natural(Large(ref xs)), Natural(Large(ref ys)), Natural(Large(ref zs))) => {
                Natural(Large(limbs_add_mul(xs, ys, zs)))
            }
        }
    }
}

impl AddMulAssign<Natural, Natural> for Natural {
    /// Adds the product of two other [`Natural`]s to a [`Natural`] in place, taking both
    /// [`Natural`]s on the right-hand side by value.
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
    /// use malachite_base::num::arithmetic::traits::{AddMulAssign, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.add_mul_assign(Natural::from(3u32), Natural::from(4u32));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.add_mul_assign(Natural::from(0x10000u32), Natural::from(10u32).pow(12));
    /// assert_eq!(x, 65537000000000000u64);
    /// ```
    fn add_mul_assign(&mut self, mut y: Natural, mut z: Natural) {
        match (&mut *self, &mut y, &mut z) {
            (Natural(Small(x)), _, _) => *self = (y * z).add_limb(*x),
            (_, Natural(Small(y)), _) => self.add_mul_assign_limb(z, *y),
            (_, _, Natural(Small(z))) => self.add_mul_assign_limb(y, *z),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys)), Natural(Large(ref zs))) => {
                limbs_add_mul_in_place_left(xs, ys, zs);
            }
        }
    }
}

impl<'a> AddMulAssign<Natural, &'a Natural> for Natural {
    /// Adds the product of two other [`Natural`]s to a [`Natural`] in place, taking the first
    /// [`Natural`] on the right-hand side by value and the second by reference.
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
    /// use malachite_base::num::arithmetic::traits::{AddMulAssign, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.add_mul_assign(Natural::from(3u32), &Natural::from(4u32));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.add_mul_assign(Natural::from(0x10000u32), &Natural::from(10u32).pow(12));
    /// assert_eq!(x, 65537000000000000u64);
    /// ```
    fn add_mul_assign(&mut self, mut y: Natural, z: &'a Natural) {
        match (&mut *self, &mut y, z) {
            (Natural(Small(x)), _, _) => *self = (y * z).add_limb(*x),
            (_, Natural(Small(y)), _) => self.add_mul_assign_limb_ref(z, *y),
            (_, _, Natural(Small(z))) => self.add_mul_assign_limb(y, *z),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys)), Natural(Large(ref zs))) => {
                limbs_add_mul_in_place_left(xs, ys, zs);
            }
        }
    }
}

impl<'a> AddMulAssign<&'a Natural, Natural> for Natural {
    /// Adds the product of two other [`Natural`]s to a [`Natural`] in place, taking the first
    /// [`Natural`] on the right-hand side by reference and the second by value.
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
    /// use malachite_base::num::arithmetic::traits::{AddMulAssign, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.add_mul_assign(&Natural::from(3u32), Natural::from(4u32));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.add_mul_assign(&Natural::from(0x10000u32), Natural::from(10u32).pow(12));
    /// assert_eq!(x, 65537000000000000u64);
    /// ```
    fn add_mul_assign(&mut self, y: &'a Natural, mut z: Natural) {
        match (&mut *self, y, &mut z) {
            (Natural(Small(x)), _, _) => *self = (y * z).add_limb(*x),
            (_, Natural(Small(y)), _) => self.add_mul_assign_limb(z, *y),
            (_, _, Natural(Small(z))) => self.add_mul_assign_limb_ref(y, *z),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys)), Natural(Large(ref zs))) => {
                limbs_add_mul_in_place_left(xs, ys, zs);
            }
        }
    }
}

impl<'a, 'b> AddMulAssign<&'a Natural, &'b Natural> for Natural {
    /// Adds the product of two other [`Natural`]s to a [`Natural`] in place, taking both
    /// [`Natural`]s on the right-hand side by reference.
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
    /// use malachite_base::num::arithmetic::traits::{AddMulAssign, Pow};
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(10u32);
    /// x.add_mul_assign(&Natural::from(3u32), &Natural::from(4u32));
    /// assert_eq!(x, 22);
    ///
    /// let mut x = Natural::from(10u32).pow(12);
    /// x.add_mul_assign(&Natural::from(0x10000u32), &Natural::from(10u32).pow(12));
    /// assert_eq!(x, 65537000000000000u64);
    /// ```
    fn add_mul_assign(&mut self, y: &'a Natural, z: &'b Natural) {
        match (&mut *self, y, z) {
            (Natural(Small(x)), _, _) => *self = (y * z).add_limb(*x),
            (_, Natural(Small(y)), _) => self.add_mul_assign_limb_ref(z, *y),
            (_, _, Natural(Small(z))) => self.add_mul_assign_limb_ref(y, *z),
            (Natural(Large(ref mut xs)), Natural(Large(ref ys)), Natural(Large(ref zs))) => {
                limbs_add_mul_in_place_left(xs, ys, zs);
            }
        }
    }
}

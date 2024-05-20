// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991, 1993-1995, 2001, 2002, 2012 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::conversion::to_twos_complement_limbs::limbs_twos_complement_in_place;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Assign, NegModPowerOf2, NegModPowerOf2Assign, RemPowerOf2,
    RemPowerOf2Assign, ShrRound,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::slices::slice_set_zero;

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the `Natural` mod two raised to `pow`. Equivalently, retains only the least-significant
// `pow` bits.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
//
// This is equivalent to `mpz_tdiv_r_2exp` from `mpz/tdiv_r_2exp.c`, GMP 6.2.1, where in is
// non-negative and the result is returned.
pub_test! {limbs_mod_power_of_2(xs: &[Limb], pow: u64) -> Vec<Limb> {
    if pow == 0 {
        return Vec::new();
    }
    let leftover_bits = pow & Limb::WIDTH_MASK;
    let result_size = usize::exact_from(pow >> Limb::LOG_WIDTH);
    if result_size >= xs.len() {
        return xs.to_vec();
    }
    let mut result = xs[..result_size].to_vec();
    if leftover_bits != 0 {
        result.push(xs[result_size].mod_power_of_2(leftover_bits));
    }
    result
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural` mod two raised to `pow` to the input slice. Equivalently, retains only the
// least-significant `pow` bits. If the upper limbs of the input slice are no longer needed, they
// are set to zero.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpz_tdiv_r_2exp` from `mpz/tdiv_r_2exp.c`, GMP 6.2.1, where `in` is
// non-negative, `res == in`, and instead of possibly being truncated, the high limbs of `res` are
// possibly filled with zeros.
pub_crate_test! {limbs_slice_mod_power_of_2_in_place(xs: &mut [Limb], pow: u64) {
    if pow == 0 {
        slice_set_zero(xs);
        return;
    }
    let new_size = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, Ceiling).0);
    if new_size > xs.len() {
        return;
    }
    slice_set_zero(&mut xs[new_size..]);
    let leftover_bits = pow & Limb::WIDTH_MASK;
    if leftover_bits != 0 {
        xs[new_size - 1].mod_power_of_2_assign(leftover_bits);
    }
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the `Natural` mod two raised to `pow` to the input `Vec`. Equivalently, retains only the
// least-significant `pow` bits.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpz_tdiv_r_2exp` from `mpz/tdiv_r_2exp.c`, GMP 6.2.1, where `in` is
// non-negative and `res == in`.
pub_crate_test! {limbs_vec_mod_power_of_2_in_place(xs: &mut Vec<Limb>, pow: u64) {
    if pow == 0 {
        xs.clear();
        return;
    }
    let new_size = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, Ceiling).0);
    if new_size > xs.len() {
        return;
    }
    xs.truncate(new_size);
    let leftover_bits = pow & Limb::WIDTH_MASK;
    if leftover_bits != 0 {
        xs[new_size - 1].mod_power_of_2_assign(leftover_bits);
    }
}}

// Interpreting a slice of `Limb`s as the limbs (in ascending order) of a `Natural`, returns the
// limbs of the negative of the `Natural` mod two raised to `pow`. Equivalently, takes the two's
// complement and retains only the least-significant `pow` bits.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
//
// This is equivalent to `mpz_tdiv_r_2exp` from `mpz/tdiv_r_2exp.c`, GMP 6.2.1, where `in` is
// negative and the result is returned. `xs` is the limbs of `-in`.
pub_crate_test! {limbs_neg_mod_power_of_2(xs: &[Limb], pow: u64) -> Vec<Limb> {
    let mut result = xs.to_vec();
    limbs_neg_mod_power_of_2_in_place(&mut result, pow);
    result
}}

// Interpreting a `Vec` of `Limb`s as the limbs (in ascending order) of a `Natural`, writes the
// limbs of the negative of the `Natural` mod two raised to `pow` to the input `Vec`. Equivalently,
// takes the two's complement and retains only the least-significant `pow` bits.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
//
// This is equivalent to `mpz_tdiv_r_2exp` from `mpz/tdiv_r_2exp.c`, GMP 6.2.1, where `in` is
// negative and `res == in`. `xs` is the limbs of `-in`.
pub_crate_test! {limbs_neg_mod_power_of_2_in_place(xs: &mut Vec<Limb>, pow: u64) {
    let new_size = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, Ceiling).0);
    xs.resize(new_size, 0);
    limbs_twos_complement_in_place(xs);
    let leftover_bits = pow & Limb::WIDTH_MASK;
    if leftover_bits != 0 {
        xs[new_size - 1].mod_power_of_2_assign(leftover_bits);
    }
}}

impl ModPowerOf2 for Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by $2^k$, returning just the remainder. The [`Natural`] is taken by
    /// value.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and
    /// $0 \leq r < 2^k$.
    ///
    /// $$
    /// f(x, k) = x - 2^k\left \lfloor \frac{x}{2^k} \right \rfloor.
    /// $$
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Natural::from(260u32).mod_power_of_2(8), 4);
    ///
    /// // 100 * 2^4 + 11 = 1611
    /// assert_eq!(Natural::from(1611u32).mod_power_of_2(4), 11);
    /// ```
    #[inline]
    fn mod_power_of_2(mut self, pow: u64) -> Natural {
        self.mod_power_of_2_assign(pow);
        self
    }
}

impl<'a> ModPowerOf2 for &'a Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by $2^k$, returning just the remainder. The [`Natural`] is taken by
    /// reference.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and
    /// $0 \leq r < 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!((&Natural::from(260u32)).mod_power_of_2(8), 4);
    /// // 100 * 2^4 + 11 = 1611
    /// assert_eq!((&Natural::from(1611u32)).mod_power_of_2(4), 11);
    /// ```
    fn mod_power_of_2(self, pow: u64) -> Natural {
        match *self {
            Natural(Small(ref small)) => Natural(Small(small.mod_power_of_2(pow))),
            Natural(Large(ref limbs)) => {
                Natural::from_owned_limbs_asc(limbs_mod_power_of_2(limbs, pow))
            }
        }
    }
}

impl ModPowerOf2Assign for Natural {
    /// Divides a [`Natural`]by $2^k$, replacing the [`Natural`] by the remainder.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and
    /// $0 \leq r < 2^k$.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Assign;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// let mut x = Natural::from(260u32);
    /// x.mod_power_of_2_assign(8);
    /// assert_eq!(x, 4);
    ///
    /// // 100 * 2^4 + 11 = 1611
    /// let mut x = Natural::from(1611u32);
    /// x.mod_power_of_2_assign(4);
    /// assert_eq!(x, 11);
    /// ```
    fn mod_power_of_2_assign(&mut self, pow: u64) {
        match *self {
            Natural(Small(ref mut small)) => small.mod_power_of_2_assign(pow),
            Natural(Large(ref mut limbs)) => {
                limbs_vec_mod_power_of_2_in_place(limbs, pow);
                self.trim();
            }
        }
    }
}

impl RemPowerOf2 for Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by $2^k$, returning just the remainder. The [`Natural`] is taken by
    /// value.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and
    /// $0 \leq r < 2^k$.
    ///
    /// $$
    /// f(x, k) = x - 2^k\left \lfloor \frac{x}{2^k} \right \rfloor.
    /// $$
    ///
    /// For [`Natural`]s, `rem_power_of_2` is equivalent to
    /// [`mod_power_of_2`](ModPowerOf2::mod_power_of_2).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RemPowerOf2;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!(Natural::from(260u32).rem_power_of_2(8), 4);
    ///
    /// // 100 * 2^4 + 11 = 1611
    /// assert_eq!(Natural::from(1611u32).rem_power_of_2(4), 11);
    /// ```
    #[inline]
    fn rem_power_of_2(self, pow: u64) -> Natural {
        self.mod_power_of_2(pow)
    }
}

impl<'a> RemPowerOf2 for &'a Natural {
    type Output = Natural;

    /// Divides a [`Natural`] by $2^k$, returning just the remainder. The [`Natural`] is taken by
    /// reference.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and
    /// $0 \leq r < 2^k$.
    ///
    /// $$
    /// f(x, k) = x - 2^k\left \lfloor \frac{x}{2^k} \right \rfloor.
    /// $$
    ///
    /// For [`Natural`]s, `rem_power_of_2` is equivalent to
    /// [`mod_power_of_2`](ModPowerOf2::mod_power_of_2).
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RemPowerOf2;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// assert_eq!((&Natural::from(260u32)).rem_power_of_2(8), 4);
    /// // 100 * 2^4 + 11 = 1611
    /// assert_eq!((&Natural::from(1611u32)).rem_power_of_2(4), 11);
    /// ```
    #[inline]
    fn rem_power_of_2(self, pow: u64) -> Natural {
        self.mod_power_of_2(pow)
    }
}

impl RemPowerOf2Assign for Natural {
    /// Divides a [`Natural`] by $2^k$, replacing the first [`Natural`] by the remainder.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k + r$ and
    /// $0 \leq r < 2^k$.
    ///
    /// $$
    /// x \gets x - 2^k\left \lfloor \frac{x}{2^k} \right \rfloor.
    /// $$
    ///
    /// For [`Natural`]s, `rem_power_of_2_assign` is equivalent to
    /// [`mod_power_of_2_assign`](ModPowerOf2Assign::mod_power_of_2_assign).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::RemPowerOf2Assign;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 1 * 2^8 + 4 = 260
    /// let mut x = Natural::from(260u32);
    /// x.rem_power_of_2_assign(8);
    /// assert_eq!(x, 4);
    ///
    /// // 100 * 2^4 + 11 = 1611
    /// let mut x = Natural::from(1611u32);
    /// x.rem_power_of_2_assign(4);
    /// assert_eq!(x, 11);
    /// ```
    #[inline]
    fn rem_power_of_2_assign(&mut self, pow: u64) {
        self.mod_power_of_2_assign(pow);
    }
}

impl NegModPowerOf2 for Natural {
    type Output = Natural;

    /// Divides the negative of a [`Natural`] by a $2^k$, returning just the remainder. The
    /// [`Natural`] is taken by value.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k - r$ and
    /// $0 \leq r < 2^k$.
    ///
    /// $$
    /// f(x, k) = 2^k\left \lceil \frac{x}{2^k} \right \rceil - x.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::NegModPowerOf2;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 2^8 - 252 = 260
    /// assert_eq!(Natural::from(260u32).neg_mod_power_of_2(8), 252);
    ///
    /// // 101 * 2^4 - 5 = 1611
    /// assert_eq!(Natural::from(1611u32).neg_mod_power_of_2(4), 5);
    /// ```
    #[inline]
    fn neg_mod_power_of_2(mut self, pow: u64) -> Natural {
        self.neg_mod_power_of_2_assign(pow);
        self
    }
}

impl<'a> NegModPowerOf2 for &'a Natural {
    type Output = Natural;

    /// Divides the negative of a [`Natural`] by a $2^k$, returning just the remainder. The
    /// [`Natural`] is taken by reference.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k - r$ and
    /// $0 \leq r < 2^k$.
    ///
    /// $$
    /// f(x, k) = 2^k\left \lceil \frac{x}{2^k} \right \rceil - x.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::NegModPowerOf2;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 2^8 - 252 = 260
    /// assert_eq!((&Natural::from(260u32)).neg_mod_power_of_2(8), 252);
    /// // 101 * 2^4 - 5 = 1611
    /// assert_eq!((&Natural::from(1611u32)).neg_mod_power_of_2(4), 5);
    /// ```
    fn neg_mod_power_of_2(self, pow: u64) -> Natural {
        match (self, pow) {
            (&Natural::ZERO, _) => Natural::ZERO,
            (_, pow) if pow <= Limb::WIDTH => {
                Natural::from(Limb::wrapping_from(self).neg_mod_power_of_2(pow))
            }
            (Natural(Small(small)), pow) => {
                Natural::from_owned_limbs_asc(limbs_neg_mod_power_of_2(&[*small], pow))
            }
            (Natural(Large(ref limbs)), pow) => {
                Natural::from_owned_limbs_asc(limbs_neg_mod_power_of_2(limbs, pow))
            }
        }
    }
}

impl NegModPowerOf2Assign for Natural {
    /// Divides the negative of a [`Natural`] by $2^k$, returning just the remainder.
    ///
    /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k - r$ and
    /// $0 \leq r < 2^k$.
    ///
    /// $$
    /// x \gets 2^k\left \lceil \frac{x}{2^k} \right \rceil - x.
    /// $$
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::NegModPowerOf2Assign;
    /// use malachite_nz::natural::Natural;
    ///
    /// // 2 * 2^8 - 252 = 260
    /// let mut x = Natural::from(260u32);
    /// x.neg_mod_power_of_2_assign(8);
    /// assert_eq!(x, 252);
    ///
    /// // 101 * 2^4 - 5 = 1611
    /// let mut x = Natural::from(1611u32);
    /// x.neg_mod_power_of_2_assign(4);
    /// assert_eq!(x, 5);
    /// ```
    fn neg_mod_power_of_2_assign(&mut self, pow: u64) {
        if *self == 0 {
        } else if pow <= Limb::WIDTH {
            *self = Natural::from(Limb::wrapping_from(&*self).neg_mod_power_of_2(pow));
        } else {
            let limbs = self.promote_in_place();
            limbs_neg_mod_power_of_2_in_place(limbs, pow);
            self.trim();
        }
    }
}

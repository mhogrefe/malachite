// Copyright © 2025 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991-2018 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::arithmetic::add::{limbs_add, limbs_add_limb};
use crate::natural::arithmetic::divisible_by::{
    limbs_divisible_by, limbs_divisible_by_limb, limbs_divisible_by_val_ref,
};
use crate::natural::arithmetic::eq_mod::{limbs_eq_limb_mod_limb, limbs_mod_exact_odd_limb};
use crate::natural::arithmetic::mod_op::limbs_mod_limb;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{Limb, BMOD_1_TO_MOD_1_THRESHOLD};
use malachite_base::fail_on_untested_path;
use malachite_base::num::arithmetic::traits::{
    DivisibleBy, EqMod, EqModPowerOf2, NegMod, PowerOf2,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::logic::traits::TrailingZeros;

// Interpreting a slice of `Limb`s as the limbs of a `Natural` in ascending order, determines
// whether that `Natural` is equal to the negative of a limb mod a given `Limb` m.
//
// This function assumes that `m` is nonzero, `limbs` has at least two elements, and the last
// element of `limbs` is nonzero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if the length of `limbs` is less than 2.
//
// This is equivalent to `mpz_congruent_ui_p` from `mpz/cong_ui.c`, GMP 6.2.1, where `a` is
// negative.
pub_test! {limbs_eq_neg_limb_mod_limb(xs: &[Limb], y: Limb, m: Limb) -> bool {
    limbs_eq_limb_mod_limb(xs, y.neg_mod(m), m)
}}

/// Set r to -n mod d. n >= d is allowed. Can give r > d. d cannot equal 0.
///
/// This is equivalent to `NEG_MOD` from `gmp-impl.h`, GMP 6.2.1, where `r` is returned.
const fn quick_neg_mod(n: Limb, d: Limb) -> Limb {
    if n <= d {
        d - n
    } else {
        let d = d << d.leading_zeros();
        (if n <= d { d } else { d << 1 }).wrapping_sub(n)
    }
}

// Interpreting two limbs `x` and `y` and slice of `Limb`s `m` as three numbers x, y, and m,
// determines whether x ≡ -y mod m.
//
// This function assumes that the input slice has at least two elements, its last element is
// nonzero, and `x` and `y` are nonzero.
//
// # Worst-case complexity
// Constant time and additional memory.
//
// This is equivalent to `mpz_congruent_p` from `mpz/cong.c`, GMP 6.2.1, where `a` and `d` are
// positive, `c` is negative, `a` and `d` are one limb long, and `c` is longer than one limb.
pub_const_test! {limbs_pos_limb_eq_neg_limb_mod(x: Limb, y: Limb, ms: &[Limb]) -> bool {
    // We are checking whether x ≡ -y mod m; that is, whether x + y = k * m for some k in Z. But
    // because of the preconditions on m, the lowest possible value of m is `2 ^ Limb::WIDTH`, while
    // the highest possible value of x + y is `2 ^ (Limb::WIDTH + 1) - 2`, so we have x + y < 2 * m.
    // This means that k can only be 1, so we're actually checking whether x + y = m.
    ms.len() == 2 && ms[1] == 1 && {
        let (sum, overflow) = x.overflowing_add(y);
        overflow && sum == ms[0]
    }
}}

#[allow(clippy::absurd_extreme_comparisons)]
fn limbs_pos_eq_neg_limb_mod_helper(xs: &[Limb], y: Limb, ms: &[Limb]) -> Option<bool> {
    let m_len = ms.len();
    let x_len = xs.len();
    assert!(m_len > 1);
    assert!(x_len > 1);
    assert_ne!(y, 0);
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(*ms.last().unwrap(), 0);
    let m_0 = ms[0];
    // Check x == y mod low zero bits of m_0. This might catch a few cases of x != y quickly.
    let twos = TrailingZeros::trailing_zeros(m_0);
    if !xs[0].wrapping_neg().eq_mod_power_of_2(y, twos) {
        return Some(false);
    }
    // m_0 == 0 is avoided since we don't want to bother handling extra low zero bits if m_1 is even
    // (would involve borrow if x_0, y_0 != 0).
    if m_len == 2 && m_0 != 0 {
        let m_1 = ms[1];
        if m_1 < Limb::power_of_2(twos) {
            let m_0 = (m_0 >> twos) | (m_1 << (Limb::WIDTH - twos));
            let y = quick_neg_mod(y, m_0);
            return Some(if x_len >= BMOD_1_TO_MOD_1_THRESHOLD {
                limbs_mod_limb(xs, m_0)
                    == if y < m_0 {
                        y
                    } else {
                        fail_on_untested_path("limbs_pos_eq_neg_limb_mod_helper, y >= m_0");
                        y % m_0
                    }
            } else {
                let r = limbs_mod_exact_odd_limb(xs, m_0, y);
                r == 0 || r == m_0
            });
        }
    }
    None
}

// Interpreting a slice of `Limb`s `xs`, a Limb `y`, and another slice of `Limb`s `m` as three
// numbers x, y, and m, determines whether x ≡ -y mod m. The second input slice is immutable.
//
// This function assumes that each of the two input slices have at least two elements, their last
// elements are nonzero, and `y` is nonzero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if the length of `xs` or `ms` is less than 2, if the last element of either of the slices
// is zero, or if `y` is zero.
//
// This is equivalent to `mpz_congruent_p` from `mpz/cong.c`, GMP 6.2.1, where `a` and `d` are
// positive, `c` is negative, `a` and `d` are longer than one limb, and `c` is one limb long.
pub_test! {limbs_pos_eq_neg_limb_mod_ref(xs: &[Limb], y: Limb, ms: &[Limb]) -> bool {
    if let Some(equal) = limbs_pos_eq_neg_limb_mod_helper(xs, y, ms) {
        return equal;
    }
    // calculate |x - y|. Different signs, add
    let mut scratch = limbs_add_limb(xs, y);
    scratch.len() >= ms.len() && limbs_divisible_by_val_ref(&mut scratch, ms)
}}

// Interpreting a slice of `Limb`s `xs`, a Limb `y`, and another slice of `Limb`s `ms` as three
// numbers x, y, and m, determines whether x ≡ -y mod m. The second input slice is mutable.
//
// This function assumes that each of the two input slices have at least two elements, their last
// elements are nonzero, and `y` is nonzero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// # Panics
// Panics if the length of `xs` or `ms` is less than 2, if the last element of either of the slices
// is zero, or if `y` is zero.
//
// This is equivalent to `mpz_congruent_p` from `mpz/cong.c`, GMP 6.2.1, where `a` and `d` are
// positive, `c` is negative, `a` and `d` are longer than one limb, and `c` is one limb long.
pub_test! {limbs_pos_eq_neg_limb_mod(xs: &[Limb], y: Limb, ms: &mut [Limb]) -> bool {
    if let Some(equal) = limbs_pos_eq_neg_limb_mod_helper(xs, y, ms) {
        return equal;
    }
    // calculate |x - y|. Different signs, add
    let mut scratch = limbs_add_limb(xs, y);
    scratch.len() >= ms.len() && limbs_divisible_by(&mut scratch, ms)
}}

// Interpreting two slices of `Limb`s `xs` and `ys` and a Limb `m` as three numbers x, y, and m,
// determines whether x ≡ -y mod m.
//
// This function assumes that each of the two input slices have at least two elements, their last
// elements are nonzero, and `m` is nonzero.
//
// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if the length of `xs` or `ys` is less than 2, if the last element of either of the slices
// is zero, or if `m` is zero.
//
// This is equivalent to `mpz_congruent_p` from `mpz/cong.c`, GMP 6.2.1, where `a` and `d` are
// positive, `c` is negative, `a` and `c` are longer than one limb, and `m` is one limb long.
pub_test! {limbs_pos_eq_neg_mod_limb(xs: &[Limb], ys: &[Limb], m: Limb) -> bool {
    if xs.len() >= ys.len() {
        limbs_pos_eq_mod_neg_limb_greater(xs, ys, m)
    } else {
        limbs_pos_eq_mod_neg_limb_greater(ys, xs, m)
    }
}}

// xs.len() >= ys.len()
fn limbs_pos_eq_mod_neg_limb_greater(xs: &[Limb], ys: &[Limb], m: Limb) -> bool {
    assert!(xs.len() > 1);
    assert!(ys.len() > 1);
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(*ys.last().unwrap(), 0);
    assert_ne!(m, 0);
    // Check x == y mod low zero bits of m_0. This might catch a few cases of x != y quickly.
    if !xs[0]
        .wrapping_neg()
        .eq_mod_power_of_2(ys[0], TrailingZeros::trailing_zeros(m))
    {
        return false;
    }
    // calculate |x - y|. Different signs, add
    limbs_divisible_by_limb(&limbs_add(xs, ys), m)
}

fn limbs_pos_eq_neg_mod_greater_helper(xs: &[Limb], ys: &[Limb], ms: &[Limb]) -> Option<bool> {
    assert!(ms.len() > 1);
    assert!(xs.len() > 1);
    assert!(ys.len() > 1);
    assert_ne!(*xs.last().unwrap(), 0);
    assert_ne!(*ys.last().unwrap(), 0);
    assert_ne!(*ms.last().unwrap(), 0);
    // Check x == y mod low zero bits of m_0. This might catch a few cases of x != y quickly.
    if xs[0]
        .wrapping_neg()
        .eq_mod_power_of_2(ys[0], TrailingZeros::trailing_zeros(ms[0]))
    {
        None
    } else {
        Some(false)
    }
}

// Interpreting three slice of `Limb`s as the limbs of three `Natural`s, determines whether the
// first `Natural` is equal to the negative of the second `Natural` mod the third `Natural`. The
// second input slice is immutable.
//
// This function assumes that each of the three input slices have at least two elements, and their
// last elements are nonzero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if the length of `xs`, `ys`, or `ms` is less than 2, or if the last element of any of the
// slices is zero.
//
// This is equivalent to `mpz_congruent_p` from `mpz/cong.c`, GMP 6.2.1, where `a` and `d` are
// positive, `c` is negative, and each is longer than one limb.
pub_test! {limbs_pos_eq_neg_mod_ref(xs: &[Limb], ys: &[Limb], ms: &[Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_pos_eq_neg_mod_greater_ref(xs, ys, ms)
    } else {
        limbs_pos_eq_neg_mod_greater_ref(ys, xs, ms)
    }
}}

// xs.len() >= ys.len()
fn limbs_pos_eq_neg_mod_greater_ref(xs: &[Limb], ys: &[Limb], ms: &[Limb]) -> bool {
    if let Some(equal) = limbs_pos_eq_neg_mod_greater_helper(xs, ys, ms) {
        return equal;
    }
    // calculate |x - y|. Different signs, add
    let mut scratch = limbs_add(xs, ys);
    scratch.len() >= ms.len() && limbs_divisible_by_val_ref(&mut scratch, ms)
}

// Interpreting three slice of `Limb`s as the limbs of three `Natural`s, determines whether the
// first `Natural` is equal to the negative of the second `Natural` mod the third `Natural`. The
// second input slice is mutable.
//
// This function assumes that each of the three input slices have at least two elements, and their
// last elements are nonzero.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log \log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `max(xs.len(), ys.len())`.
//
// # Panics
// Panics if the length of `xs`, `ys`, or `ms` is less than 2, or if the last element of any of the
// slices is zero.
//
// This is equivalent to `mpz_congruent_p` from `mpz/cong.c`, GMP 6.2.1, where `a` and `d` are
// positive, `c` is negative, and each is longer than one limb.
pub_test! {limbs_pos_eq_neg_mod(xs: &[Limb], ys: &[Limb], ms: &mut [Limb]) -> bool {
    if xs.len() >= ys.len() {
        limbs_pos_eq_neg_mod_greater(xs, ys, ms)
    } else {
        limbs_pos_eq_neg_mod_greater(ys, xs, ms)
    }
}}

// xs.len() >= ys.len()
fn limbs_pos_eq_neg_mod_greater(xs: &[Limb], ys: &[Limb], ms: &mut [Limb]) -> bool {
    if let Some(equal) = limbs_pos_eq_neg_mod_greater_helper(xs, ys, ms) {
        return equal;
    }
    // calculate |x - y|. Different signs, add
    let mut scratch = limbs_add(xs, ys);
    scratch.len() >= ms.len() && limbs_divisible_by(&mut scratch, ms)
}

impl Natural {
    fn eq_neg_limb_mod_limb(&self, other: Limb, m: Limb) -> bool {
        m != 0
            && match *self {
                Natural(Small(small)) => small % m == other.neg_mod(m),
                Natural(Large(ref limbs)) => limbs_eq_neg_limb_mod_limb(limbs, other, m),
            }
    }

    fn pos_eq_neg_mod(&self, other: &Natural, m: Natural) -> bool {
        match (self, other, m) {
            (_, _, Natural::ZERO) => false,
            (x, &Natural::ZERO, m) => x.divisible_by(m),
            (&Natural::ZERO, y, m) => y.divisible_by(m),
            (x, &Natural(Small(y)), Natural(Small(m))) => x.eq_neg_limb_mod_limb(y, m),
            (&Natural(Small(x)), y, Natural(Small(m))) => y.eq_neg_limb_mod_limb(x, m),
            (&Natural(Small(x)), &Natural(Small(y)), Natural(Large(ref m))) => {
                limbs_pos_limb_eq_neg_limb_mod(x, y, m)
            }
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), Natural(Small(m))) => {
                limbs_pos_eq_neg_mod_limb(xs, ys, m)
            }
            (&Natural(Large(ref xs)), &Natural(Small(y)), Natural(Large(ref mut m))) => {
                limbs_pos_eq_neg_limb_mod(xs, y, m)
            }
            (&Natural(Small(x)), &Natural(Large(ref ys)), Natural(Large(ref mut m))) => {
                limbs_pos_eq_neg_limb_mod(ys, x, m)
            }
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), Natural(Large(ref mut m))) => {
                limbs_pos_eq_neg_mod(xs, ys, m)
            }
        }
    }

    fn pos_eq_neg_mod_ref(&self, other: &Natural, m: &Natural) -> bool {
        match (self, other, m) {
            (_, _, &Natural::ZERO) => false,
            (x, &Natural::ZERO, m) => x.divisible_by(m),
            (&Natural::ZERO, y, m) => y.divisible_by(m),
            (x, &Natural(Small(y)), &Natural(Small(m))) => x.eq_neg_limb_mod_limb(y, m),
            (&Natural(Small(x)), y, &Natural(Small(m))) => y.eq_neg_limb_mod_limb(x, m),
            (&Natural(Small(x)), &Natural(Small(y)), &Natural(Large(ref m))) => {
                limbs_pos_limb_eq_neg_limb_mod(x, y, m)
            }
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), &Natural(Small(m))) => {
                limbs_pos_eq_neg_mod_limb(xs, ys, m)
            }
            (&Natural(Large(ref xs)), &Natural(Small(y)), &Natural(Large(ref m))) => {
                limbs_pos_eq_neg_limb_mod_ref(xs, y, m)
            }
            (&Natural(Small(x)), &Natural(Large(ref ys)), &Natural(Large(ref m))) => {
                limbs_pos_eq_neg_limb_mod_ref(ys, x, m)
            }
            (&Natural(Large(ref xs)), &Natural(Large(ref ys)), &Natural(Large(ref m))) => {
                limbs_pos_eq_neg_mod_ref(xs, ys, m)
            }
        }
    }
}

impl EqMod<Integer, Natural> for Integer {
    /// Returns whether an [`Integer`] is equivalent to another [`Integer`] modulo a [`Natural`];
    /// that is, whether the difference between the two [`Integer`]s is a multiple of the
    /// [`Natural`]. All three numbers are taken by value.
    ///
    /// Two [`Integer`]s are equal to each other modulo 0 iff they are equal.
    ///
    /// $f(x, y, m) = (x \equiv y \mod m)$.
    ///
    /// $f(x, y, m) = (\exists k \in \Z : x - y = km)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Integer::from(123).eq_mod(Integer::from(223), Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from_str("1000000987654").unwrap().eq_mod(
    ///         Integer::from_str("-999999012346").unwrap(),
    ///         Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from_str("1000000987654").unwrap().eq_mod(
    ///         Integer::from_str("2000000987655").unwrap(),
    ///         Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: Integer, m: Natural) -> bool {
        if self.sign == other.sign {
            self.abs.eq_mod(other.abs, m)
        } else {
            self.abs.pos_eq_neg_mod(&other.abs, m)
        }
    }
}

impl EqMod<Integer, &Natural> for Integer {
    /// Returns whether an [`Integer`] is equivalent to another [`Integer`] modulo a [`Natural`];
    /// that is, whether the difference between the two [`Integer`]s is a multiple of the
    /// [`Natural`]. The first two numbers are taken by value and the third by reference.
    ///
    /// Two [`Integer`]s are equal to each other modulo 0 iff they are equal.
    ///
    /// $f(x, y, m) = (x \equiv y \mod m)$.
    ///
    /// $f(x, y, m) = (\exists k \in \Z : x - y = km)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Integer::from(123).eq_mod(Integer::from(223), &Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from_str("1000000987654").unwrap().eq_mod(
    ///         Integer::from_str("-999999012346").unwrap(),
    ///         &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from_str("1000000987654").unwrap().eq_mod(
    ///         Integer::from_str("2000000987655").unwrap(),
    ///         &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: Integer, m: &Natural) -> bool {
        if self.sign == other.sign {
            self.abs.eq_mod(other.abs, m)
        } else {
            self.abs.pos_eq_neg_mod_ref(&other.abs, m)
        }
    }
}

impl EqMod<&Integer, Natural> for Integer {
    /// Returns whether an [`Integer`] is equivalent to another [`Integer`] modulo a [`Natural`];
    /// that is, whether the difference between the two [`Integer`]s is a multiple of the
    /// [`Natural`]. The first and third numbers are taken by value and the second by reference.
    ///
    /// Two [`Integer`]s are equal to each other modulo 0 iff they are equal.
    ///
    /// $f(x, y, m) = (x \equiv y \mod m)$.
    ///
    /// $f(x, y, m) = (\exists k \in \Z : x - y = km)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Integer::from(123).eq_mod(&Integer::from(223), Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from_str("1000000987654").unwrap().eq_mod(
    ///         &Integer::from_str("-999999012346").unwrap(),
    ///         Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from_str("1000000987654").unwrap().eq_mod(
    ///         &Integer::from_str("2000000987655").unwrap(),
    ///         Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: &Integer, m: Natural) -> bool {
        if self.sign == other.sign {
            self.abs.eq_mod(&other.abs, m)
        } else {
            self.abs.pos_eq_neg_mod(&other.abs, m)
        }
    }
}

impl EqMod<&Integer, &Natural> for Integer {
    /// Returns whether an [`Integer`] is equivalent to another [`Integer`] modulo a [`Natural`];
    /// that is, whether the difference between the two [`Integer`]s is a multiple of the
    /// [`Natural`]. The first number is taken by value and the second and third by reference.
    ///
    /// Two [`Integer`]s are equal to each other modulo 0 iff they are equal.
    ///
    /// $f(x, y, m) = (x \equiv y \mod m)$.
    ///
    /// $f(x, y, m) = (\exists k \in \Z : x - y = km)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Integer::from(123).eq_mod(&Integer::from(223), &Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from_str("1000000987654").unwrap().eq_mod(
    ///         &Integer::from_str("-999999012346").unwrap(),
    ///         &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     Integer::from_str("1000000987654").unwrap().eq_mod(
    ///         &Integer::from_str("2000000987655").unwrap(),
    ///         &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: &Integer, m: &Natural) -> bool {
        if self.sign == other.sign {
            self.abs.eq_mod(&other.abs, m)
        } else {
            self.abs.pos_eq_neg_mod_ref(&other.abs, m)
        }
    }
}

impl EqMod<Integer, Natural> for &Integer {
    /// Returns whether an [`Integer`] is equivalent to another [`Integer`] modulo a [`Natural`];
    /// that is, whether the difference between the two [`Integer`]s is a multiple of the
    /// [`Natural`]. The first number is taken by reference and the second and third by value.
    ///
    /// Two [`Integer`]s are equal to each other modulo 0 iff they are equal.
    ///
    /// $f(x, y, m) = (x \equiv y \mod m)$.
    ///
    /// $f(x, y, m) = (\exists k \in \Z : x - y = km)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Integer::from(123)).eq_mod(Integer::from(223), Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("1000000987654").unwrap()).eq_mod(
    ///         Integer::from_str("-999999012346").unwrap(),
    ///         Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("1000000987654").unwrap()).eq_mod(
    ///         Integer::from_str("2000000987655").unwrap(),
    ///         Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: Integer, m: Natural) -> bool {
        if self.sign == other.sign {
            (&self.abs).eq_mod(other.abs, m)
        } else {
            self.abs.pos_eq_neg_mod(&other.abs, m)
        }
    }
}

impl EqMod<Integer, &Natural> for &Integer {
    /// Returns whether an [`Integer`] is equivalent to another [`Integer`] modulo a [`Natural`];
    /// that is, whether the difference between the two [`Integer`]s is a multiple of the
    /// [`Natural`]. The first and third numbers are taken by reference and the third by value.
    ///
    /// Two [`Integer`]s are equal to each other modulo 0 iff they are equal.
    ///
    /// $f(x, y, m) = (x \equiv y \mod m)$.
    ///
    /// $f(x, y, m) = (\exists k \in \Z : x - y = km)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Integer::from(123)).eq_mod(Integer::from(223), &Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("1000000987654").unwrap()).eq_mod(
    ///         Integer::from_str("-999999012346").unwrap(),
    ///         &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("1000000987654").unwrap()).eq_mod(
    ///         Integer::from_str("2000000987655").unwrap(),
    ///         &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: Integer, m: &Natural) -> bool {
        if self.sign == other.sign {
            (&self.abs).eq_mod(other.abs, m)
        } else {
            self.abs.pos_eq_neg_mod_ref(&other.abs, m)
        }
    }
}

impl EqMod<&Integer, Natural> for &Integer {
    /// Returns whether an [`Integer`] is equivalent to another [`Integer`] modulo a [`Natural`];
    /// that is, whether the difference between the two [`Integer`]s is a multiple of the
    /// [`Natural`]. The first two numbers are taken by reference and the third by value.
    ///
    /// Two [`Integer`]s are equal to each other modulo 0 iff they are equal.
    ///
    /// $f(x, y, m) = (x \equiv y \mod m)$.
    ///
    /// $f(x, y, m) = (\exists k \in \Z : x - y = km)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Integer::from(123)).eq_mod(&Integer::from(223), Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("1000000987654").unwrap()).eq_mod(
    ///         &Integer::from_str("-999999012346").unwrap(),
    ///         Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("1000000987654").unwrap()).eq_mod(
    ///         &Integer::from_str("2000000987655").unwrap(),
    ///         Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: &Integer, m: Natural) -> bool {
        if self.sign == other.sign {
            (&self.abs).eq_mod(&other.abs, m)
        } else {
            self.abs.pos_eq_neg_mod(&other.abs, m)
        }
    }
}

impl EqMod<&Integer, &Natural> for &Integer {
    /// Returns whether an [`Integer`] is equivalent to another [`Integer`] modulo a [`Natural`];
    /// that is, whether the difference between the two [`Integer`]s is a multiple of the
    /// [`Natural`]. All three numbers are taken by reference.
    ///
    /// Two [`Integer`]s are equal to each other modulo 0 iff they are equal.
    ///
    /// $f(x, y, m) = (x \equiv y \mod m)$.
    ///
    /// $f(x, y, m) = (\exists k \in \Z : x - y = km)$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log \log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `max(self.significant_bits(),
    /// other.significant_bits())`.
    ///
    /// # Examples
    /// ```
    /// use core::str::FromStr;
    /// use malachite_base::num::arithmetic::traits::EqMod;
    /// use malachite_nz::integer::Integer;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Integer::from(123)).eq_mod(&Integer::from(223), &Natural::from(100u32)),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("1000000987654").unwrap()).eq_mod(
    ///         &Integer::from_str("-999999012346").unwrap(),
    ///         &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     true
    /// );
    /// assert_eq!(
    ///     (&Integer::from_str("1000000987654").unwrap()).eq_mod(
    ///         &Integer::from_str("2000000987655").unwrap(),
    ///         &Natural::from_str("1000000000000").unwrap()
    ///     ),
    ///     false
    /// );
    /// ```
    fn eq_mod(self, other: &Integer, m: &Natural) -> bool {
        if self.sign == other.sign {
            (&self.abs).eq_mod(&other.abs, m)
        } else {
            self.abs.pos_eq_neg_mod_ref(&other.abs, m)
        }
    }
}

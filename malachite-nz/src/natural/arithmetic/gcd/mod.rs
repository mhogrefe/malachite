// Copyright © 2024 Mikhail Hogrefe
//
// Uses code adopted from the GNU MP Library.
//
//      Copyright © 1991-2019 Free Software Foundation, Inc.
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::eq_mod::limbs_mod_exact_odd_limb;
use crate::natural::arithmetic::gcd::half_gcd::limbs_gcd_reduced;
use crate::natural::arithmetic::mod_op::limbs_mod_limb_alt_2;
use crate::natural::arithmetic::shr::limbs_slice_shr_in_place;
use crate::natural::comparison::cmp::limbs_cmp;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{Limb, BMOD_1_TO_MOD_1_THRESHOLD};
use core::cmp::{min, Ordering::*};
use core::mem::swap;
use malachite_base::num::arithmetic::traits::{Gcd, GcdAssign};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::TrailingZeros;
use malachite_base::slices::slice_leading_zeros;

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `ns.len()`.
//
// This is equivalent to `MPN_MOD_OR_MODEXACT_1_ODD` from `gmp-impl.h`, GMP 6.2.1, where `size > 1`.
fn limbs_mod_or_modexact(ns: &[Limb], d: Limb) -> Limb {
    if ns.len() < BMOD_1_TO_MOD_1_THRESHOLD {
        limbs_mod_exact_odd_limb(ns, d, 0)
    } else {
        limbs_mod_limb_alt_2(ns, d)
    }
}

// # Worst-case complexity
// $T(n) = O(n)$
//
// $M(n) = O(1)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
//
// This is equivalent to `mpn_gcd_1` from `mpn/generic/gcd_1.c`, GMP 6.2.1.
pub_test! {limbs_gcd_limb(xs: &[Limb], mut y: Limb) -> Limb {
    assert!(xs.len() > 1);
    assert_ne!(y, 0);
    let mut x = xs[0];
    let mut zeros = y.trailing_zeros();
    y >>= zeros;
    if x != 0 {
        zeros = min(zeros, x.trailing_zeros());
    }
    x = limbs_mod_or_modexact(xs, y);
    if x != 0 {
        y.gcd_assign(x >> x.trailing_zeros());
    }
    y << zeros
}}

// # Worst-case complexity
// $T(n) = O(n (\log n)^2 \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `xs.len()`.
fn gcd_greater_helper(mut xs: &mut [Limb], mut ys: &mut [Limb]) -> Natural {
    let xs_zero_limbs = slice_leading_zeros(xs);
    let ys_zero_limbs = slice_leading_zeros(ys);
    let common_zero_limbs = min(xs_zero_limbs, ys_zero_limbs);
    xs = &mut xs[common_zero_limbs..];
    ys = &mut ys[common_zero_limbs..];
    let xs_zero_bits = TrailingZeros::trailing_zeros(xs[0]);
    let ys_zero_bits = TrailingZeros::trailing_zeros(ys[0]);
    let common_zero_bits = min(xs_zero_bits, ys_zero_bits);
    if common_zero_bits != 0 {
        limbs_slice_shr_in_place(xs, common_zero_bits);
        limbs_slice_shr_in_place(ys, common_zero_bits);
        if *xs.last().unwrap() == 0 {
            let n = xs.len();
            xs = &mut xs[..n - 1];
        }
        if *ys.last().unwrap() == 0 {
            let n = ys.len();
            ys = &mut ys[..n - 1];
        }
    }
    let n = if ys.len() == 1 {
        Natural::from(if xs.len() == 1 {
            xs[0].gcd(ys[0])
        } else {
            limbs_gcd_limb(xs, ys[0])
        })
    } else {
        let mut out = vec![0; xs.len()];
        let out_len = limbs_gcd_reduced(&mut out, xs, ys);
        out.resize(out_len, 0);
        Natural::from_owned_limbs_asc(out)
    };
    n << ((u64::exact_from(common_zero_limbs) << Limb::LOG_WIDTH) + common_zero_bits)
}

impl Gcd<Natural> for Natural {
    type Output = Natural;

    /// Computes the GCD (greatest common divisor) of two [`Natural`]s, taking both by value.
    ///
    /// The GCD of 0 and $n$, for any $n$, is 0. In particular, $\gcd(0, 0) = 0$, which makes sense
    /// if we interpret "greatest" to mean "greatest by the divisibility order".
    ///
    /// $$
    /// f(x, y) = \gcd(x, y).
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
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).gcd(Natural::from(5u32)), 1);
    /// assert_eq!(Natural::from(12u32).gcd(Natural::from(90u32)), 6);
    /// ```
    fn gcd(mut self, other: Natural) -> Natural {
        self.gcd_assign(other);
        self
    }
}

impl<'a> Gcd<&'a Natural> for Natural {
    type Output = Natural;

    /// Computes the GCD (greatest common divisor) of two [`Natural`]s, taking the first by value
    /// and the second by reference.
    ///
    /// The GCD of 0 and $n$, for any $n$, is 0. In particular, $\gcd(0, 0) = 0$, which makes sense
    /// if we interpret "greatest" to mean "greatest by the divisibility order".
    ///
    /// $$
    /// f(x, y) = \gcd(x, y).
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
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(Natural::from(3u32).gcd(&Natural::from(5u32)), 1);
    /// assert_eq!(Natural::from(12u32).gcd(&Natural::from(90u32)), 6);
    /// ```
    #[inline]
    fn gcd(mut self, other: &'a Natural) -> Natural {
        self.gcd_assign(other);
        self
    }
}

impl<'a> Gcd<Natural> for &'a Natural {
    type Output = Natural;

    /// Computes the GCD (greatest common divisor) of two [`Natural`]s, taking the first by
    /// reference and the second by value.
    ///
    /// The GCD of 0 and $n$, for any $n$, is 0. In particular, $\gcd(0, 0) = 0$, which makes sense
    /// if we interpret "greatest" to mean "greatest by the divisibility order".
    ///
    /// $$
    /// f(x, y) = \gcd(x, y).
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
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(3u32)).gcd(Natural::from(5u32)), 1);
    /// assert_eq!((&Natural::from(12u32)).gcd(Natural::from(90u32)), 6);
    /// ```
    #[inline]
    fn gcd(self, mut other: Natural) -> Natural {
        other.gcd_assign(self);
        other
    }
}

impl<'a, 'b> Gcd<&'a Natural> for &'b Natural {
    type Output = Natural;

    /// Computes the GCD (greatest common divisor) of two [`Natural`]s, taking both by reference.
    ///
    /// The GCD of 0 and $n$, for any $n$, is 0. In particular, $\gcd(0, 0) = 0$, which makes sense
    /// if we interpret "greatest" to mean "greatest by the divisibility order".
    ///
    /// $$
    /// f(x, y) = \gcd(x, y).
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
    /// use malachite_base::num::arithmetic::traits::Gcd;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!((&Natural::from(3u32)).gcd(&Natural::from(5u32)), 1);
    /// assert_eq!((&Natural::from(12u32)).gcd(&Natural::from(90u32)), 6);
    /// ```
    #[inline]
    fn gcd(self, other: &'a Natural) -> Natural {
        match (self, other) {
            (x, &Natural::ZERO) => x.clone(),
            (&Natural::ZERO, y) => y.clone(),
            (x, y) if core::ptr::eq(x, y) => x.clone(),
            (Natural(Small(x)), Natural(Small(y))) => Natural::from(x.gcd(*y)),
            (Natural(Large(ref xs)), Natural(Small(y))) => Natural::from(limbs_gcd_limb(xs, *y)),
            (Natural(Small(x)), Natural(Large(ref ys))) => Natural::from(limbs_gcd_limb(ys, *x)),
            (Natural(Large(xs)), Natural(Large(ys))) => {
                let c = limbs_cmp(xs, ys);
                if c == Equal {
                    return self.clone();
                }
                let mut xs = xs.clone();
                let mut xs: &mut [Limb] = &mut xs;
                let mut ys = ys.clone();
                let mut ys: &mut [Limb] = &mut ys;
                if c == Less {
                    swap(&mut xs, &mut ys);
                }
                gcd_greater_helper(xs, ys)
            }
        }
    }
}

impl GcdAssign<Natural> for Natural {
    /// Replaces a [`Natural`] by its GCD (greatest common divisor) with another [`Natural`], taking
    /// the [`Natural`] on the right-hand side by value.
    ///
    /// $$
    /// x \gets \gcd(x, y).
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
    /// use malachite_base::num::arithmetic::traits::GcdAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.gcd_assign(Natural::from(5u32));
    /// assert_eq!(x, 1);
    ///
    /// let mut x = Natural::from(12u32);
    /// x.gcd_assign(Natural::from(90u32));
    /// assert_eq!(x, 6);
    /// ```
    #[inline]
    fn gcd_assign(&mut self, other: Natural) {
        match (&mut *self, other) {
            (_, Natural::ZERO) => {}
            (&mut Natural::ZERO, y) => *self = y,
            (Natural(Small(ref mut x)), Natural(Small(y))) => x.gcd_assign(y),
            (Natural(Large(ref xs)), Natural(Small(y))) => {
                *self = Natural::from(limbs_gcd_limb(xs, y));
            }
            (Natural(Small(x)), Natural(Large(ref ys))) => {
                *self = Natural::from(limbs_gcd_limb(ys, *x));
            }
            (Natural(Large(ref mut xs)), Natural(Large(mut ys))) => {
                let mut xs: &mut [Limb] = &mut *xs;
                let mut ys: &mut [Limb] = &mut ys;
                match limbs_cmp(xs, ys) {
                    Equal => return,
                    Less => {
                        swap(&mut xs, &mut ys);
                    }
                    _ => {}
                }
                *self = gcd_greater_helper(xs, ys);
            }
        }
    }
}

impl<'a> GcdAssign<&'a Natural> for Natural {
    /// Replaces a [`Natural`] by its GCD (greatest common divisor) with another [`Natural`], taking
    /// the [`Natural`] on the right-hand side by reference.
    ///
    /// $$
    /// x \gets \gcd(x, y).
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
    /// use malachite_base::num::arithmetic::traits::GcdAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.gcd_assign(&Natural::from(5u32));
    /// assert_eq!(x, 1);
    ///
    /// let mut x = Natural::from(12u32);
    /// x.gcd_assign(&Natural::from(90u32));
    /// assert_eq!(x, 6);
    /// ```
    #[inline]
    fn gcd_assign(&mut self, other: &'a Natural) {
        match (&mut *self, other) {
            (_, &Natural::ZERO) => {}
            (&mut Natural::ZERO, y) => self.clone_from(y),
            (Natural(Small(ref mut x)), Natural(Small(y))) => x.gcd_assign(*y),
            (Natural(Large(ref xs)), Natural(Small(y))) => {
                *self = Natural::from(limbs_gcd_limb(xs, *y));
            }
            (Natural(Small(x)), Natural(Large(ref ys))) => {
                *self = Natural::from(limbs_gcd_limb(ys, *x));
            }
            (Natural(Large(ref mut xs)), Natural(Large(ys))) => {
                let c = limbs_cmp(xs, ys);
                if c == Equal {
                    return;
                }
                let mut xs: &mut [Limb] = &mut *xs;
                let mut ys = ys.clone();
                let mut ys: &mut [Limb] = &mut ys;
                if c == Less {
                    swap(&mut xs, &mut ys);
                }
                *self = gcd_greater_helper(xs, ys);
            }
        }
    }
}

/// Implementations of [`ExtendedGcd`](malachite_base::num::arithmetic::traits::ExtendedGcd), a
/// trait for computing the extended GCD of two numbers.
pub mod extended_gcd;
/// Code for the half-GCD algorithm, described [here](https://gmplib.org/manual/Subquadratic-GCD).
pub mod half_gcd;
/// Code for working with 2-by-2 matrices.
pub mod matrix_2_2;

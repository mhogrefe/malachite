// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::arithmetic::mod_power_of_2::limbs_vec_mod_power_of_2_in_place;
use crate::natural::arithmetic::mod_power_of_2_square::{
    limbs_mod_power_of_2_square, limbs_mod_power_of_2_square_ref,
};
use crate::natural::arithmetic::mul::limbs_mul;
use crate::natural::arithmetic::mul::mul_low::limbs_mul_low_same_length;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::{DoubleLimb, Limb};
use alloc::vec::Vec;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Assign, ModPowerOf2Mul, ModPowerOf2MulAssign, ShrRound,
};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::*;

// Interpreting two `Vec<Limb>`s as the limbs (in ascending order) of two `Natural`s, returns a
// `Vec` of the limbs of the product of the `Natural`s mod `2 ^ pow`. Assumes the inputs are already
// reduced mod `2 ^ pow`. The input `Vec`s may be mutated. Neither input may be empty or have
// trailing zeros.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
//
// # Panics
// Panics if either input is empty. May panic if either input has trailing zeros.
pub_test! {limbs_mod_power_of_2_mul(xs: &mut Vec<Limb>, ys: &mut Vec<Limb>, pow: u64) -> Vec<Limb> {
    if core::ptr::eq(xs.as_slice(), ys.as_slice()) {
        return limbs_mod_power_of_2_square(xs, pow);
    }
    let xs_len = xs.len();
    assert_ne!(xs_len, 0);
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    let max_len = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, Ceiling).0);
    if max_len > xs_len + ys_len + 1 {
        return limbs_mul(xs, ys);
    }
    // Should really be max_len / sqrt(2); 0.75 * max_len is close enough
    let limit = max_len.checked_mul(3).unwrap() >> 2;
    let mut product = if xs_len >= limit && ys_len >= limit {
        if xs_len != max_len {
            xs.resize(max_len, 0);
        }
        if ys_len != max_len {
            ys.resize(max_len, 0);
        }
        let mut product_limbs = vec![0; max_len];
        limbs_mul_low_same_length(&mut product_limbs, xs, ys);
        product_limbs
    } else {
        limbs_mul(xs, ys)
    };
    limbs_vec_mod_power_of_2_in_place(&mut product, pow);
    product
}}

// Interpreting a slice of `Limb` and a `Vec<Limb>` as the limbs (in ascending order) of two
// `Natural`s, returns a `Vec` of the limbs of the product of the `Natural`s mod `2 ^ pow`. Assumes
// the inputs are already reduced mod `2 ^ pow`. The input `Vec` may be mutated. Neither input may
// be empty or have trailing zeros.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
//
// # Panics
// Panics if either input is empty. May panic if either input has trailing zeros.
pub_test! {limbs_mod_power_of_2_mul_val_ref(
    xs: &mut Vec<Limb>,
    ys: &[Limb],
    pow: u64
) -> Vec<Limb> {
    if core::ptr::eq(xs.as_slice(), ys) {
        return limbs_mod_power_of_2_square(xs, pow);
    }
    let xs_len = xs.len();
    assert_ne!(xs_len, 0);
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    let max_len = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, Ceiling).0);
    if max_len > xs_len + ys_len + 1 {
        return limbs_mul(xs, ys);
    }
    // Should really be max_len / sqrt(2); 0.75 * max_len is close enough
    let limit = max_len.checked_mul(3).unwrap() >> 2;
    let mut product = if xs_len >= limit && ys_len >= limit {
        if xs_len != max_len {
            xs.resize(max_len, 0);
        }
        let mut ys_adjusted_vec;
        let ys_adjusted = if ys_len == max_len {
            ys
        } else {
            ys_adjusted_vec = vec![0; max_len];
            ys_adjusted_vec[..ys_len].copy_from_slice(ys);
            &ys_adjusted_vec
        };
        let mut product = vec![0; max_len];
        limbs_mul_low_same_length(&mut product, xs, ys_adjusted);
        product
    } else {
        limbs_mul(xs, ys)
    };
    limbs_vec_mod_power_of_2_in_place(&mut product, pow);
    product
}}

// Interpreting two slices of `Limb` as the limbs (in ascending order) of two `Natural`s, returns a
// `Vec` of the limbs of the product of the `Natural`s mod `2 ^ pow`. Assumes the inputs are already
// reduced mod `2 ^ pow`. Neither input may be empty or have trailing zeros.
//
// # Worst-case complexity
// $T(n) = O(n \log n \log\log n)$
//
// $M(n) = O(n \log n)$
//
// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
//
// # Panics
// Panics if either input is empty. May panic if either input has trailing zeros.
pub_test! {limbs_mod_power_of_2_mul_ref_ref(xs: &[Limb], ys: &[Limb], pow: u64) -> Vec<Limb> {
    if core::ptr::eq(xs, ys) {
        return limbs_mod_power_of_2_square_ref(xs, pow);
    }
    let xs_len = xs.len();
    assert_ne!(xs_len, 0);
    let ys_len = ys.len();
    assert_ne!(ys_len, 0);
    let max_len = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, Ceiling).0);
    if max_len > xs_len + ys_len + 1 {
        return limbs_mul(xs, ys);
    }
    // Should really be max_len / sqrt(2); 0.75 * max_len is close enough
    let limit = max_len.checked_mul(3).unwrap() >> 2;
    let mut product = if xs_len >= limit && ys_len >= limit {
        let mut xs_adjusted_vec;
        let mut ys_adjusted_vec;
        let xs_adjusted = if xs_len == max_len {
            xs
        } else {
            xs_adjusted_vec = vec![0; max_len];
            xs_adjusted_vec[..xs_len].copy_from_slice(xs);
            &xs_adjusted_vec
        };
        let ys_adjusted = if ys_len == max_len {
            ys
        } else {
            ys_adjusted_vec = vec![0; max_len];
            ys_adjusted_vec[..ys_len].copy_from_slice(ys);
            &ys_adjusted_vec
        };
        let mut product = vec![0; max_len];
        limbs_mul_low_same_length(&mut product, xs_adjusted, ys_adjusted);
        product
    } else {
        limbs_mul(xs, ys)
    };
    limbs_vec_mod_power_of_2_in_place(&mut product, pow);
    product
}}

impl Natural {
    fn mod_power_of_2_mul_limb_ref(&self, y: Limb, pow: u64) -> Natural {
        match (self, y, pow) {
            (_, 0, _) | (&Natural::ZERO, _, _) => Natural::ZERO,
            (_, 1, _) => self.clone(),
            (&Natural::ONE, _, _) => Natural(Small(y)),
            (&Natural(Small(small)), other, pow) if pow <= Limb::WIDTH => {
                Natural(Small(small.mod_power_of_2_mul(other, pow)))
            }
            (&Natural(Small(small)), other, pow) => Natural::from(
                (DoubleLimb::from(small) * DoubleLimb::from(other)).mod_power_of_2(pow),
            ),
            (x, other, pow) => (x * Natural::from(other)).mod_power_of_2(pow),
        }
    }

    fn mod_power_of_2_mul_limb_assign(&mut self, y: Limb, pow: u64) {
        match (&mut *self, y, pow) {
            (_, 1, _) | (&mut Natural::ZERO, _, _) => {}
            (_, 0, _) => *self = Natural::ZERO,
            (&mut Natural::ONE, _, _) => *self = Natural(Small(y)),
            (&mut Natural(Small(ref mut small)), other, pow) if pow <= Limb::WIDTH => {
                small.mod_power_of_2_mul_assign(other, pow);
            }
            (&mut Natural(Small(small)), other, pow) => {
                *self = Natural::from(
                    (DoubleLimb::from(small) * DoubleLimb::from(other)).mod_power_of_2(pow),
                );
            }
            (x, other, pow) => {
                *x *= Natural::from(other);
                x.mod_power_of_2_assign(pow);
            }
        }
    }
}

impl ModPowerOf2Mul<Natural> for Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s modulo $2^k$. The inputs must be already reduced modulo $2^k$.
    /// Both [`Natural`]s are taken by value.
    ///
    /// $f(x, y, k) = z$, where $x, y, z < 2^k$ and $xy \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_power_of_2_mul(Natural::from(2u32), 5),
    ///     6
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).mod_power_of_2_mul(Natural::from(14u32), 4),
    ///     12
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_mul(mut self, other: Natural, pow: u64) -> Natural {
        self.mod_power_of_2_mul_assign(other, pow);
        self
    }
}

impl<'a> ModPowerOf2Mul<&'a Natural> for Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s modulo $2^k$. The inputs must be already reduced modulo $2^k$.
    /// The first [`Natural`] is taken by value and the second by reference.
    ///
    /// $f(x, y, k) = z$, where $x, y, z < 2^k$ and $xy \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_power_of_2_mul(&Natural::from(2u32), 5),
    ///     6
    /// );
    /// assert_eq!(
    ///     Natural::from(10u32).mod_power_of_2_mul(&Natural::from(14u32), 4),
    ///     12
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_mul(mut self, other: &'a Natural, pow: u64) -> Natural {
        self.mod_power_of_2_mul_assign(other, pow);
        self
    }
}

impl<'a> ModPowerOf2Mul<Natural> for &'a Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s modulo $2^k$. The inputs must be already reduced modulo $2^k$.
    /// The first [`Natural`] is taken by reference and the second by value.
    ///
    /// $f(x, y, k) = z$, where $x, y, z < 2^k$ and $xy \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_power_of_2_mul(Natural::from(2u32), 5),
    ///     6
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_power_of_2_mul(Natural::from(14u32), 4),
    ///     12
    /// );
    /// ```
    #[inline]
    fn mod_power_of_2_mul(self, mut other: Natural, pow: u64) -> Natural {
        other.mod_power_of_2_mul_assign(self, pow);
        other
    }
}

impl<'a, 'b> ModPowerOf2Mul<&'b Natural> for &'a Natural {
    type Output = Natural;

    /// Multiplies two [`Natural`]s modulo $2^k$. The inputs must be already reduced modulo $2^k$.
    /// Both [`Natural`]s are taken by reference.
    ///
    /// $f(x, y, k) = z$, where $x, y, z < 2^k$ and $xy \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Mul;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_power_of_2_mul(&Natural::from(2u32), 5),
    ///     6
    /// );
    /// assert_eq!(
    ///     (&Natural::from(10u32)).mod_power_of_2_mul(&Natural::from(14u32), 4),
    ///     12
    /// );
    /// ```
    fn mod_power_of_2_mul(self, other: &'b Natural, pow: u64) -> Natural {
        assert!(
            self.significant_bits() <= pow,
            "self must be reduced mod 2^pow, but {self} >= 2^{pow}"
        );
        assert!(
            other.significant_bits() <= pow,
            "other must be reduced mod 2^pow, but {other} >= 2^{pow}"
        );
        match (self, other) {
            (x, &Natural(Small(y))) => x.mod_power_of_2_mul_limb_ref(y, pow),
            (&Natural(Small(x)), y) => y.mod_power_of_2_mul_limb_ref(x, pow),
            (&Natural(Large(ref xs)), &Natural(Large(ref ys))) => {
                Natural::from_owned_limbs_asc(limbs_mod_power_of_2_mul_ref_ref(xs, ys, pow))
            }
        }
    }
}

impl ModPowerOf2MulAssign<Natural> for Natural {
    /// Multiplies two [`Natural`]s modulo $2^k$, in place. The inputs must be already reduced
    /// modulo $2^k$. The [`Natural`] on the right-hand side is taken by value.
    ///
    /// $x \gets z$, where $x, y, z < 2^k$ and $x + y \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_power_of_2_mul_assign(Natural::from(2u32), 5);
    /// assert_eq!(x, 6);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_power_of_2_mul_assign(Natural::from(14u32), 4);
    /// assert_eq!(x, 12);
    /// ```
    fn mod_power_of_2_mul_assign(&mut self, mut other: Natural, pow: u64) {
        assert!(
            self.significant_bits() <= pow,
            "self must be reduced mod 2^pow, but {self} >= 2^{pow}"
        );
        assert!(
            other.significant_bits() <= pow,
            "other must be reduced mod 2^pow, but {other} >= 2^{pow}"
        );
        match (&mut *self, &mut other) {
            (x, &mut Natural(Small(y))) => x.mod_power_of_2_mul_limb_assign(y, pow),
            (&mut Natural(Small(x)), y) => {
                y.mod_power_of_2_mul_limb_assign(x, pow);
                *self = other;
            }
            (&mut Natural(Large(ref mut xs)), &mut Natural(Large(ref mut ys))) => {
                *xs = limbs_mod_power_of_2_mul(xs, ys, pow);
                self.trim();
            }
        }
    }
}

impl<'a> ModPowerOf2MulAssign<&'a Natural> for Natural {
    /// Multiplies two [`Natural`]s modulo $2^k$, in place. The inputs must be already reduced
    /// modulo $2^k$. The [`Natural`] on the right-hand side is taken by reference.
    ///
    /// $x \gets z$, where $x, y, z < 2^k$ and $x + y \equiv z \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` or `other` are greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2MulAssign;
    /// use malachite_nz::natural::Natural;
    ///
    /// let mut x = Natural::from(3u32);
    /// x.mod_power_of_2_mul_assign(&Natural::from(2u32), 5);
    /// assert_eq!(x, 6);
    ///
    /// let mut x = Natural::from(10u32);
    /// x.mod_power_of_2_mul_assign(&Natural::from(14u32), 4);
    /// assert_eq!(x, 12);
    /// ```
    fn mod_power_of_2_mul_assign(&mut self, other: &'a Natural, pow: u64) {
        assert!(
            self.significant_bits() <= pow,
            "self must be reduced mod 2^pow, but {self} >= 2^{pow}"
        );
        assert!(
            other.significant_bits() <= pow,
            "other must be reduced mod 2^pow, but {other} >= 2^{pow}"
        );
        match (&mut *self, other) {
            (x, &Natural(Small(y))) => x.mod_power_of_2_mul_limb_assign(y, pow),
            (&mut Natural(Small(x)), y) => {
                *self = y.mod_power_of_2_mul_limb_ref(x, pow);
            }
            (&mut Natural(Large(ref mut xs)), &Natural(Large(ref ys))) => {
                *xs = limbs_mod_power_of_2_mul_val_ref(xs, ys, pow);
                self.trim();
            }
        }
    }
}

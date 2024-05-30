// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::conversion::to_twos_complement_limbs::limbs_twos_complement_in_place;
use crate::natural::arithmetic::add::limbs_slice_add_limb_in_place;
use crate::natural::arithmetic::mod_power_of_2::limbs_slice_mod_power_of_2_in_place;
use crate::natural::arithmetic::mul::mul_low::limbs_mul_low_same_length;
use crate::natural::InnerNatural::{Large, Small};
use crate::natural::Natural;
use crate::platform::Limb;
use malachite_base::num::arithmetic::traits::{ModPowerOf2Inverse, Parity, ShrRound};
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::rounding_modes::RoundingMode::*;

// - out should be just long enough for `pow` bits.
// - xs should have the same length as out.
// - scratch should be at least twice as long as out.
// - out should be filled with zeros.
fn limbs_mod_power_of_2_inverse(out: &mut [Limb], xs: &[Limb], pow: u64, scratch: &mut [Limb]) {
    let len = out.len();
    split_into_chunks_mut!(scratch, len, [scratch_0, scratch_1], _unused);
    let mut limb_pow = 1;
    out[0] = xs[0].mod_power_of_2_inverse(Limb::WIDTH).unwrap();
    while limb_pow < len {
        limb_pow <<= 1;
        if limb_pow > len {
            limb_pow = len;
        }
        let out_lo = &mut out[..limb_pow];
        let scratch_0_lo = &mut scratch_0[..limb_pow];
        let scratch_1_lo = &mut scratch_1[..limb_pow];
        limbs_mul_low_same_length(scratch_0_lo, out_lo, &xs[..limb_pow]);
        limbs_twos_complement_in_place(scratch_0_lo);
        limbs_slice_add_limb_in_place(scratch_0_lo, 2);
        limbs_mul_low_same_length(scratch_1_lo, scratch_0_lo, out_lo);
        out_lo.copy_from_slice(scratch_1_lo);
    }
    limbs_slice_mod_power_of_2_in_place(out, pow);
}

#[allow(clippy::unnecessary_wraps)]
fn mod_power_of_2_inverse_helper(xs: &[Limb], pow: u64) -> Option<Natural> {
    let len = xs.len();
    let mut big_scratch = vec![0; len * 3];
    let (out, scratch) = big_scratch.split_at_mut(len);
    limbs_mod_power_of_2_inverse(out, xs, pow, scratch);
    big_scratch.truncate(len);
    Some(Natural::from_owned_limbs_asc(big_scratch))
}

impl ModPowerOf2Inverse for Natural {
    type Output = Natural;

    /// Computes the multiplicative inverse of a [`Natural`] modulo $2^k$. The input must be already
    /// reduced modulo $2^k$. The [`Natural`] is taken by value.
    ///
    /// Returns `None` if $x$ is even.
    ///
    /// $f(x, k) = y$, where $x, y < 2^k$, $x$ is odd, and $xy \equiv 1 \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` is 0 or if `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Inverse;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     Natural::from(3u32).mod_power_of_2_inverse(8),
    ///     Some(Natural::from(171u32))
    /// );
    /// assert_eq!(Natural::from(4u32).mod_power_of_2_inverse(8), None);
    /// ```
    fn mod_power_of_2_inverse(self, pow: u64) -> Option<Natural> {
        assert_ne!(self, 0u32);
        assert!(
            self.significant_bits() <= pow,
            "self must be reduced mod 2^pow, but {self} >= 2^{pow}"
        );
        match (self, pow) {
            (Natural::ONE, _) => Some(Natural::ONE),
            (x, _) if x.even() => None,
            (Natural(Small(x)), pow) if pow <= Limb::WIDTH => {
                x.mod_power_of_2_inverse(pow).map(Natural::from)
            }
            (Natural(Small(x)), pow) => {
                let len = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, Ceiling).0);
                let mut xs = vec![0; len];
                xs[0] = x;
                mod_power_of_2_inverse_helper(&xs, pow)
            }
            (Natural(Large(mut xs)), pow) => {
                let len = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, Ceiling).0);
                xs.resize(len, 0);
                mod_power_of_2_inverse_helper(&xs, pow)
            }
        }
    }
}

impl<'a> ModPowerOf2Inverse for &'a Natural {
    type Output = Natural;

    /// Computes the multiplicative inverse of a [`Natural`] modulo $2^k$. The input must be already
    /// reduced modulo $2^k$. The [`Natural`] is taken by reference.
    ///
    /// Returns `None` if $x$ is even.
    ///
    /// $f(x, k) = y$, where $x, y < 2^k$, $x$ is odd, and $xy \equiv 1 \mod 2^k$.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n \log n \log\log n)$
    ///
    /// $M(n) = O(n \log n)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ is `pow`.
    ///
    /// # Panics
    /// Panics if `self` is 0 or if `self` is greater than or equal to $2^k$.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::arithmetic::traits::ModPowerOf2Inverse;
    /// use malachite_nz::natural::Natural;
    ///
    /// assert_eq!(
    ///     (&Natural::from(3u32)).mod_power_of_2_inverse(8),
    ///     Some(Natural::from(171u32))
    /// );
    /// assert_eq!((&Natural::from(4u32)).mod_power_of_2_inverse(8), None);
    /// ```
    fn mod_power_of_2_inverse(self, pow: u64) -> Option<Natural> {
        assert_ne!(*self, 0u32);
        assert!(
            self.significant_bits() <= pow,
            "self must be reduced mod 2^pow, but {self} >= 2^{pow}"
        );
        match (self, pow) {
            (&Natural::ONE, _) => Some(Natural::ONE),
            (x, _) if x.even() => None,
            (Natural(Small(x)), pow) if pow <= Limb::WIDTH => {
                x.mod_power_of_2_inverse(pow).map(Natural::from)
            }
            (Natural(Small(x)), pow) => {
                let len = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, Ceiling).0);
                let mut xs = vec![0; len];
                xs[0] = *x;
                mod_power_of_2_inverse_helper(&xs, pow)
            }
            (Natural(Large(xs)), pow) => {
                let len = usize::exact_from(pow.shr_round(Limb::LOG_WIDTH, Ceiling).0);
                let mut xs = xs.clone();
                xs.resize(len, 0);
                mod_power_of_2_inverse_helper(&xs, pow)
            }
        }
    }
}

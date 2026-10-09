// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_vector::NaturalVector;
use alloc::vec;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Assign, ModPowerOf2IsReduced, ModPowerOf2Shl, ModPowerOf2ShlAssign,
};
use malachite_base::num::basic::traits::Zero;

fn assert_reduced(v: &NaturalVector, pow: u64) {
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {v} has an element >= 2^{pow}"
    );
}

// The shift amount, or `None` if it is at least `pow`, in which case every element becomes zero.
// `bits` is `None` if it does not fit in a `u64`, so it is larger than any `pow`.
fn shift_below_pow(bits: Option<u64>, pow: u64) -> Option<u64> {
    bits.filter(|&bits| bits < pow)
}

// Shifts every element left by `bits` modulo 2^pow: only the low `pow - bits` bits of each survive.
fn mod_power_of_2_shl_ref(v: &NaturalVector, bits: Option<u64>, pow: u64) -> NaturalVector {
    assert_reduced(v, pow);
    let Some(bits) = shift_below_pow(bits, pow) else {
        return NaturalVector {
            elements: vec![Natural::ZERO; v.elements.len()],
        };
    };
    NaturalVector {
        elements: v
            .elements
            .iter()
            .map(|x| x.mod_power_of_2(pow - bits) << bits)
            .collect(),
    }
}

fn mod_power_of_2_shl_assign(v: &mut NaturalVector, bits: Option<u64>, pow: u64) {
    assert_reduced(v, pow);
    let Some(bits) = shift_below_pow(bits, pow) else {
        v.elements.fill(Natural::ZERO);
        return;
    };
    if bits != 0 {
        for x in &mut v.elements {
            x.mod_power_of_2_assign(pow - bits);
            *x <<= bits;
        }
    }
}

macro_rules! impl_mod_power_of_2_shl_unsigned {
    ($t:ident) => {
        impl ModPowerOf2Shl<$t> for NaturalVector {
            type Output = NaturalVector;

            /// Left-shifts a [`NaturalVector`] (multiplies it by a power of 2) modulo $2^k$, taking
            /// the vector by value. The elements must already be reduced modulo $2^k$.
            ///
            /// Every element is shifted and reduced, and the dimension is unchanged, so elements
            /// can become zero; if `bits` is at least `pow`, every element is zero.
            ///
            /// $$
            /// f(v, m, k) = 2^mv \bmod 2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, k) = O(nk)$
            ///
            /// $M(n, k) = O(k)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $k$ is
            /// `pow`.
            ///
            /// # Panics
            /// Panics if any element of `self` is greater than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl).
            #[inline]
            fn mod_power_of_2_shl(mut self, bits: $t, pow: u64) -> NaturalVector {
                self.mod_power_of_2_shl_assign(bits, pow);
                self
            }
        }

        impl ModPowerOf2Shl<$t> for &NaturalVector {
            type Output = NaturalVector;

            /// Left-shifts a [`NaturalVector`] (multiplies it by a power of 2) modulo $2^k$, taking
            /// the vector by reference. The elements must already be reduced modulo $2^k$.
            ///
            /// See the documentation for the [`ModPowerOf2Shl`] implementation that takes the
            /// vector by value for details.
            ///
            /// # Worst-case complexity
            /// $T(n, k) = O(nk)$
            ///
            /// $M(n, k) = O(nk)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $k$ is
            /// `pow`.
            ///
            /// # Panics
            /// Panics if any element of `self` is greater than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl).
            #[inline]
            fn mod_power_of_2_shl(self, bits: $t, pow: u64) -> NaturalVector {
                mod_power_of_2_shl_ref(self, u64::try_from(bits).ok(), pow)
            }
        }

        impl ModPowerOf2ShlAssign<$t> for NaturalVector {
            /// Left-shifts a [`NaturalVector`] (multiplies it by a power of 2) modulo $2^k$, in
            /// place. The elements must already be reduced modulo $2^k$.
            ///
            /// See the documentation for the [`ModPowerOf2Shl`] implementation that takes the
            /// vector by value for details.
            ///
            /// # Worst-case complexity
            /// $T(n, k) = O(nk)$
            ///
            /// $M(n, k) = O(k)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.dimension()`, and $k$ is
            /// `pow`.
            ///
            /// # Panics
            /// Panics if any element of `self` is greater than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl_assign).
            #[inline]
            fn mod_power_of_2_shl_assign(&mut self, bits: $t, pow: u64) {
                mod_power_of_2_shl_assign(self, u64::try_from(bits).ok(), pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_shl_unsigned);

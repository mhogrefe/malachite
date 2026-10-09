// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModPowerOf2IsReduced, ModPowerOf2Shl, ModPowerOf2ShlAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_vector::UnsignedVector;
use alloc::vec;

fn assert_reduced<T: PrimitiveUnsigned>(v: &UnsignedVector<T>, pow: u64) {
    assert!(
        pow <= T::WIDTH,
        "pow must be at most T::WIDTH, but it is {pow}"
    );
    assert!(
        v.mod_power_of_2_is_reduced(pow),
        "self must be reduced mod 2^pow, but {v} has an element >= 2^{pow}"
    );
}

// The shift amount as a `u64`, or `None` if it is at least `pow`, in which case every element
// becomes zero. `pow` is at most `T::WIDTH`, which is at most 128, so it fits in every unsigned
// type.
fn shift_below_pow<U: PrimitiveUnsigned>(bits: U, pow: u64) -> Option<u64> {
    if bits >= U::exact_from(pow) {
        None
    } else {
        Some(bits.exact_into())
    }
}

// Shifts every element left by `bits` modulo 2^pow: only the low `pow - bits` bits of each survive,
// so the shift cannot overflow.
fn mod_power_of_2_shl_ref<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    v: &UnsignedVector<T>,
    bits: U,
    pow: u64,
) -> UnsignedVector<T> {
    assert_reduced(v, pow);
    let Some(bits) = shift_below_pow(bits, pow) else {
        return UnsignedVector {
            elements: vec![T::ZERO; v.elements.len()],
        };
    };
    UnsignedVector {
        elements: v
            .elements
            .iter()
            .map(|&x| x.mod_power_of_2(pow - bits) << bits)
            .collect(),
    }
}

fn mod_power_of_2_shl_assign<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    v: &mut UnsignedVector<T>,
    bits: U,
    pow: u64,
) {
    assert_reduced(v, pow);
    let Some(bits) = shift_below_pow(bits, pow) else {
        v.elements.fill(T::ZERO);
        return;
    };
    if bits != 0 {
        for x in &mut v.elements {
            *x = x.mod_power_of_2(pow - bits) << bits;
        }
    }
}

macro_rules! impl_mod_power_of_2_shl_unsigned {
    ($t:ident) => {
        impl<T: PrimitiveUnsigned> ModPowerOf2Shl<$t> for UnsignedVector<T> {
            type Output = UnsignedVector<T>;

            /// Left-shifts an [`UnsignedVector`] (multiplies it by a power of 2) modulo $2^k$,
            /// taking the vector by value. The elements must already be reduced modulo $2^k$.
            ///
            /// Every element is shifted and reduced, and the dimension is unchanged, so elements
            /// can become zero; if `bits` is at least `pow`, every element is zero.
            ///
            /// $$
            /// f(v, m, k) = 2^mv \bmod 2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `T::WIDTH`, or if any element of `self` is greater
            /// than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl).
            #[inline]
            fn mod_power_of_2_shl(mut self, bits: $t, pow: u64) -> UnsignedVector<T> {
                self.mod_power_of_2_shl_assign(bits, pow);
                self
            }
        }

        impl<T: PrimitiveUnsigned> ModPowerOf2Shl<$t> for &UnsignedVector<T> {
            type Output = UnsignedVector<T>;

            /// Left-shifts an [`UnsignedVector`] (multiplies it by a power of 2) modulo $2^k$,
            /// taking the vector by reference. The elements must already be reduced modulo $2^k$.
            ///
            /// See the documentation for the [`ModPowerOf2Shl`] implementation that takes the
            /// vector by value for details.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `T::WIDTH`, or if any element of `self` is greater
            /// than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl).
            #[inline]
            fn mod_power_of_2_shl(self, bits: $t, pow: u64) -> UnsignedVector<T> {
                mod_power_of_2_shl_ref(self, bits, pow)
            }
        }

        impl<T: PrimitiveUnsigned> ModPowerOf2ShlAssign<$t> for UnsignedVector<T> {
            /// Left-shifts an [`UnsignedVector`] (multiplies it by a power of 2) modulo $2^k$, in
            /// place. The elements must already be reduced modulo $2^k$.
            ///
            /// See the documentation for the [`ModPowerOf2Shl`] implementation that takes the
            /// vector by value for details.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is `self.dimension()`.
            ///
            /// # Panics
            /// Panics if `pow` is greater than `T::WIDTH`, or if any element of `self` is greater
            /// than or equal to $2^k$.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2_shl#mod_power_of_2_shl_assign).
            #[inline]
            fn mod_power_of_2_shl_assign(&mut self, bits: $t, pow: u64) {
                mod_power_of_2_shl_assign(self, bits, pow);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_shl_unsigned);

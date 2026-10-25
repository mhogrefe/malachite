// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use crate::natural_matrix::arithmetic::mod_mul::{mod_mul_assign_unchecked, mod_mul_unchecked};
use malachite_base::num::arithmetic::traits::{ModIsReduced, ModShl, ModShlAssign};
use malachite_base::num::basic::traits::One;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;

fn assert_reduced(a: &NaturalMatrix, m: &Natural) {
    assert!(
        a.mod_is_reduced(m),
        "self must be reduced mod m, but {a} has an element >= {m}"
    );
}

// Whether shifting changes nothing: a shift by 0, an empty matrix, or a modulus of 1, modulo which
// every element is 0. Otherwise the matrix has an element, which is less than `m`, so `m` is at
// least 2 and 1 is reduced modulo `m`.
fn is_unchanged<T: PrimitiveUnsigned>(a: &NaturalMatrix, bits: T, m: &Natural) -> bool {
    bits == T::ZERO || a.elements.is_empty() || *m == 1u32
}

// Multiplies every element by 2^bits mod m, which is computed once.
fn mod_shl_assign<T: PrimitiveUnsigned>(a: &mut NaturalMatrix, bits: T, m: &Natural)
where
    for<'a> Natural: ModShl<T, &'a Natural, Output = Natural>,
{
    assert_reduced(a, m);
    if !is_unchanged(a, bits, m) {
        mod_mul_assign_unchecked(a, &Natural::ONE.mod_shl(bits, m), m);
    }
}

fn mod_shl_ref<T: PrimitiveUnsigned>(a: &NaturalMatrix, bits: T, m: &Natural) -> NaturalMatrix
where
    for<'a> Natural: ModShl<T, &'a Natural, Output = Natural>,
{
    assert_reduced(a, m);
    if is_unchanged(a, bits, m) {
        return a.clone();
    }
    mod_mul_unchecked(a, &Natural::ONE.mod_shl(bits, m), m)
}

macro_rules! impl_mod_shl_unsigned {
    ($t:ident) => {
        impl ModShl<$t, Natural> for NaturalMatrix {
            type Output = NaturalMatrix;

            /// Left-shifts a [`NaturalMatrix`] (multiplies it by a power of 2) modulo $m$, taking
            /// the matrix by value and the modulus by value. The elements must already be reduced
            /// modulo $m$.
            ///
            /// $2^k \bmod m$ is computed once, and every element is multiplied by it, with the data
            /// for multiplying modulo $m$ also computed once. Since $m$ need not be odd, elements
            /// can become zero; the shape is unchanged.
            ///
            /// $$
            /// f(A, k, m) = 2^kA \bmod m.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(nm \log m \log\log m + km^2)$
            ///
            /// $M(n, m) = O(m \\log m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.height() * self.width()`,
            /// $m$ is `m.significant_bits()`, and $k$ is `bits.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any element of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(mut self, bits: $t, m: Natural) -> NaturalMatrix {
                mod_shl_assign(&mut self, bits, &m);
                self
            }
        }

        impl ModShl<$t, &Natural> for NaturalMatrix {
            type Output = NaturalMatrix;

            /// Left-shifts a [`NaturalMatrix`] (multiplies it by a power of 2) modulo $m$, taking
            /// the matrix by value and the modulus by reference. The elements must already be
            /// reduced modulo $m$.
            ///
            /// See the documentation for the [`ModShl`] implementation that takes everything by
            /// value for details.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(nm \log m \log\log m + km^2)$
            ///
            /// $M(n, m) = O(m \\log m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.height() * self.width()`,
            /// $m$ is `m.significant_bits()`, and $k$ is `bits.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any element of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(mut self, bits: $t, m: &Natural) -> NaturalMatrix {
                mod_shl_assign(&mut self, bits, m);
                self
            }
        }

        impl ModShl<$t, Natural> for &NaturalMatrix {
            type Output = NaturalMatrix;

            /// Left-shifts a [`NaturalMatrix`] (multiplies it by a power of 2) modulo $m$, taking
            /// the matrix by reference and the modulus by value. The elements must already be
            /// reduced modulo $m$.
            ///
            /// See the documentation for the [`ModShl`] implementation that takes everything by
            /// value for details.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(nm \log m \log\log m + km^2)$
            ///
            /// $M(n, m) = O(nm)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.height() * self.width()`,
            /// $m$ is `m.significant_bits()`, and $k$ is `bits.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any element of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(self, bits: $t, m: Natural) -> NaturalMatrix {
                mod_shl_ref(self, bits, &m)
            }
        }

        impl ModShl<$t, &Natural> for &NaturalMatrix {
            type Output = NaturalMatrix;

            /// Left-shifts a [`NaturalMatrix`] (multiplies it by a power of 2) modulo $m$, taking
            /// the matrix by reference and the modulus by reference. The elements must already be
            /// reduced modulo $m$.
            ///
            /// See the documentation for the [`ModShl`] implementation that takes everything by
            /// value for details.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(nm \log m \log\log m + km^2)$
            ///
            /// $M(n, m) = O(nm)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.height() * self.width()`,
            /// $m$ is `m.significant_bits()`, and $k$ is `bits.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any element of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(self, bits: $t, m: &Natural) -> NaturalMatrix {
                mod_shl_ref(self, bits, m)
            }
        }

        impl ModShlAssign<$t, Natural> for NaturalMatrix {
            /// Left-shifts a [`NaturalMatrix`] (multiplies it by a power of 2) modulo $m$, in
            /// place, taking the modulus by value. The elements must already be reduced modulo $m$.
            ///
            /// See the documentation for the [`ModShl`] implementation that takes everything by
            /// value for details.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(nm \log m \log\log m + km^2)$
            ///
            /// $M(n, m) = O(m \\log m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.height() * self.width()`,
            /// $m$ is `m.significant_bits()`, and $k$ is `bits.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any element of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl_assign).
            #[inline]
            fn mod_shl_assign(&mut self, bits: $t, m: Natural) {
                mod_shl_assign(self, bits, &m);
            }
        }

        impl ModShlAssign<$t, &Natural> for NaturalMatrix {
            /// Left-shifts a [`NaturalMatrix`] (multiplies it by a power of 2) modulo $m$, in
            /// place, taking the modulus by reference. The elements must already be reduced modulo
            /// $m$.
            ///
            /// See the documentation for the [`ModShl`] implementation that takes everything by
            /// value for details.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(nm \log m \log\log m + km^2)$
            ///
            /// $M(n, m) = O(m \\log m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.height() * self.width()`,
            /// $m$ is `m.significant_bits()`, and $k$ is `bits.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any element of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl_assign).
            #[inline]
            fn mod_shl_assign(&mut self, bits: $t, m: &Natural) {
                mod_shl_assign(self, bits, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_shl_unsigned);

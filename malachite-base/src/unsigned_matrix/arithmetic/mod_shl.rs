// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{ModIsReduced, ModShl, ModShlAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::unsigned_matrix::UnsignedMatrix;
use crate::unsigned_vector::arithmetic::mod_mul::{mod_mul_assign_unchecked, mod_mul_unchecked};

fn assert_reduced<T: PrimitiveUnsigned>(a: &UnsignedMatrix<T>, m: T) {
    assert!(
        a.mod_is_reduced(&m),
        "self must be reduced mod m, but {a} has an element >= {m}"
    );
}

// Whether shifting changes nothing: a shift by 0, an empty matrix, or a modulus of 1, modulo which
// every element is 0. Otherwise the matrix has an element, which is less than `m`, so `m` is at
// least 2 and 1 is reduced modulo `m`.
fn is_unchanged<T: PrimitiveUnsigned, U: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    bits: U,
    m: T,
) -> bool {
    bits == U::ZERO || a.elements.is_empty() || m == T::ONE
}

// Multiplies every element by 2^bits mod m, which is computed once.
fn mod_shl_assign<T: PrimitiveUnsigned + ModShl<U, T, Output = T>, U: PrimitiveUnsigned>(
    a: &mut UnsignedMatrix<T>,
    bits: U,
    m: T,
) {
    assert_reduced(a, m);
    if !is_unchanged(a, bits, m) {
        mod_mul_assign_unchecked(&mut a.elements, T::ONE.mod_shl(bits, m), m);
    }
}

fn mod_shl_ref<T: PrimitiveUnsigned + ModShl<U, T, Output = T>, U: PrimitiveUnsigned>(
    a: &UnsignedMatrix<T>,
    bits: U,
    m: T,
) -> UnsignedMatrix<T> {
    assert_reduced(a, m);
    if is_unchanged(a, bits, m) {
        return a.clone();
    }
    UnsignedMatrix {
        height: a.height,
        width: a.width,
        elements: mod_mul_unchecked(&a.elements, T::ONE.mod_shl(bits, m), m),
    }
}

macro_rules! impl_mod_shl_unsigned {
    ($t:ident) => {
        impl<T: PrimitiveUnsigned + ModShl<$t, T, Output = T>> ModShl<$t, T> for UnsignedMatrix<T> {
            type Output = UnsignedMatrix<T>;

            /// Left-shifts an [`UnsignedMatrix`] (multiplies it by a power of 2) modulo $m$, taking
            /// the matrix by value. The elements must already be reduced modulo $m$.
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
            /// $T(n, k) = O(n + k)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.elements.len()`, and $k$
            /// is `bits.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any element of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(mut self, bits: $t, m: T) -> UnsignedMatrix<T> {
                mod_shl_assign(&mut self, bits, m);
                self
            }
        }

        impl<T: PrimitiveUnsigned + ModShl<$t, T, Output = T>> ModShl<$t, T>
            for &UnsignedMatrix<T>
        {
            type Output = UnsignedMatrix<T>;

            /// Left-shifts an [`UnsignedMatrix`] (multiplies it by a power of 2) modulo $m$, taking
            /// the matrix by reference. The elements must already be reduced modulo $m$.
            ///
            /// See the documentation for the [`ModShl`] implementation that takes everything by
            /// value for details.
            ///
            /// # Worst-case complexity
            /// $T(n, k) = O(n + k)$
            ///
            /// $M(n) = O(n)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.elements.len()`, and $k$
            /// is `bits.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any element of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl).
            #[inline]
            fn mod_shl(self, bits: $t, m: T) -> UnsignedMatrix<T> {
                mod_shl_ref(self, bits, m)
            }
        }

        impl<T: PrimitiveUnsigned + ModShl<$t, T, Output = T>> ModShlAssign<$t, T>
            for UnsignedMatrix<T>
        {
            /// Left-shifts an [`UnsignedMatrix`] (multiplies it by a power of 2) modulo $m$, in
            /// place. The elements must already be reduced modulo $m$.
            ///
            /// See the documentation for the [`ModShl`] implementation that takes everything by
            /// value for details.
            ///
            /// # Worst-case complexity
            /// $T(n, k) = O(n + k)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.elements.len()`, and $k$
            /// is `bits.significant_bits()`.
            ///
            /// # Panics
            /// Panics if `m` is 0, or if any element of `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_shl#mod_shl_assign).
            #[inline]
            fn mod_shl_assign(&mut self, bits: $t, m: T) {
                mod_shl_assign(self, bits, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_shl_unsigned);

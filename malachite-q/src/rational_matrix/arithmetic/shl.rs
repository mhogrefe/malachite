// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_matrix::RationalMatrix;
use core::ops::{Shl, ShlAssign};

// Shifts every element left by `bits`.
fn shl_ref<T: Copy>(a: &RationalMatrix, bits: T) -> RationalMatrix
where
    for<'a> &'a Rational: Shl<T, Output = Rational>,
{
    RationalMatrix {
        height: a.height,
        width: a.width,
        elements: a.elements.iter().map(|x| x << bits).collect(),
    }
}

// Shifts every element left by `bits`, in place.
fn shl_assign<T: Copy>(a: &mut RationalMatrix, bits: T)
where
    Rational: ShlAssign<T>,
{
    for x in &mut a.elements {
        *x <<= bits;
    }
}

macro_rules! impl_rational_matrix_shl_unsigned {
    ($t:ident) => {
        impl Shl<$t> for RationalMatrix {
            type Output = RationalMatrix;

            /// Left-shifts a [`RationalMatrix`] (multiplies it by a power of 2), taking it by
            /// value. Every element is shifted, and the shape is unchanged.
            ///
            /// $f(A, k) = 2^kA$.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits`, and $k$ is the number of elements.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(mut self, bits: $t) -> RationalMatrix {
                self <<= bits;
                self
            }
        }

        impl Shl<$t> for &RationalMatrix {
            type Output = RationalMatrix;

            /// Left-shifts a [`RationalMatrix`] (multiplies it by a power of 2), taking it by
            /// reference. Every element is shifted, and the shape is unchanged.
            ///
            /// $f(A, k) = 2^kA$.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits`, and $k$ is the number of elements.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> RationalMatrix {
                shl_ref(self, bits)
            }
        }

        impl ShlAssign<$t> for RationalMatrix {
            /// Left-shifts a [`RationalMatrix`] (multiplies it by a power of 2), in place. Every
            /// element is shifted, and the shape is unchanged.
            ///
            /// $A \gets 2^kA$.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits`, and $k$ is the number of elements.
            ///
            /// # Examples
            /// See [here](super::shl#shl_assign).
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                shl_assign(self, bits);
            }
        }
    };
}
apply_to_unsigneds!(impl_rational_matrix_shl_unsigned);

macro_rules! impl_rational_matrix_shl_signed {
    ($t:ident) => {
        impl Shl<$t> for RationalMatrix {
            type Output = RationalMatrix;

            /// Left-shifts a [`RationalMatrix`] (multiplies it by a power of 2), taking it by
            /// value. Every element is shifted, and the shape is unchanged.
            ///
            /// $f(A, k) = 2^kA$.
            ///
            /// A negative `bits` shifts right, dividing every element by $2^{-k}$; each element
            /// stays in lowest terms.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits.unsigned_abs()`, and $k$ is the number of elements.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(mut self, bits: $t) -> RationalMatrix {
                self <<= bits;
                self
            }
        }

        impl Shl<$t> for &RationalMatrix {
            type Output = RationalMatrix;

            /// Left-shifts a [`RationalMatrix`] (multiplies it by a power of 2), taking it by
            /// reference. Every element is shifted, and the shape is unchanged.
            ///
            /// $f(A, k) = 2^kA$.
            ///
            /// A negative `bits` shifts right, dividing every element by $2^{-k}$; each element
            /// stays in lowest terms.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits.unsigned_abs()`, and $k$ is the number of elements.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> RationalMatrix {
                shl_ref(self, bits)
            }
        }

        impl ShlAssign<$t> for RationalMatrix {
            /// Left-shifts a [`RationalMatrix`] (multiplies it by a power of 2), in place. Every
            /// element is shifted, and the shape is unchanged.
            ///
            /// $A \gets 2^kA$.
            ///
            /// A negative `bits` shifts right, dividing every element by $2^{-k}$; each element
            /// stays in lowest terms.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits.unsigned_abs()`, and $k$ is the number of elements.
            ///
            /// # Examples
            /// See [here](super::shl#shl_assign).
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                shl_assign(self, bits);
            }
        }
    };
}
apply_to_signeds!(impl_rational_matrix_shl_signed);

// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::integer_matrix::IntegerMatrix;
use alloc::vec::Vec;
use core::ops::{Shl, ShlAssign};
use malachite_base::matrix::Matrix;

// Shifts every element left by `bits`.
fn shl_ref<T: Copy>(a: &IntegerMatrix, bits: T) -> IntegerMatrix
where
    for<'a> &'a Integer: Shl<T, Output = Integer>,
{
    IntegerMatrix::from_owned_elements(
        a.height(),
        a.width(),
        a.elements.iter().map(|x| x << bits).collect::<Vec<_>>(),
    )
}

// Shifts every element left by `bits`, in place.
fn shl_assign<T: Copy>(a: &mut IntegerMatrix, bits: T)
where
    Integer: ShlAssign<T>,
{
    for x in &mut a.elements {
        *x <<= bits;
    }
}

macro_rules! impl_integer_matrix_shl_unsigned {
    ($t:ident) => {
        impl Shl<$t> for IntegerMatrix {
            type Output = IntegerMatrix;

            /// Left-shifts an [`IntegerMatrix`] (multiplies it by a power of 2), taking it by
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
            ///
            /// This is equivalent to `fmpz_mat_scalar_mul_2exp` from `fmpz_mat/scalar.c`, FLINT
            /// 3.6.0.
            #[inline]
            fn shl(mut self, bits: $t) -> IntegerMatrix {
                self <<= bits;
                self
            }
        }

        impl Shl<$t> for &IntegerMatrix {
            type Output = IntegerMatrix;

            /// Left-shifts an [`IntegerMatrix`] (multiplies it by a power of 2), taking it by
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
            ///
            /// This is equivalent to `fmpz_mat_scalar_mul_2exp` from `fmpz_mat/scalar.c`, FLINT
            /// 3.6.0.
            #[inline]
            fn shl(self, bits: $t) -> IntegerMatrix {
                shl_ref(self, bits)
            }
        }

        impl ShlAssign<$t> for IntegerMatrix {
            /// Left-shifts an [`IntegerMatrix`] (multiplies it by a power of 2), in place. Every
            /// element is shifted, and the shape is unchanged.
            ///
            /// $A \\gets 2^kA$.
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
            ///
            /// This is equivalent to `fmpz_mat_scalar_mul_2exp` from `fmpz_mat/scalar.c`, FLINT
            /// 3.6.0.
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                shl_assign(self, bits);
            }
        }
    };
}
apply_to_unsigneds!(impl_integer_matrix_shl_unsigned);

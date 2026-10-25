// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::natural::Natural;
use crate::natural_matrix::NaturalMatrix;
use core::ops::{Shr, ShrAssign};

// Shifts every element right by `bits`, taking the floor.
fn shr_ref<T: Copy>(a: &NaturalMatrix, bits: T) -> NaturalMatrix
where
    for<'a> &'a Natural: Shr<T, Output = Natural>,
{
    NaturalMatrix {
        height: a.height,
        width: a.width,
        elements: a.elements.iter().map(|x| x >> bits).collect(),
    }
}

// Shifts every element right by `bits`, taking the floor, in place.
fn shr_assign<T: Copy>(a: &mut NaturalMatrix, bits: T)
where
    Natural: ShrAssign<T>,
{
    for x in &mut a.elements {
        *x >>= bits;
    }
}

macro_rules! impl_natural_matrix_shr_unsigned {
    ($t:ident) => {
        impl Shr<$t> for NaturalMatrix {
            type Output = NaturalMatrix;

            /// Right-shifts a [`NaturalMatrix`] (divides it by a power of 2 and takes the floor),
            /// taking it by value. Every element is shifted, and the shape is unchanged.
            ///
            /// $f(A, k) = \lfloor A/2^k \rfloor$. The floor is taken entrywise, rounding toward
            /// negative infinity; to round differently, use
            /// [`entrywise_shr_round`](malachite_base::num::arithmetic::traits::EntrywiseShrRound).
            ///
            /// # Worst-case complexity
            /// $T(n, k) = O(n + k)$
            ///
            /// $M(n, k) = O(n + k)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, and $k$ is the number of elements.
            ///
            /// # Examples
            /// See [here](super::shr#shr).
            ///
            /// This is equivalent to `fmpz_mat_scalar_tdiv_q_2exp` from `fmpz_mat/scalar.c`, FLINT
            /// 3.6.0.
            #[inline]
            fn shr(mut self, bits: $t) -> NaturalMatrix {
                self >>= bits;
                self
            }
        }

        impl Shr<$t> for &NaturalMatrix {
            type Output = NaturalMatrix;

            /// Right-shifts a [`NaturalMatrix`] (divides it by a power of 2 and takes the floor),
            /// taking it by reference. Every element is shifted, and the shape is unchanged.
            ///
            /// $f(A, k) = \lfloor A/2^k \rfloor$. The floor is taken entrywise, rounding toward
            /// negative infinity; to round differently, use
            /// [`entrywise_shr_round`](malachite_base::num::arithmetic::traits::EntrywiseShrRound).
            ///
            /// # Worst-case complexity
            /// $T(n, k) = O(n + k)$
            ///
            /// $M(n, k) = O(n + k)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, and $k$ is the number of elements.
            ///
            /// # Examples
            /// See [here](super::shr#shr).
            ///
            /// This is equivalent to `fmpz_mat_scalar_tdiv_q_2exp` from `fmpz_mat/scalar.c`, FLINT
            /// 3.6.0.
            #[inline]
            fn shr(self, bits: $t) -> NaturalMatrix {
                shr_ref(self, bits)
            }
        }

        impl ShrAssign<$t> for NaturalMatrix {
            /// Right-shifts a [`NaturalMatrix`] (divides it by a power of 2 and takes the floor),
            /// in place. Every element is shifted, and the shape is unchanged.
            ///
            /// $A \gets \lfloor A/2^k \rfloor$. The floor is taken entrywise, rounding toward
            /// negative infinity; to round differently, use
            /// [`entrywise_shr_round`](malachite_base::num::arithmetic::traits::EntrywiseShrRound).
            ///
            /// # Worst-case complexity
            /// $T(n, k) = O(n + k)$
            ///
            /// $M(k) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, and $k$ is the number of elements.
            ///
            /// # Examples
            /// See [here](super::shr#shr_assign).
            ///
            /// This is equivalent to `fmpz_mat_scalar_tdiv_q_2exp` from `fmpz_mat/scalar.c`, FLINT
            /// 3.6.0.
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                shr_assign(self, bits);
            }
        }
    };
}
apply_to_unsigneds!(impl_natural_matrix_shr_unsigned);

macro_rules! impl_natural_matrix_shr_signed {
    ($t:ident) => {
        impl Shr<$t> for NaturalMatrix {
            type Output = NaturalMatrix;

            /// Right-shifts a [`NaturalMatrix`] (divides or multiplies it by a power of 2), taking
            /// it by value. Every element is shifted, and the shape is unchanged.
            ///
            /// $f(A, k) = \lfloor A/2^k \rfloor$. The floor is taken entrywise, rounding toward
            /// negative infinity; to round differently, use
            /// [`entrywise_shr_round`](malachite_base::num::arithmetic::traits::EntrywiseShrRound).
            ///
            /// A negative `bits` shifts left, multiplying every element by $2^{-k}$.
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
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(mut self, bits: $t) -> NaturalMatrix {
                self >>= bits;
                self
            }
        }

        impl Shr<$t> for &NaturalMatrix {
            type Output = NaturalMatrix;

            /// Right-shifts a [`NaturalMatrix`] (divides or multiplies it by a power of 2), taking
            /// it by reference. Every element is shifted, and the shape is unchanged.
            ///
            /// $f(A, k) = \lfloor A/2^k \rfloor$. The floor is taken entrywise, rounding toward
            /// negative infinity; to round differently, use
            /// [`entrywise_shr_round`](malachite_base::num::arithmetic::traits::EntrywiseShrRound).
            ///
            /// A negative `bits` shifts left, multiplying every element by $2^{-k}$.
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
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(self, bits: $t) -> NaturalMatrix {
                shr_ref(self, bits)
            }
        }

        impl ShrAssign<$t> for NaturalMatrix {
            /// Right-shifts a [`NaturalMatrix`] (divides or multiplies it by a power of 2), in
            /// place. Every element is shifted, and the shape is unchanged.
            ///
            /// $A \gets \lfloor A/2^k \rfloor$. The floor is taken entrywise, rounding toward
            /// negative infinity; to round differently, use
            /// [`entrywise_shr_round`](malachite_base::num::arithmetic::traits::EntrywiseShrRound).
            ///
            /// A negative `bits` shifts left, multiplying every element by $2^{-k}$.
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
            /// See [here](super::shr#shr_assign).
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                shr_assign(self, bits);
            }
        }
    };
}
apply_to_signeds!(impl_natural_matrix_shr_signed);

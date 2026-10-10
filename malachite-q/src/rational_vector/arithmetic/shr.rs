// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use crate::rational_vector::RationalVector;
use core::ops::{Shr, ShrAssign};

// Shifts every element right by `bits`.
fn shr_ref<T: Copy>(v: &RationalVector, bits: T) -> RationalVector
where
    for<'a> &'a Rational: Shr<T, Output = Rational>,
{
    RationalVector {
        elements: v.elements.iter().map(|x| x >> bits).collect(),
    }
}

// Shifts every element right by `bits`, in place.
fn shr_assign<T: Copy>(v: &mut RationalVector, bits: T)
where
    Rational: ShrAssign<T>,
{
    for x in &mut v.elements {
        *x >>= bits;
    }
}

macro_rules! impl_rational_vector_shr_unsigned {
    ($t:ident) => {
        impl Shr<$t> for RationalVector {
            type Output = RationalVector;

            /// Right-shifts a [`RationalVector`] (divides it by a power of 2), taking it by value.
            /// Every element is shifted, and the dimension is unchanged.
            ///
            /// $f(v, k) = v/2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits`, and $k$ is `self.dimension()`.
            ///
            /// # Examples
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(mut self, bits: $t) -> RationalVector {
                self >>= bits;
                self
            }
        }

        impl Shr<$t> for &RationalVector {
            type Output = RationalVector;

            /// Right-shifts a [`RationalVector`] (divides it by a power of 2), taking it by
            /// reference. Every element is shifted, and the dimension is unchanged.
            ///
            /// $f(v, k) = v/2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits`, and $k$ is `self.dimension()`.
            ///
            /// # Examples
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(self, bits: $t) -> RationalVector {
                shr_ref(self, bits)
            }
        }

        impl ShrAssign<$t> for RationalVector {
            /// Right-shifts a [`RationalVector`] (divides it by a power of 2), in place. Every
            /// element is shifted, and the dimension is unchanged.
            ///
            /// $v \gets v/2^k$.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits`, and $k$ is `self.dimension()`.
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
apply_to_unsigneds!(impl_rational_vector_shr_unsigned);

macro_rules! impl_rational_vector_shr_signed {
    ($t:ident) => {
        impl Shr<$t> for RationalVector {
            type Output = RationalVector;

            /// Right-shifts a [`RationalVector`] (divides it by a power of 2), taking it by value.
            /// Every element is shifted, and the dimension is unchanged.
            ///
            /// $f(v, k) = v/2^k$.
            ///
            /// A negative `bits` shifts left, multiplying every element by $2^{-k}$.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits.unsigned_abs()`, and $k$ is `self.dimension()`.
            ///
            /// # Examples
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(mut self, bits: $t) -> RationalVector {
                self >>= bits;
                self
            }
        }

        impl Shr<$t> for &RationalVector {
            type Output = RationalVector;

            /// Right-shifts a [`RationalVector`] (divides it by a power of 2), taking it by
            /// reference. Every element is shifted, and the dimension is unchanged.
            ///
            /// $f(v, k) = v/2^k$.
            ///
            /// A negative `bits` shifts left, multiplying every element by $2^{-k}$.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits.unsigned_abs()`, and $k$ is `self.dimension()`.
            ///
            /// # Examples
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(self, bits: $t) -> RationalVector {
                shr_ref(self, bits)
            }
        }

        impl ShrAssign<$t> for RationalVector {
            /// Right-shifts a [`RationalVector`] (divides it by a power of 2), in place. Every
            /// element is shifted, and the dimension is unchanged.
            ///
            /// $v \gets v/2^k$.
            ///
            /// A negative `bits` shifts left, multiplying every element by $2^{-k}$.
            ///
            /// # Worst-case complexity
            /// $T(n, m, k) = O(n + km)$
            ///
            /// $M(n, m, k) = O(n + km)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is the total number of bits of the
            /// elements, $m$ is `bits.unsigned_abs()`, and $k$ is `self.dimension()`.
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
apply_to_signeds!(impl_rational_vector_shr_signed);

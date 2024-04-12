// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Rational;
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};
use malachite_base::num::arithmetic::traits::UnsignedAbs;
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::ExactFrom;

fn shl_unsigned_assign<T>(x: &mut Rational, bits: T)
where
    u64: TryFrom<T>,
{
    if *x == 0u32 {
        return;
    }
    let denominator_zeros = x.denominator.trailing_zeros().unwrap();
    let bits_64 = u64::exact_from(bits);
    if denominator_zeros >= bits_64 {
        x.denominator >>= bits_64;
    } else {
        x.denominator >>= denominator_zeros;
        x.numerator <<= bits_64 - denominator_zeros;
    }
}

fn shl_unsigned_ref<T>(x: &Rational, bits: T) -> Rational
where
    u64: TryFrom<T>,
{
    if *x == 0u32 {
        return x.clone();
    }
    let denominator_zeros = x.denominator.trailing_zeros().unwrap();
    let bits_64 = u64::exact_from(bits);
    if denominator_zeros >= bits_64 {
        Rational {
            sign: x.sign,
            numerator: x.numerator.clone(),
            denominator: &x.denominator >> bits_64,
        }
    } else {
        Rational {
            sign: x.sign,
            numerator: &x.numerator << (bits_64 - denominator_zeros),
            denominator: &x.denominator >> denominator_zeros,
        }
    }
}

macro_rules! impl_shl_unsigned {
    ($t:ident) => {
        impl Shl<$t> for Rational {
            type Output = Rational;

            /// Left-shifts a [`Rational`] (multiplies it by a power of 2), taking it by value.
            ///
            /// $$
            /// f(x, k) = x2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `bits`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(mut self, bits: $t) -> Rational {
                self <<= bits;
                self
            }
        }

        impl<'a> Shl<$t> for &'a Rational {
            type Output = Rational;

            /// Left-shifts a [`Rational`] (multiplies it by a power of 2), taking it by value.
            ///
            /// $$
            /// f(x, k) = x2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `bits`.
            ///
            /// # Examples
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> Rational {
                shl_unsigned_ref(self, bits)
            }
        }

        impl ShlAssign<$t> for Rational {
            /// Left-shifts a [`Rational`] (multiplies it by a power of 2), in place.
            ///
            /// $$
            /// x \gets x2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `bits`.
            ///
            /// # Examples
            /// See [here](super::shl#shl_assign).
            #[inline]
            fn shl_assign(&mut self, bits: $t) {
                shl_unsigned_assign(self, bits);
            }
        }
    };
}
apply_to_unsigneds!(impl_shl_unsigned);

fn shl_signed_ref<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &'a Rational,
    bits: S,
) -> Rational
where
    &'a Rational: Shl<U, Output = Rational> + Shr<U, Output = Rational>,
{
    if bits >= S::ZERO {
        x << bits.unsigned_abs()
    } else {
        x >> bits.unsigned_abs()
    }
}

fn shl_assign_signed<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(x: &mut Rational, bits: S)
where
    Rational: ShlAssign<U> + ShrAssign<U>,
{
    if bits >= S::ZERO {
        *x <<= bits.unsigned_abs();
    } else {
        *x >>= bits.unsigned_abs();
    }
}

macro_rules! impl_shl_signed {
    ($t:ident) => {
        impl Shl<$t> for Rational {
            type Output = Rational;

            /// Left-shifts a [`Rational`] (multiplies it or divides it by a power of 2), taking it
            /// by value.
            ///
            /// $$
            /// f(x, k) = x2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(bits, 0)`.
            ///
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(mut self, bits: $t) -> Rational {
                self <<= bits;
                self
            }
        }

        impl<'a> Shl<$t> for &'a Rational {
            type Output = Rational;

            /// Left-shifts a [`Rational`] (multiplies or divides it by a power of 2), taking it by
            /// reference.
            ///
            /// $$
            /// f(x, k) = x2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(bits, 0)`.
            ///
            /// See [here](super::shl#shl).
            #[inline]
            fn shl(self, bits: $t) -> Rational {
                shl_signed_ref(self, bits)
            }
        }

        impl ShlAssign<$t> for Rational {
            /// Left-shifts a [`Rational`] (multiplies or divides it by a power of 2), in place.
            ///
            /// $$
            /// x \gets x2^k.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `bits`.
            ///
            /// See [here](super::shl#shl_assign).
            fn shl_assign(&mut self, bits: $t) {
                shl_assign_signed(self, bits);
            }
        }
    };
}
apply_to_signeds!(impl_shl_signed);

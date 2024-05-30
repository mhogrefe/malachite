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

fn shr_unsigned_assign<T>(x: &mut Rational, bits: T)
where
    u64: TryFrom<T>,
{
    if *x == 0u32 {
        return;
    }
    let numerator_zeros = x.numerator.trailing_zeros().unwrap();
    let bits_64 = u64::exact_from(bits);
    if numerator_zeros >= bits_64 {
        x.numerator >>= bits_64;
    } else {
        x.denominator <<= bits_64 - numerator_zeros;
        x.numerator >>= numerator_zeros;
    }
}

fn shr_unsigned_ref<T>(x: &Rational, bits: T) -> Rational
where
    u64: TryFrom<T>,
{
    if *x == 0u32 {
        return x.clone();
    }
    let numerator_zeros = x.numerator.trailing_zeros().unwrap();
    let bits_64 = u64::exact_from(bits);
    if numerator_zeros >= bits_64 {
        Rational {
            sign: x.sign,
            numerator: &x.numerator >> bits_64,
            denominator: x.denominator.clone(),
        }
    } else {
        Rational {
            sign: x.sign,
            numerator: &x.numerator >> numerator_zeros,
            denominator: &x.denominator << (bits_64 - numerator_zeros),
        }
    }
}

macro_rules! impl_shr_unsigned {
    ($t:ident) => {
        impl Shr<$t> for Rational {
            type Output = Rational;

            /// Right-shifts a [`Rational`] (divides it by a power of 2), taking it by value.
            ///
            /// $$
            /// f(x, k) = \frac{x}{2^k}.
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
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(mut self, bits: $t) -> Rational {
                self >>= bits;
                self
            }
        }

        impl<'a> Shr<$t> for &'a Rational {
            type Output = Rational;

            /// Right-shifts a [`Rational`] (divides it by a power of 2), taking it by reference.
            ///
            /// $$
            /// f(x, k) = \frac{x}{2^k}.
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
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(self, bits: $t) -> Rational {
                shr_unsigned_ref(self, bits)
            }
        }

        impl ShrAssign<$t> for Rational {
            /// Right-shifts a [`Rational`] (divides it by a power of 2), in place.
            ///
            /// $$
            /// f(x, k) = \frac{x}{2^k}.
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
            /// See [here](super::shr#shr_assign).
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                shr_unsigned_assign(self, bits);
            }
        }
    };
}
apply_to_unsigneds!(impl_shr_unsigned);

fn shr_signed_ref<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &'a Rational,
    bits: S,
) -> Rational
where
    &'a Rational: Shl<U, Output = Rational> + Shr<U, Output = Rational>,
{
    if bits >= S::ZERO {
        x >> bits.unsigned_abs()
    } else {
        x << bits.unsigned_abs()
    }
}

fn shr_assign_signed<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(x: &mut Rational, bits: S)
where
    Rational: ShlAssign<U> + ShrAssign<U>,
{
    if bits >= S::ZERO {
        *x >>= bits.unsigned_abs();
    } else {
        *x <<= bits.unsigned_abs();
    }
}

macro_rules! impl_shr_signed {
    ($t:ident) => {
        impl Shr<$t> for Rational {
            type Output = Rational;

            /// Right-shifts a [`Rational`] (divides it by a power of 2), taking it by value.
            ///
            /// $$
            /// f(x, k) = \frac{x}{2^k}.
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
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(mut self, bits: $t) -> Rational {
                self >>= bits;
                self
            }
        }

        impl<'a> Shr<$t> for &'a Rational {
            type Output = Rational;

            /// Right-shifts a [`Rational`] (divides it by a power of 2), taking it by reference.
            ///
            /// $$
            /// f(x, k) = \frac{x}{2^k}.
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
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(self, bits: $t) -> Rational {
                shr_signed_ref(self, bits)
            }
        }

        impl ShrAssign<$t> for Rational {
            /// Right-shifts a [`Rational`] (divides it by a power of 2), in reference.
            ///
            /// $$
            /// f(x, k) = \frac{x}{2^k}.
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
            /// See [here](super::shr#shr_assign).
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                shr_assign_signed(self, bits)
            }
        }
    };
}
apply_to_signeds!(impl_shr_signed);

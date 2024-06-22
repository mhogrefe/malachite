// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::Float;
use crate::InnerFloat::Finite;
use core::ops::{Shr, ShrAssign};
use malachite_base::num::conversion::traits::ExactFrom;

fn shr_primitive_int_assign<T>(x: &mut Float, bits: T)
where
    i32: TryFrom<T>,
{
    if let Float(Finite { exponent, .. }) = x {
        *exponent = exponent.checked_sub(i32::exact_from(bits)).unwrap();
    }
}

fn shr_primitive_int_ref<T>(x: &Float, bits: T) -> Float
where
    i32: TryFrom<T>,
{
    match x {
        Float(Finite {
            sign,
            exponent,
            precision,
            significand,
        }) => Float(Finite {
            sign: *sign,
            exponent: exponent.checked_sub(i32::exact_from(bits)).unwrap(),
            precision: *precision,
            significand: significand.clone(),
        }),
        f => f.clone(),
    }
}

macro_rules! impl_shr {
    ($t:ident) => {
        impl Shr<$t> for Float {
            type Output = Float;

            /// Left-shifts a [`Float`] (multiplies it by a power of 2), taking it by value.
            ///
            /// `NaN`, infinities, and zeros are unchanged.
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
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(mut self, bits: $t) -> Float {
                self >>= bits;
                self
            }
        }

        impl<'a> Shr<$t> for &'a Float {
            type Output = Float;

            /// Left-shifts a [`Float`] (multiplies it by a power of 2), taking it by value.
            ///
            /// `NaN`, infinities, and zeros are unchanged.
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
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(self, bits: $t) -> Float {
                shr_primitive_int_ref(self, bits)
            }
        }

        impl ShrAssign<$t> for Float {
            /// Left-shifts a [`Float`] (multiplies it by a power of 2), in place.
            ///
            /// `NaN`, infinities, and zeros are unchanged.
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
            /// See [here](super::shr#shr_assign).
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                shr_primitive_int_assign(self, bits);
            }
        }
    };
}
apply_to_primitive_ints!(impl_shr);

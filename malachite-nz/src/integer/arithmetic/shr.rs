// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use core::ops::{Shl, ShlAssign, Shr, ShrAssign};
use malachite_base::num::arithmetic::traits::{ShrRound, ShrRoundAssign, UnsignedAbs};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode::*;

fn shr_unsigned_ref<'a, T>(x: &'a Integer, bits: T) -> Integer
where
    &'a Natural: Shr<T, Output = Natural> + ShrRound<T, Output = Natural>,
{
    match *x {
        Integer {
            sign: true,
            ref abs,
        } => Integer {
            sign: true,
            abs: abs >> bits,
        },
        Integer {
            sign: false,
            ref abs,
        } => {
            let abs_shifted = abs.shr_round(bits, Ceiling).0;
            if abs_shifted == 0 {
                Integer::ZERO
            } else {
                Integer {
                    sign: false,
                    abs: abs_shifted,
                }
            }
        }
    }
}

fn shr_assign_unsigned<T>(x: &mut Integer, bits: T)
where
    Natural: ShrAssign<T> + ShrRoundAssign<T>,
{
    match *x {
        Integer {
            sign: true,
            ref mut abs,
        } => {
            *abs >>= bits;
        }
        Integer {
            sign: false,
            ref mut abs,
        } => {
            abs.shr_round_assign(bits, Ceiling);
            if *abs == 0 {
                x.sign = true;
            }
        }
    }
}

macro_rules! impl_shr_unsigned {
    ($t:ident) => {
        impl Shr<$t> for Integer {
            type Output = Integer;

            /// Right-shifts an [`Integer`] (divides it by a power of 2 and takes the floor), taking
            /// it by value.
            ///
            /// $$
            /// f(x, k) = \left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory and $n$ is `max(1,
            /// self.significant_bits() - bits)`.
            ///
            /// # Examples
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(mut self, bits: $t) -> Integer {
                self >>= bits;
                self
            }
        }

        impl<'a> Shr<$t> for &Integer {
            type Output = Integer;

            /// Right-shifts an [`Integer`] (divides it by a power of 2 and takes the floor), taking
            /// it by reference.
            ///
            /// $$
            /// f(x, k) = \left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory and $n$ is `max(1,
            /// self.significant_bits() - bits)`.
            ///
            /// # Examples
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(self, bits: $t) -> Integer {
                shr_unsigned_ref(self, bits)
            }
        }

        impl ShrAssign<$t> for Integer {
            /// Right-shifts an [`Integer`] (divides it by a power of 2 and takes the floor), in
            /// place.
            ///
            /// $$
            /// x \gets \left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory and $n$ is `max(1,
            /// self.significant_bits() - bits)`.
            ///
            /// # Examples
            /// See [here](super::shr#shr_assign).
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                shr_assign_unsigned(self, bits);
            }
        }
    };
}
apply_to_unsigneds!(impl_shr_unsigned);

fn shr_signed_ref<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &'a Integer,
    bits: S,
) -> Integer
where
    &'a Integer: Shl<U, Output = Integer> + Shr<U, Output = Integer>,
{
    if bits >= S::ZERO {
        x >> bits.unsigned_abs()
    } else {
        x << bits.unsigned_abs()
    }
}

fn shr_assign_signed<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(x: &mut Integer, bits: S)
where
    Integer: ShlAssign<U> + ShrAssign<U>,
{
    if bits >= S::ZERO {
        *x >>= bits.unsigned_abs();
    } else {
        *x <<= bits.unsigned_abs();
    }
}

macro_rules! impl_shr_signed {
    ($t:ident) => {
        impl Shr<$t> for Integer {
            type Output = Integer;

            /// Right-shifts an [`Integer`] (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2), taking it by value.
            ///
            /// $$
            /// f(x, k) = \left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory and $n$ is `max(1,
            /// self.significant_bits() - bits)`.
            ///
            /// # Examples
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(mut self, bits: $t) -> Integer {
                self >>= bits;
                self
            }
        }

        impl<'a> Shr<$t> for &Integer {
            type Output = Integer;

            /// Right-shifts an [`Integer`] (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2), taking it by reference.
            ///
            /// $$
            /// f(x, k) = \left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory and $n$ is `max(1,
            /// self.significant_bits() - bits)`.
            ///
            /// # Examples
            /// See [here](super::shr#shr).
            #[inline]
            fn shr(self, bits: $t) -> Integer {
                shr_signed_ref(self, bits)
            }
        }

        impl ShrAssign<$t> for Integer {
            /// Right-shifts an [`Integer`] (divides it by a power of 2 and takes the floor or
            /// multiplies it by a power of 2), in place.
            ///
            /// $$
            /// x \gets \left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory and $n$ is `max(1,
            /// self.significant_bits() - bits)`.
            ///
            /// # Examples
            /// See [here](super::shr#shr_assign).
            #[inline]
            fn shr_assign(&mut self, bits: $t) {
                shr_assign_signed(self, bits)
            }
        }
    };
}
apply_to_signeds!(impl_shr_signed);

// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    CeilingModPowerOf2, CeilingModPowerOf2Assign, ModPowerOf2, ModPowerOf2Assign, NegModPowerOf2,
    NegModPowerOf2Assign, RemPowerOf2, RemPowerOf2Assign,
};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use core::fmt::Debug;

const ERROR_MESSAGE: &str = "Result exceeds width of output type";

fn mod_power_of_2_unsigned<T: PrimitiveUnsigned>(x: T, pow: u64) -> T {
    if x == T::ZERO || pow >= T::WIDTH {
        x
    } else {
        x & T::low_mask(pow)
    }
}

fn mod_power_of_2_assign_unsigned<T: PrimitiveUnsigned>(x: &mut T, pow: u64) {
    if *x != T::ZERO && pow < T::WIDTH {
        *x &= T::low_mask(pow);
    }
}

#[inline]
fn neg_mod_power_of_2_unsigned<T: PrimitiveUnsigned>(x: T, pow: u64) -> T {
    assert!(x == T::ZERO || pow <= T::WIDTH, "{ERROR_MESSAGE}");
    x.wrapping_neg().mod_power_of_2(pow)
}

macro_rules! impl_mod_power_of_2_unsigned {
    ($s:ident) => {
        impl ModPowerOf2 for $s {
            type Output = $s;

            /// Divides a number by $2^k$, returning just the remainder.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k +
            /// r$ and $0 \leq r < 2^k$.
            ///
            /// $$
            /// f(x, k) = x - 2^k\left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2#mod_power_of_2).
            #[inline]
            fn mod_power_of_2(self, pow: u64) -> $s {
                mod_power_of_2_unsigned(self, pow)
            }
        }

        impl ModPowerOf2Assign for $s {
            /// Divides a number by $2^k$, replacing the first number by the remainder.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k +
            /// r$ and $0 \leq r < 2^k$.
            ///
            /// $$
            /// x \gets x - 2^k\left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2#mod_power_of_2_assign).
            #[inline]
            fn mod_power_of_2_assign(&mut self, pow: u64) {
                mod_power_of_2_assign_unsigned(self, pow);
            }
        }

        impl RemPowerOf2 for $s {
            type Output = $s;

            /// Divides a number by $2^k$, returning just the remainder. For unsigned integers,
            /// `rem_power_of_2` is equivalent to [`mod_power_of_2`](super::traits::ModPowerOf2).
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k +
            /// r$ and $0 \leq r < 2^k$.
            ///
            /// $$
            /// f(x, k) = x - 2^k\left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2#rem_power_of_2).
            #[inline]
            fn rem_power_of_2(self, pow: u64) -> $s {
                self.mod_power_of_2(pow)
            }
        }

        impl RemPowerOf2Assign for $s {
            /// Divides a number by $2^k$, replacing the first number by the remainder. For unsigned
            /// integers, `rem_power_of_2_assign` is equivalent to
            /// [`mod_power_of_2_assign`](super::traits::ModPowerOf2Assign).
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k +
            /// r$ and $0 \leq r < 2^k$.
            ///
            /// $$
            /// x \gets x - 2^k\left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2#rem_power_of_2_assign).
            #[inline]
            fn rem_power_of_2_assign(&mut self, pow: u64) {
                self.mod_power_of_2_assign(pow)
            }
        }

        impl NegModPowerOf2 for $s {
            type Output = $s;

            /// Divides the negative of a number by a $2^k$, returning just the remainder.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k -
            /// r$ and $0 \leq r < 2^k$.
            ///
            /// $$
            /// f(x, k) = 2^k\left \lceil \frac{x}{2^k} \right \rceil - x.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is nonzero and `pow` is greater than `Self::WIDTH`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2#neg_mod_power_of_2).
            #[inline]
            fn neg_mod_power_of_2(self, pow: u64) -> $s {
                neg_mod_power_of_2_unsigned(self, pow)
            }
        }

        impl NegModPowerOf2Assign for $s {
            /// Divides the negative of a number by $2^k$, returning just the remainder.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k -
            /// r$ and $0 \leq r < 2^k$.
            ///
            /// $$
            /// x \gets 2^k\left \lceil \frac{x}{2^k} \right \rceil - x.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is nonzero and `pow` is greater than `Self::WIDTH`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2#neg_mod_power_of_2_assign).
            #[inline]
            fn neg_mod_power_of_2_assign(&mut self, pow: u64) {
                *self = self.neg_mod_power_of_2(pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_unsigned);

fn mod_power_of_2_signed<U: PrimitiveUnsigned + WrappingFrom<S>, S: PrimitiveSigned>(
    x: S,
    pow: u64,
) -> U {
    assert!(x >= S::ZERO || pow <= S::WIDTH, "{ERROR_MESSAGE}");
    U::wrapping_from(x).mod_power_of_2(pow)
}

fn mod_power_of_2_assign_signed<U, S: TryFrom<U> + ModPowerOf2<Output = U> + PrimitiveSigned>(
    x: &mut S,
    pow: u64,
) where
    <S as TryFrom<U>>::Error: Debug,
{
    *x = S::try_from(x.mod_power_of_2(pow)).expect(ERROR_MESSAGE);
}

fn rem_power_of_2_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    x: S,
    pow: u64,
) -> S {
    if x >= S::ZERO {
        S::wrapping_from(U::wrapping_from(x).mod_power_of_2(pow))
    } else {
        S::wrapping_from(U::wrapping_from(x.wrapping_neg()).mod_power_of_2(pow)).wrapping_neg()
    }
}

fn ceiling_mod_power_of_2_signed<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: TryFrom<U> + PrimitiveSigned,
>(
    x: S,
    pow: u64,
) -> S
where
    <S as TryFrom<U>>::Error: Debug,
{
    let abs_result = if x >= S::ZERO {
        U::wrapping_from(x).neg_mod_power_of_2(pow)
    } else {
        U::wrapping_from(x.wrapping_neg()).mod_power_of_2(pow)
    };
    S::try_from(abs_result)
        .expect(ERROR_MESSAGE)
        .checked_neg()
        .expect(ERROR_MESSAGE)
}

macro_rules! impl_mod_power_of_2_signed {
    ($u:ident, $s:ident) => {
        impl ModPowerOf2 for $s {
            type Output = $u;

            /// Divides a number by $2^k$, returning just the remainder. The remainder is
            /// non-negative.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k +
            /// r$ and $0 \leq r < 2^k$.
            ///
            /// $$
            /// f(x, k) = x - 2^k\left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is negative and `pow` is greater than `Self::WIDTH`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2#mod_power_of_2).
            #[inline]
            fn mod_power_of_2(self, pow: u64) -> $u {
                mod_power_of_2_signed(self, pow)
            }
        }

        impl ModPowerOf2Assign for $s {
            /// Divides a number by $2^k$, replacing the first number by the remainder. The
            /// remainder is non-negative.
            ///
            /// If the quotient were computed, he quotient and remainder would satisfy $x = q2^k +
            /// r$ and $0 \leq r < 2^k$.
            ///
            /// $$
            /// x \gets x - 2^k\left \lfloor \frac{x}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is negative and `pow` is greater than or equal to `Self::WIDTH`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2#mod_power_of_2_assign).
            #[inline]
            fn mod_power_of_2_assign(&mut self, pow: u64) {
                mod_power_of_2_assign_signed(self, pow);
            }
        }

        impl RemPowerOf2 for $s {
            type Output = $s;

            /// Divides a number by $2^k$, returning just the remainder. The remainder has the same
            /// sign as the first number.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k +
            /// r$ and $0 \leq |r| < 2^k$.
            ///
            /// $$
            /// f(x, k) = x - 2^k\operatorname{sgn}(x)\left \lfloor \frac{|x|}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2#rem_power_of_2).
            #[inline]
            fn rem_power_of_2(self, pow: u64) -> $s {
                rem_power_of_2_signed::<$u, $s>(self, pow)
            }
        }

        impl RemPowerOf2Assign for $s {
            /// Divides a number by $2^k$, replacing the first number by the remainder. The
            /// remainder has the same sign as the first number.
            ///
            /// If the quotient were computed, he quotient and remainder would satisfy $x = q2^k +
            /// r$ and $0 \leq r < 2^k$.
            ///
            /// $$
            /// x \gets x - 2^k\operatorname{sgn}(x)\left \lfloor \frac{|x|}{2^k} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2#rem_power_of_2_assign).
            #[inline]
            fn rem_power_of_2_assign(&mut self, pow: u64) {
                *self = self.rem_power_of_2(pow)
            }
        }

        impl CeilingModPowerOf2 for $s {
            type Output = $s;

            /// Divides a number by $2^k$, returning just the remainder. The remainder is
            /// non-positive.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k +
            /// r$ and $0 \leq -r < 2^k$.
            ///
            /// $$
            /// f(x, y) =  x - 2^k\left \lceil \frac{x}{2^k} \right \rceil.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is positive or `Self::MIN`, and `pow` is greater than or equal to
            /// `Self::WIDTH`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2#ceiling_mod_power_of_2).
            #[inline]
            fn ceiling_mod_power_of_2(self, pow: u64) -> $s {
                ceiling_mod_power_of_2_signed::<$u, $s>(self, pow)
            }
        }

        impl CeilingModPowerOf2Assign for $s {
            /// Divides a number by $2^k$, replacing the first number by the remainder. The
            /// remainder is non-positive.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = q2^k +
            /// r$ and $0 \leq -r < 2^k$.
            ///
            /// $$
            /// x \gets x - 2^k\left \lceil\frac{x}{2^k} \right \rceil.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` is positive or `Self::MIN`, and `pow` is greater than or equal to
            /// `Self::WIDTH`.
            ///
            /// # Examples
            /// See [here](super::mod_power_of_2#ceiling_mod_power_of_2_assign).
            #[inline]
            fn ceiling_mod_power_of_2_assign(&mut self, pow: u64) {
                *self = self.ceiling_mod_power_of_2(pow)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_mod_power_of_2_signed);

// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    CeilingMod, CeilingModAssign, Mod, ModAssign, NegMod, NegModAssign, UnsignedAbs,
};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;

fn neg_mod_unsigned<T: PrimitiveUnsigned>(x: T, other: T) -> T {
    let remainder = x % other;
    if remainder == T::ZERO {
        T::ZERO
    } else {
        other - remainder
    }
}

fn neg_mod_assign_unsigned<T: PrimitiveUnsigned>(x: &mut T, other: T) {
    *x %= other;
    if *x != T::ZERO {
        *x = other - *x;
    }
}

macro_rules! impl_mod_unsigned {
    ($t:ident) => {
        impl Mod<$t> for $t {
            type Output = $t;

            /// Divides a number by another number, returning just the remainder.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$
            /// and $0 \leq r < y$.
            ///
            /// $$
            /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor.
            /// $$
            ///
            /// This function is called `mod_op` rather than `mod` because `mod` is a Rust keyword.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::mod_op#mod_op).
            #[inline]
            fn mod_op(self, other: $t) -> $t {
                self % other
            }
        }

        impl ModAssign<$t> for $t {
            /// Divides a number by another number, replacing the first number by the remainder.
            ///
            /// If the quotient were computed, he quotient and remainder would satisfy $x = qy + r$
            /// and $0 \leq r < y$.
            ///
            /// $$
            /// x \gets x - y\left \lfloor \frac{x}{y} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::mod_op#mod_assign).
            #[inline]
            fn mod_assign(&mut self, other: $t) {
                *self %= other;
            }
        }

        impl NegMod<$t> for $t {
            type Output = $t;

            /// Divides the negative of a number by another number, returning just the remainder.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = qy - r$
            /// and $0 \leq r < y$.
            ///
            /// $$
            /// f(x, y) = y\left \lceil \frac{x}{y} \right \rceil - x.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::mod_op#neg_mod).
            #[inline]
            fn neg_mod(self, other: $t) -> $t {
                neg_mod_unsigned(self, other)
            }
        }

        impl NegModAssign<$t> for $t {
            /// Divides the negative of a number by another number, returning just the remainder.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = qy - r$
            /// and $0 \leq r < y$.
            ///
            /// $$
            /// x \gets y\left \lceil \frac{x}{y} \right \rceil - x.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::mod_op#neg_mod_assign).
            #[inline]
            fn neg_mod_assign(&mut self, other: $t) {
                neg_mod_assign_unsigned(self, other);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_unsigned);

fn mod_op_signed<
    U: PrimitiveUnsigned,
    S: ExactFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: S,
    other: S,
) -> S {
    let remainder = if (x >= S::ZERO) == (other >= S::ZERO) {
        x.unsigned_abs() % other.unsigned_abs()
    } else {
        x.unsigned_abs().neg_mod(other.unsigned_abs())
    };
    if other >= S::ZERO {
        S::exact_from(remainder)
    } else {
        -S::exact_from(remainder)
    }
}

fn ceiling_mod_signed<
    U: PrimitiveUnsigned,
    S: ExactFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: S,
    other: S,
) -> S {
    let remainder = if (x >= S::ZERO) == (other >= S::ZERO) {
        x.unsigned_abs().neg_mod(other.unsigned_abs())
    } else {
        x.unsigned_abs() % other.unsigned_abs()
    };
    if other >= S::ZERO {
        -S::exact_from(remainder)
    } else {
        S::exact_from(remainder)
    }
}

macro_rules! impl_mod_signed {
    ($t:ident) => {
        impl Mod<$t> for $t {
            type Output = $t;

            /// Divides a number by another number, returning just the remainder. The remainder has
            /// the same sign as the second number.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$
            /// and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor.
            /// $$
            ///
            /// This function is called `mod_op` rather than `mod` because `mod` is a Rust keyword.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::mod_op#mod_op).
            #[inline]
            fn mod_op(self, other: $t) -> $t {
                mod_op_signed(self, other)
            }
        }

        impl ModAssign<$t> for $t {
            /// Divides a number by another number, replacing the first number by the remainder. The
            /// remainder has the same sign as the second number.
            ///
            /// If the quotient were computed, he quotient and remainder would satisfy $x = qy + r$
            /// and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// x \gets x - y\left \lfloor \frac{x}{y} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::mod_op#mod_assign).
            #[inline]
            fn mod_assign(&mut self, other: $t) {
                *self = self.mod_op(other);
            }
        }

        impl CeilingMod<$t> for $t {
            type Output = $t;

            /// Divides a number by another number, returning just the remainder. The remainder has
            /// the opposite sign as the second number.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$
            /// and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// f(x, y) =  x - y\left \lceil \frac{x}{y} \right \rceil.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::mod_op#ceiling_mod).
            #[inline]
            fn ceiling_mod(self, other: $t) -> $t {
                ceiling_mod_signed(self, other)
            }
        }

        impl CeilingModAssign<$t> for $t {
            /// Divides a number by another number, replacing the first number by the remainder. The
            /// remainder has the opposite sign as the second number.
            ///
            /// If the quotient were computed, the quotient and remainder would satisfy $x = qy + r$
            /// and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// x \gets x - y\left \lceil\frac{x}{y} \right \rceil.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::mod_op#ceiling_mod_assign).
            #[inline]
            fn ceiling_mod_assign(&mut self, other: $t) {
                *self = self.ceiling_mod(other);
            }
        }
    };
}
apply_to_signeds!(impl_mod_signed);

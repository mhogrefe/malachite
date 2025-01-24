// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{
    CeilingDivAssignMod, CeilingDivAssignNegMod, CeilingDivMod, CeilingDivNegMod, DivAssignMod,
    DivAssignRem, DivMod, DivRem, UnsignedAbs,
};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};

fn div_mod_unsigned<T: PrimitiveUnsigned>(x: T, other: T) -> (T, T) {
    let q = x / other;
    (q, x - q * other)
}

fn div_assign_mod_unsigned<T: PrimitiveUnsigned>(x: &mut T, other: T) -> T {
    let original = *x;
    *x /= other;
    original - *x * other
}

fn ceiling_div_neg_mod_unsigned<T: PrimitiveUnsigned>(x: T, other: T) -> (T, T) {
    let (quotient, remainder) = x.div_mod(other);
    if remainder == T::ZERO {
        (quotient, T::ZERO)
    } else {
        // Here remainder != 0, so other > 1, so quotient < T::MAX.
        (quotient + T::ONE, other - remainder)
    }
}

fn ceiling_div_assign_neg_mod_unsigned<T: PrimitiveUnsigned>(x: &mut T, other: T) -> T {
    let remainder = x.div_assign_mod(other);
    if remainder == T::ZERO {
        T::ZERO
    } else {
        // Here remainder != 0, so other > 1, so self < T::MAX.
        *x += T::ONE;
        other - remainder
    }
}

macro_rules! impl_div_mod_unsigned {
    ($t:ident) => {
        impl DivMod<$t> for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            /// Divides a number by another number, returning the quotient and remainder. The
            /// quotient is rounded towards negative infinity.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
            ///
            /// $$
            /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
            /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_mod).
            #[inline]
            fn div_mod(self, other: $t) -> ($t, $t) {
                div_mod_unsigned(self, other)
            }
        }

        impl DivAssignMod<$t> for $t {
            type ModOutput = $t;

            /// Divides a number by another number in place, returning the remainder. The quotient
            /// is rounded towards negative infinity.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
            ///
            /// $$
            /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor,
            /// $$
            /// $$
            /// x \gets \left \lfloor \frac{x}{y} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_assign_mod).
            #[inline]
            fn div_assign_mod(&mut self, other: $t) -> $t {
                div_assign_mod_unsigned(self, other)
            }
        }

        impl DivRem<$t> for $t {
            type DivOutput = $t;
            type RemOutput = $t;

            /// Divides a number by another number, returning the quotient and remainder. The
            /// quotient is rounded towards zero.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
            ///
            /// $$
            /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
            /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
            /// $$
            ///
            /// For unsigned integers, `div_rem` is equivalent to `div_mod`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_rem).
            #[inline]
            fn div_rem(self, other: $t) -> ($t, $t) {
                self.div_mod(other)
            }
        }

        impl DivAssignRem<$t> for $t {
            type RemOutput = $t;

            /// Divides a number by another number in place, returning the remainder. The quotient
            /// is rounded towards zero.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq r < y$.
            ///
            /// $$
            /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor,
            /// $$
            /// $$
            /// x \gets \left \lfloor \frac{x}{y} \right \rfloor.
            /// $$
            ///
            /// For unsigned integers, `div_assign_rem` is equivalent to `div_assign_mod`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_assign_rem).
            #[inline]
            fn div_assign_rem(&mut self, other: $t) -> $t {
                self.div_assign_mod(other)
            }
        }

        impl CeilingDivNegMod<$t> for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            /// Divides a number by another number, returning the ceiling of the quotient and the
            /// remainder of the negative of the first number divided by the second.
            ///
            /// The quotient and remainder satisfy $x = qy - r$ and $0 \leq r < y$.
            ///
            /// $$
            /// f(x, y) = \left ( \left \lceil \frac{x}{y} \right \rceil, \space
            /// y\left \lceil \frac{x}{y} \right \rceil - x \right ).
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::div_mod#ceiling_div_neg_mod).
            #[inline]
            fn ceiling_div_neg_mod(self, other: $t) -> ($t, $t) {
                ceiling_div_neg_mod_unsigned(self, other)
            }
        }

        impl CeilingDivAssignNegMod<$t> for $t {
            type ModOutput = $t;

            /// Divides a number by another number in place, returning the remainder of the negative
            /// of the first number divided by the second.
            ///
            /// The quotient and remainder satisfy $x = qy - r$ and $0 \leq r < y$.
            ///
            /// $$
            /// f(x, y) = y\left \lceil \frac{x}{y} \right \rceil - x,
            /// $$
            /// $$
            /// x \gets \left \lceil \frac{x}{y} \right \rceil.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0.
            ///
            /// # Examples
            /// See [here](super::div_mod#ceiling_div_assign_neg_mod).
            #[inline]
            fn ceiling_div_assign_neg_mod(&mut self, other: $t) -> $t {
                ceiling_div_assign_neg_mod_unsigned(self, other)
            }
        }
    };
}
apply_to_unsigneds!(impl_div_mod_unsigned);

fn div_mod_signed<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + ExactFrom<U> + UnsignedAbs<Output = U> + WrappingFrom<U>,
>(
    x: S,
    other: S,
) -> (S, S) {
    let (quotient, remainder) = if (x >= S::ZERO) == (other >= S::ZERO) {
        let (quotient, remainder) = x.unsigned_abs().div_mod(other.unsigned_abs());
        (S::exact_from(quotient), remainder)
    } else {
        let (quotient, remainder) = x.unsigned_abs().ceiling_div_neg_mod(other.unsigned_abs());
        (S::wrapping_from(quotient).wrapping_neg(), remainder)
    };
    (
        quotient,
        if other >= S::ZERO {
            S::exact_from(remainder)
        } else {
            -S::exact_from(remainder)
        },
    )
}

fn div_rem_signed<T: PrimitiveSigned>(x: T, other: T) -> (T, T) {
    let q = x.checked_div(other).unwrap();
    (q, x - q * other)
}

fn div_assign_rem_signed<T: PrimitiveSigned>(x: &mut T, other: T) -> T {
    let original = *x;
    *x = x.checked_div(other).unwrap();
    original - *x * other
}

fn ceiling_div_mod_signed<
    U: PrimitiveUnsigned,
    T: PrimitiveSigned + ExactFrom<U> + UnsignedAbs<Output = U> + WrappingFrom<U>,
>(
    x: T,
    other: T,
) -> (T, T) {
    let (quotient, remainder) = if (x >= T::ZERO) == (other >= T::ZERO) {
        let (quotient, remainder) = x.unsigned_abs().ceiling_div_neg_mod(other.unsigned_abs());
        (T::exact_from(quotient), remainder)
    } else {
        let (quotient, remainder) = x.unsigned_abs().div_mod(other.unsigned_abs());
        (T::wrapping_from(quotient).wrapping_neg(), remainder)
    };
    (
        quotient,
        if other >= T::ZERO {
            -T::exact_from(remainder)
        } else {
            T::exact_from(remainder)
        },
    )
}

macro_rules! impl_div_mod_signed {
    ($t:ident) => {
        impl DivMod<$t> for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            /// Divides a number by another number, returning the quotient and remainder. The
            /// quotient is rounded towards negative infinity, and the remainder has the same sign
            /// as the second number.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// f(x, y) = \left ( \left \lfloor \frac{x}{y} \right \rfloor, \space
            /// x - y\left \lfloor \frac{x}{y} \right \rfloor \right ).
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_mod).
            #[inline]
            fn div_mod(self, other: $t) -> ($t, $t) {
                div_mod_signed(self, other)
            }
        }

        impl DivAssignMod<$t> for $t {
            type ModOutput = $t;

            /// Divides a number by another number in place, returning the remainder. The quotient
            /// is rounded towards negative infinity, and the remainder has the same sign as the
            /// second number.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// f(x, y) = x - y\left \lfloor \frac{x}{y} \right \rfloor,
            /// $$
            /// $$
            /// x \gets \left \lfloor \frac{x}{y} \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_assign_mod).
            #[inline]
            fn div_assign_mod(&mut self, other: $t) -> $t {
                let (q, r) = self.div_mod(other);
                *self = q;
                r
            }
        }

        impl DivRem<$t> for $t {
            type DivOutput = $t;
            type RemOutput = $t;

            /// Divides a number by another number, returning the quotient and remainder. The
            /// quotient is rounded towards zero and the remainder has the same sign as the
            /// dividend.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// f(x, y) = \left ( \operatorname{sgn}(xy) \left \lfloor \left | \frac{x}{y} \right |
            /// \right \rfloor, \space
            /// x - y \operatorname{sgn}(xy)
            /// \left \lfloor \left | \frac{x}{y} \right | \right \rfloor \right ).
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_rem).
            #[inline]
            fn div_rem(self, other: $t) -> ($t, $t) {
                div_rem_signed(self, other)
            }
        }

        impl DivAssignRem<$t> for $t {
            type RemOutput = $t;

            /// Divides a number by another number in place, returning the remainder. The quotient
            /// is rounded towards zero and the remainder has the same sign as the dividend.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// f(x, y) = x - y \operatorname{sgn}(xy)
            /// \left \lfloor \left | \frac{x}{y} \right | \right \rfloor,
            /// $$
            /// $$
            /// x \gets \operatorname{sgn}(xy) \left \lfloor \left | \frac{x}{y} \right |
            /// \right \rfloor.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// See [here](super::div_mod#div_assign_rem).
            #[inline]
            fn div_assign_rem(&mut self, other: $t) -> $t {
                div_assign_rem_signed(self, other)
            }
        }

        impl CeilingDivMod<$t> for $t {
            type DivOutput = $t;
            type ModOutput = $t;

            /// Divides a number by another number, returning the quotient and remainder. The
            /// quotient is rounded towards positive infinity and the remainder has the opposite
            /// sign as the second number.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// f(x, y) = \left ( \left \lceil \frac{x}{y} \right \rceil, \space
            /// x - y\left \lceil \frac{x}{y} \right \rceil \right ).
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// See [here](super::div_mod#ceiling_div_mod).
            #[inline]
            fn ceiling_div_mod(self, other: $t) -> ($t, $t) {
                ceiling_div_mod_signed(self, other)
            }
        }

        impl CeilingDivAssignMod<$t> for $t {
            type ModOutput = $t;

            /// Divides a number by another number in place, returning the remainder. The quotient
            /// is rounded towards positive infinity and the remainder has the opposite sign as the
            /// second number.
            ///
            /// The quotient and remainder satisfy $x = qy + r$ and $0 \leq |r| < |y|$.
            ///
            /// $$
            /// f(x, y) = x - y\left \lceil\frac{x}{y} \right \rceil,
            /// $$
            /// $$
            /// x \gets \left \lceil \frac{x}{y} \right \rceil.
            /// $$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is 0, or if `self` is `$t::MIN` and `other` is -1.
            ///
            /// # Examples
            /// See [here](super::div_mod#ceiling_div_assign_mod).
            #[inline]
            fn ceiling_div_assign_mod(&mut self, other: $t) -> $t {
                let (q, r) = self.ceiling_div_mod(other);
                *self = q;
                r
            }
        }
    };
}
apply_to_signeds!(impl_div_mod_signed);

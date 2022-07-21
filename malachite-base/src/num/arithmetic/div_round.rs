use crate::num::arithmetic::traits::{DivRound, DivRoundAssign, UnsignedAbs};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::{ExactFrom, WrappingFrom};
use crate::rounding_modes::RoundingMode;

fn div_round_unsigned<T: PrimitiveUnsigned>(x: T, other: T, rm: RoundingMode) -> T {
    let quotient = x / other;
    if rm == RoundingMode::Down || rm == RoundingMode::Floor {
        quotient
    } else {
        let remainder = x - quotient * other;
        match rm {
            _ if remainder == T::ZERO => quotient,
            RoundingMode::Up | RoundingMode::Ceiling => quotient + T::ONE,
            RoundingMode::Nearest => {
                let shifted_other = other >> 1;
                if remainder > shifted_other
                    || remainder == shifted_other && other.even() && quotient.odd()
                {
                    quotient + T::ONE
                } else {
                    quotient
                }
            }
            RoundingMode::Exact => {
                panic!("Division is not exact: {} / {}", x, other);
            }
            _ => unreachable!(),
        }
    }
}

macro_rules! impl_div_round_unsigned {
    ($t:ident) => {
        impl DivRound<$t> for $t {
            type Output = $t;

            /// Divides a value by another value and rounds according to a specified rounding mode.
            ///
            /// Let $q = \frac{x}{y}$:
            ///
            /// $$
            /// f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
            /// $$
            ///
            /// $$
            /// f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
            /// $$
            ///
            /// $$
            /// f(x, y, \mathrm{Nearest}) = \begin{cases}
            ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
            ///     \lceil q \rceil & \text{if} \\quad  q - \lfloor q \rfloor > \frac{1}{2}, \\\\
            ///     \lfloor q \rfloor &
            ///     \text{if} \\quad  q - \lfloor q \rfloor = \frac{1}{2}
            ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is even}, \\\\
            ///     \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
            ///     \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
            /// \end{cases}
            /// $$
            ///
            /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by
            /// `other`.
            ///
            /// # Examples
            /// See [here](super::div_round#div_round).
            #[inline]
            fn div_round(self, other: $t, rm: RoundingMode) -> $t {
                div_round_unsigned(self, other, rm)
            }
        }

        impl DivRoundAssign<$t> for $t {
            /// Divides a value by another value in place and rounds according to a specified
            /// rounding mode.
            ///
            /// See the [`DivRound`](super::traits::DivRound) documentation for details.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by
            /// `other`.
            ///
            /// # Examples
            /// See [here](super::div_round#div_round_assign).
            #[inline]
            fn div_round_assign(&mut self, other: $t, rm: RoundingMode) {
                *self = self.div_round(other, rm);
            }
        }
    };
}
apply_to_unsigneds!(impl_div_round_unsigned);

fn div_round_signed<
    U: PrimitiveUnsigned,
    S: ExactFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
>(
    x: S,
    other: S,
    rm: RoundingMode,
) -> S {
    if (x >= S::ZERO) == (other >= S::ZERO) {
        S::exact_from(x.unsigned_abs().div_round(other.unsigned_abs(), rm))
    } else {
        // Has to be wrapping so that (self, other) == (T::MIN, 1) works
        S::wrapping_from(x.unsigned_abs().div_round(other.unsigned_abs(), -rm)).wrapping_neg()
    }
}

macro_rules! impl_div_round_signed {
    ($t:ident) => {
        impl DivRound<$t> for $t {
            type Output = $t;

            /// Divides a value by another value and rounds according to a specified rounding mode.
            ///
            /// Let $q = \frac{x}{y}$:
            ///
            /// $$
            /// f(x, y, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.
            /// $$
            ///
            /// $$
            /// f(x, y, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.
            /// $$
            ///
            /// $$
            /// f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.
            /// $$
            ///
            /// $$
            /// f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.
            /// $$
            ///
            /// $$
            /// f(x, y, \mathrm{Nearest}) = \begin{cases}
            ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
            ///     \lceil q \rceil & q - \lfloor q \rfloor > \frac{1}{2}, \\\\
            ///     \lfloor q \rfloor &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
            ///     \\ \lfloor q \rfloor \\ \text{is even}, \\\\
            ///     \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
            ///     \\ \lfloor q \rfloor \\ \text{is odd.}
            /// \end{cases}
            /// $$
            ///
            /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is zero, if `self` is `Self::MIN` and `other` is `-1`, or if `rm`
            /// is `Exact` but `self` is not divisible by `other`.
            ///
            /// # Examples
            /// See [here](super::div_round#div_round).
            fn div_round(self, other: $t, rm: RoundingMode) -> $t {
                div_round_signed(self, other, rm)
            }
        }

        impl DivRoundAssign<$t> for $t {
            /// Divides a value by another value in place and rounds according to a specified
            /// rounding mode.
            ///
            /// See the [`DivRound`](super::traits::DivRound) documentation for details.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is zero, if `self` is `Self::MIN` and `other` is `-1`, or if `rm`
            /// is `Exact` but `self` is not divisible by `other`.
            ///
            /// # Examples
            /// See [here](super::div_round#div_round_assign).
            #[inline]
            fn div_round_assign(&mut self, other: $t, rm: RoundingMode) {
                *self = self.div_round(other, rm);
            }
        }
    };
}
apply_to_signeds!(impl_div_round_signed);

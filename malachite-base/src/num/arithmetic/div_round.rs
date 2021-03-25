use num::arithmetic::traits::{DivRound, DivRoundAssign, Parity, UnsignedAbs, WrappingNeg};
use num::basic::traits::{One, Zero};
use num::conversion::traits::{ExactFrom, WrappingFrom};
use rounding_modes::RoundingMode;
use std::fmt::Display;
use std::ops::{Add, Div, Mul, Shr, Sub};

fn _div_round_unsigned<
    T: Add<T, Output = T>
        + Copy
        + Display
        + Div<T, Output = T>
        + Eq
        + Mul<T, Output = T>
        + One
        + Ord
        + Parity
        + Shr<u64, Output = T>
        + Sub<T, Output = T>
        + Zero,
>(
    x: T,
    other: T,
    rm: RoundingMode,
) -> T {
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
            /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.$
            ///
            /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.$
            ///
            /// $$
            /// f(x, y, \mathrm{Nearest}) = \begin{cases}
            ///     \lfloor q \rfloor & q - \lfloor q \rfloor < \frac{1}{2} \\\\
            ///     \lceil q \rceil & q - \lfloor q \rfloor > \frac{1}{2} \\\\
            ///     \lfloor q \rfloor &
            ///     q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is even} \\\\
            ///     \lceil q \rceil &
            ///     q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is odd.}
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
            /// See the documentation of the `num::arithmetic::div_round` module.
            #[inline]
            fn div_round(self, other: $t, rm: RoundingMode) -> $t {
                _div_round_unsigned(self, other, rm)
            }
        }

        impl DivRoundAssign<$t> for $t {
            /// Divides a value by another value in place and rounds according to a specified
            /// rounding mode.
            ///
            /// See the `DivRound` documentation for details.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is zero, or if `rm` is `Exact` but `self` is not divisible by
            /// `other`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::div_round` module.
            #[inline]
            fn div_round_assign(&mut self, other: $t, rm: RoundingMode) {
                *self = self.div_round(other, rm);
            }
        }
    };
}
apply_to_unsigneds!(impl_div_round_unsigned);

fn _div_round_signed<
    U: DivRound<U, Output = U>,
    S: Copy
        + ExactFrom<U>
        + Ord
        + UnsignedAbs<Output = U>
        + WrappingFrom<U>
        + WrappingNeg<Output = S>
        + Zero,
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
            /// $f(x, y, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.$
            ///
            /// $f(x, y, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.$
            ///
            /// $f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.$
            ///
            /// $f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.$
            ///
            /// $$
            /// f(x, y, \mathrm{Nearest}) = \begin{cases}
            ///     \lfloor q \rfloor & q - \lfloor q \rfloor < \frac{1}{2} \\\\
            ///     \lceil q \rceil & q - \lfloor q \rfloor > \frac{1}{2} \\\\
            ///     \lfloor q \rfloor &
            ///     q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is even} \\\\
            ///     \lceil q \rceil &
            ///     q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is odd.}
            /// \end{cases}
            /// $$
            ///
            /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is zero, if `self` is `$t::MIN` and `other` is `-1`, or if `rm` is
            /// `Exact` but `self` is not divisible by `other`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::div_round` module.
            fn div_round(self, other: $t, rm: RoundingMode) -> $t {
                _div_round_signed(self, other, rm)
            }
        }

        impl DivRoundAssign<$t> for $t {
            /// Divides a value by another value in place and rounds according to a specified
            /// rounding mode.
            ///
            /// See the `DivRound` documentation for details.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `other` is zero, if `self` is `$t::MIN` and `other` is `-1`, or if `rm` is
            /// `Exact` but `self` is not divisible by `other`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::div_round` module.
            #[inline]
            fn div_round_assign(&mut self, other: $t, rm: RoundingMode) {
                *self = self.div_round(other, rm);
            }
        }
    };
}
apply_to_signeds!(impl_div_round_signed);

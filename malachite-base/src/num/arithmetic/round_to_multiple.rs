use crate::num::arithmetic::traits::{RoundToMultiple, RoundToMultipleAssign, UnsignedAbs};
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::ExactFrom;
use crate::rounding_modes::RoundingMode;
use std::cmp::Ordering;

fn round_to_multiple_unsigned<T: PrimitiveUnsigned>(x: T, other: T, rm: RoundingMode) -> T {
    match (x, other) {
        (x, y) if x == y => x,
        (x, y) if y == T::ZERO => match rm {
            RoundingMode::Down | RoundingMode::Floor | RoundingMode::Nearest => T::ZERO,
            _ => panic!("Cannot round {} to zero using RoundingMode {}", x, rm),
        },
        (x, y) => {
            let r = x % y;
            if r == T::ZERO {
                x
            } else {
                let floor = x - r;
                match rm {
                    RoundingMode::Down | RoundingMode::Floor => floor,
                    RoundingMode::Up | RoundingMode::Ceiling => floor.checked_add(y).unwrap(),
                    RoundingMode::Nearest => {
                        match r.cmp(&(y >> 1)) {
                            Ordering::Less => floor,
                            Ordering::Greater => floor.checked_add(y).unwrap(),
                            Ordering::Equal => {
                                if y.odd() {
                                    floor
                                } else {
                                    // The even multiple of y will have more trailing zeros.
                                    let (ceiling, overflow) = floor.overflowing_add(y);
                                    if floor.trailing_zeros() > ceiling.trailing_zeros() {
                                        floor
                                    } else if overflow {
                                        panic!(
                                            "Cannot round {} to {} using RoundingMode {}",
                                            x, y, rm
                                        );
                                    } else {
                                        ceiling
                                    }
                                }
                            }
                        }
                    }
                    RoundingMode::Exact => {
                        panic!("Cannot round {} to {} using RoundingMode {}", x, y, rm)
                    }
                }
            }
        }
    }
}

macro_rules! impl_round_to_multiple_unsigned {
    ($t:ident) => {
        impl RoundToMultiple<$t> for $t {
            type Output = $t;

            /// Rounds a number to a multiple of another number, according to a specified rounding
            /// mode.
            ///
            /// The only rounding modes that are guaranteed to return without a panic are `Down`
            /// and `Floor`.
            ///
            /// Let $q = \frac{x}{y}$:
            ///
            /// $f(x, y, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = y \lfloor q \rfloor.$
            ///
            /// $f(x, y, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = y \lceil q \rceil.$
            ///
            /// $$
            /// f(x, y, \mathrm{Nearest}) = \begin{cases}
            ///     y \lfloor q \rfloor & \text{if} \\quad
            ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
            ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
            ///     y \lfloor q \rfloor &
            ///     \text{if} \\quad q - \lfloor q \rfloor =
            ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is even} \\\\
            ///     y \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
            ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
            /// \end{cases}
            /// $$
            ///
            /// $f(x, y, \mathrm{Exact}) = x$, but panics if $q \notin \N$.
            ///
            /// The following two expressions are equivalent:
            /// - `x.round_to_multiple(other, RoundingMode::Exact)`
            /// - `{ assert!(x.divisible_by(other)); x }`
            ///
            /// but the latter should be used as it is clearer and more efficient.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
            /// - If the multiple is outside the representable range.
            /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
            ///
            /// # Examples
            /// See [here](super::round_to_multiple#round_to_multiple).
            #[inline]
            fn round_to_multiple(self, other: $t, rm: RoundingMode) -> $t {
                round_to_multiple_unsigned(self, other, rm)
            }
        }

        impl RoundToMultipleAssign<$t> for $t {
            /// Rounds a number to a multiple of another number in place, according to a specified
            /// rounding mode.
            ///
            /// The only rounding modes that are guaranteed to return without a panic are `Down`
            /// and `Floor`.
            ///
            /// See the [`RoundToMultiple`](super::traits::RoundToMultiple) documentation for
            /// details.
            ///
            /// The following two expressions are equivalent:
            /// - `x.round_to_multiple_assign(other, RoundingMode::Exact);`
            /// - `assert!(x.divisible_by(other));`
            ///
            /// but the latter should be used as it is clearer and more efficient.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
            /// - If the multiple is outside the representable range.
            /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
            ///
            /// # Examples
            /// See [here](super::round_to_multiple#round_to_multiple_assign).
            #[inline]
            fn round_to_multiple_assign(&mut self, other: $t, rm: RoundingMode) {
                *self = self.round_to_multiple(other, rm);
            }
        }
    };
}
apply_to_unsigneds!(impl_round_to_multiple_unsigned);

fn round_to_multiple_signed<
    U: PrimitiveUnsigned,
    S: ExactFrom<U> + PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: S,
    other: S,
    rm: RoundingMode,
) -> S {
    if x >= S::ZERO {
        S::exact_from(x.unsigned_abs().round_to_multiple(other.unsigned_abs(), rm))
    } else {
        let abs_result = x
            .unsigned_abs()
            .round_to_multiple(other.unsigned_abs(), -rm);
        if abs_result == S::MIN.unsigned_abs() {
            S::MIN
        } else {
            S::exact_from(abs_result).checked_neg().unwrap()
        }
    }
}

macro_rules! impl_round_to_multiple_signed {
    ($t:ident) => {
        impl RoundToMultiple<$t> for $t {
            type Output = $t;

            /// Rounds a number to a multiple of another number, according to a specified rounding
            /// mode.
            ///
            /// The only rounding mode that is guaranteed to return without a panic is `Down`.
            ///
            /// Let $q = \frac{x}{|y|}$:
            ///
            /// $f(x, y, \mathrm{Down}) =  \operatorname{sgn}(q) |y| \lfloor |q| \rfloor.$
            ///
            /// $f(x, y, \mathrm{Up}) = \operatorname{sgn}(q) |y| \lceil |q| \rceil.$
            ///
            /// $f(x, y, \mathrm{Floor}) = |y| \lfloor q \rfloor.$
            ///
            /// $f(x, y, \mathrm{Ceiling}) = |y| \lceil q \rceil.$
            ///
            /// $$
            /// f(x, y, \mathrm{Nearest}) = \begin{cases}
            ///     y \lfloor q \rfloor & \text{if} \\quad
            ///         q - \lfloor q \rfloor < \frac{1}{2} \\\\
            ///     y \lceil q \rceil & \text{if} \\quad q - \lfloor q \rfloor > \frac{1}{2} \\\\
            ///     y \lfloor q \rfloor &
            ///     \text{if} \\quad q - \lfloor q \rfloor =
            ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is even} \\\\
            ///     y \lceil q \rceil &
            ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2}
            ///         \\ \text{and} \\ \lfloor q \rfloor \\ \text{is odd.}
            /// \end{cases}
            /// $$
            ///
            /// $f(x, y, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
            ///
            /// The following two expressions are equivalent:
            /// - `x.round_to_multiple(other, RoundingMode::Exact)`
            /// - `{ assert!(x.divisible_by(other)); x }`
            ///
            /// but the latter should be used as it is clearer and more efficient.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
            /// - If the multiple is outside the representable range.
            /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
            ///
            /// # Examples
            /// See [here](super::round_to_multiple#round_to_multiple).
            #[inline]
            fn round_to_multiple(self, other: $t, rm: RoundingMode) -> $t {
                round_to_multiple_signed(self, other, rm)
            }
        }

        impl RoundToMultipleAssign<$t> for $t {
            /// Rounds a number to a multiple of another number in place, according to a specified
            /// rounding mode.
            ///
            /// The only rounding mode that is guaranteed to return without a panic is `Down`.
            ///
            /// See the [`RoundToMultiple`](super::traits::RoundToMultiple) documentation for
            /// details.
            ///
            /// The following two expressions are equivalent:
            /// - `x.round_to_multiple_assign(other, RoundingMode::Exact);`
            /// - `assert!(x.divisible_by(other));`
            ///
            /// but the latter should be used as it is clearer and more efficient.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// - If `rm` is `Exact`, but `self` is not a multiple of `other`.
            /// - If the multiple is outside the representable range.
            /// - If `self` is nonzero, `other` is zero, and `rm` is trying to round away from zero.
            ///
            /// # Examples
            /// See [here](super::round_to_multiple#round_to_multiple_assign).
            #[inline]
            fn round_to_multiple_assign(&mut self, other: $t, rm: RoundingMode) {
                *self = self.round_to_multiple(other, rm);
            }
        }
    };
}
apply_to_signeds!(impl_round_to_multiple_signed);

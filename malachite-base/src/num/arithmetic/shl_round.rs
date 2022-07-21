use crate::num::arithmetic::traits::{ShlRound, ShlRoundAssign, ShrRound, ShrRoundAssign, UnsignedAbs};
use crate::num::basic::integers::PrimitiveInt;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::rounding_modes::RoundingMode;
use std::ops::{Shl, ShlAssign};

fn shl_round<
    T: PrimitiveInt + Shl<U, Output = T> + ShrRound<U, Output = T>,
    U,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: T,
    bits: S,
    rm: RoundingMode,
) -> T {
    if bits >= S::ZERO {
        let width = S::wrapping_from(T::WIDTH);
        if width >= S::ZERO && bits >= width {
            T::ZERO
        } else {
            x << bits.unsigned_abs()
        }
    } else {
        x.shr_round(bits.unsigned_abs(), rm)
    }
}

fn shl_round_assign<
    T: PrimitiveInt + ShlAssign<U> + ShrRoundAssign<U>,
    U,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: &mut T,
    bits: S,
    rm: RoundingMode,
) {
    if bits >= S::ZERO {
        let width = S::wrapping_from(T::WIDTH);
        if width >= S::ZERO && bits >= width {
            *x = T::ZERO;
        } else {
            *x <<= bits.unsigned_abs();
        }
    } else {
        x.shr_round_assign(bits.unsigned_abs(), rm);
    }
}

macro_rules! impl_shl_round {
    ($t:ident) => {
        macro_rules! impl_shl_round_inner {
            ($u:ident) => {
                impl ShlRound<$u> for $t {
                    type Output = $t;

                    /// Left-shifts a number (multiplies it by a power of 2 or divides it by a
                    /// power of 2 and takes the floor) and rounds according to the specified
                    /// rounding mode.
                    ///
                    /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to
                    /// using `>>`. To test whether `RoundingMode::Exact` can be passed, use
                    /// `bits > 0 || self.divisible_by_power_of_2(bits)`. Rounding might only be
                    /// necessary if `bits` is negative.
                    ///
                    /// Let $q = x2^k$:
                    ///
                    /// $f(x, k, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.$
                    ///
                    /// $f(x, k, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.$
                    ///
                    /// $$
                    /// f(x, k, \mathrm{Nearest}) = \begin{cases}
                    ///     \lfloor q \rfloor & \text{if}
                    ///         \\quad q - \lfloor q \rfloor < \frac{1}{2}, \\\\
                    ///     \lceil q \rceil & \text{if}
                    ///         \\quad q - \lfloor q \rfloor > \frac{1}{2}, \\\\
                    ///     \lfloor q \rfloor & \text{if} \\quad q - \lfloor q \rfloor =
                    ///         \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
                    ///     \\ \text{is even}, \\\\
                    ///     \lceil q \rceil &
                    ///     \text{if} \\quad q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and}
                    ///         \\ \lfloor q \rfloor \\ \text{is odd}.
                    /// \end{cases}
                    /// $$
                    ///
                    /// $f(x, k, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `bits` is positive and `rm` is `RoundingMode::Exact` but `self` is
                    /// not divisible by $2^b$.
                    ///
                    /// # Examples
                    /// See [here](super::shl_round#shl_round).
                    #[inline]
                    fn shl_round(self, bits: $u, rm: RoundingMode) -> $t {
                        shl_round(self, bits, rm)
                    }
                }

                impl ShlRoundAssign<$u> for $t {
                    /// Left-shifts a number (multiplies it by a power of 2 or divides it by a
                    /// power of 2 and takes the floor) and rounds according to the specified
                    /// rounding mode, in place.
                    ///
                    /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to
                    /// using `>>`. To test whether `RoundingMode::Exact` can be passed, use
                    /// `bits > 0 || self.divisible_by_power_of_2(bits)`. Rounding might only be
                    /// necessary if `bits` is negative.
                    ///
                    /// See the [`ShlRound`](super::traits::ShlRound) documentation for details.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `bits` is positive and `rm` is `RoundingMode::Exact` but `self` is
                    /// not divisible by $2^b$.
                    ///
                    /// # Examples
                    /// See [here](super::shl_round#shl_round_assign).
                    #[inline]
                    fn shl_round_assign(&mut self, bits: $u, rm: RoundingMode) {
                        shl_round_assign(self, bits, rm)
                    }
                }
            };
        }
        apply_to_signeds!(impl_shl_round_inner);
    };
}
apply_to_primitive_ints!(impl_shl_round);

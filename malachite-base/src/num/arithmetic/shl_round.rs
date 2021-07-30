use num::arithmetic::traits::{ShlRound, ShlRoundAssign, ShrRound, ShrRoundAssign, UnsignedAbs};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::Zero;
use num::conversion::traits::WrappingFrom;
use rounding_modes::RoundingMode;
use std::ops::{Shl, ShlAssign};

fn _shl_round<
    T: PrimitiveInt + Shl<U, Output = T> + ShrRound<U, Output = T>,
    U,
    S: Copy + Ord + UnsignedAbs<Output = U> + WrappingFrom<u64> + Zero,
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

fn _shl_round_assign<
    T: PrimitiveInt + ShlAssign<U> + ShrRoundAssign<U>,
    U,
    S: Copy + Ord + UnsignedAbs<Output = U> + WrappingFrom<u64> + Zero,
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

                    /// Shifts `self` left (multiplies it by a power of 2 or divides it by a power
                    /// of 2 and takes the floor) and rounds according to the specified rounding
                    /// mode.
                    ///
                    /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to
                    /// using `>>`. To test whether `RoundingMode::Exact` can be passed, use
                    /// `bits > 0 || self.divisible_by_power_of_2(bits)`. Rounding might only be
                    /// necessary if `bits` is non-negative.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `bits` is positive and `rm` is `RoundingMode::Exact` but `self` is
                    /// not divisible by $2^b$
                    ///
                    /// # Examples
                    /// See the documentation of the `num::arithmetic::shl_round` module.
                    #[inline]
                    fn shl_round(self, bits: $u, rm: RoundingMode) -> $t {
                        _shl_round(self, bits, rm)
                    }
                }

                impl ShlRoundAssign<$u> for $t {
                    /// Shifts `self` left (multiplies it by a power of 2 or divides it by a power
                    /// of 2 and takes the floor) and rounds according to the specified rounding
                    /// mode, in place.
                    ///
                    /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to
                    /// using `>>`. To test whether `RoundingMode::Exact` can be passed, use
                    /// `bits > 0 || self.divisible_by_power_of_2(bits)`. Rounding might only be
                    /// necessary if `bits` is non-negative.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `bits` is positive and `rm` is `RoundingMode::Exact` but `self` is
                    /// not divisible by $2^b$.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::arithmetic::shl_round` module.
                    #[inline]
                    fn shl_round_assign(&mut self, bits: $u, rm: RoundingMode) {
                        _shl_round_assign(self, bits, rm)
                    }
                }
            };
        }
        apply_to_signeds!(impl_shl_round_inner);
    };
}
apply_to_primitive_ints!(impl_shl_round);

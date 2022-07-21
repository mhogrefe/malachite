use crate::integer::Integer;
use malachite_base::num::arithmetic::traits::{
    ShlRound, ShlRoundAssign, ShrRound, ShrRoundAssign, UnsignedAbs,
};
use malachite_base::num::basic::traits::Zero;
use malachite_base::rounding_modes::RoundingMode;
use std::ops::{Shl, ShlAssign};

fn shl_round_signed_ref<'a, U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &'a Integer,
    bits: S,
    rm: RoundingMode,
) -> Integer
where
    &'a Integer: Shl<U, Output = Integer> + ShrRound<U, Output = Integer>,
{
    if bits >= S::ZERO {
        x << bits.unsigned_abs()
    } else {
        x.shr_round(bits.unsigned_abs(), rm)
    }
}

fn shl_round_assign_i<U, S: Copy + Ord + UnsignedAbs<Output = U> + Zero>(
    x: &mut Integer,
    bits: S,
    rm: RoundingMode,
) where
    Integer: ShlAssign<U> + ShrRoundAssign<U>,
{
    if bits >= S::ZERO {
        *x <<= bits.unsigned_abs();
    } else {
        x.shr_round_assign(bits.unsigned_abs(), rm);
    }
}

macro_rules! impl_shl_round_signed {
    ($t:ident) => {
        impl ShlRound<$t> for Integer {
            type Output = Integer;

            /// Left-shifts an [`Integer`] (multiplies or divides it by a power of 2), taking it by
            /// value, and rounds according to the specified rounding mode.
            ///
            /// Passing `RoundingMode::Floor` is equivalent to using `>>`. To test whether
            /// `RoundingMode::Exact` can be passed, use
            /// `bits > 0 || self.divisible_by_power_of_2(bits)`. Rounding might only be necessary
            /// if `bits` is negative.
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
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(bits, 0)`.
            ///
            /// # Panics
            /// Let $k$ be `bits`. Panics if $k$ is negative and `rm` is `RoundingMode::Exact` but
            /// `self` is not divisible by $2^{-k}$.
            ///
            /// # Examples
            /// See [here](super::shl_round#shl_round).
            #[inline]
            fn shl_round(mut self, bits: $t, rm: RoundingMode) -> Integer {
                self.shl_round_assign(bits, rm);
                self
            }
        }

        impl<'a> ShlRound<$t> for &'a Integer {
            type Output = Integer;

            /// Left-shifts an [`Integer`] (multiplies or divides it by a power of 2), taking it by
            /// reference, and rounds according to the specified rounding mode.
            ///
            /// Passing `RoundingMode::Floor` is equivalent to using `>>`. To test whether
            /// `RoundingMode::Exact` can be passed, use
            /// `bits > 0 || self.divisible_by_power_of_2(bits)`. Rounding might only be necessary
            /// if `bits` is negative.
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
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(bits, 0)`.
            ///
            /// # Panics
            /// Let $k$ be `bits`. Panics if $k$ is negative and `rm` is `RoundingMode::Exact` but
            /// `self` is not divisible by $2^{-k}$.
            ///
            /// # Examples
            /// See [here](super::shl_round#shl_round).
            #[inline]
            fn shl_round(self, bits: $t, rm: RoundingMode) -> Integer {
                shl_round_signed_ref(self, bits, rm)
            }
        }

        impl ShlRoundAssign<$t> for Integer {
            /// Left-shifts an [`Integer`] (multiplies or divides it by a power of 2) and rounds
            /// according to the specified rounding mode, in place.
            ///
            /// Passing `RoundingMode::Floor` is equivalent to using `>>`. To test whether
            /// `RoundingMode::Exact` can be passed, use
            /// `bits > 0 || self.divisible_by_power_of_2(bits)`. Rounding might only be
            /// necessary if `bits` is negative.
            ///
            /// See the [`ShlRound`](malachite_base::num::arithmetic::traits::ShlRound)
            /// documentation for details.
            ///
            /// # Worst-case complexity
            /// $T(n, m) = O(n + m)$
            ///
            /// $M(n, m) = O(n + m)$
            ///
            /// where $T$ is time, $M$ is additional memory, $n$ is `self.significant_bits()`, and
            /// $m$ is `max(bits, 0)`.
            ///
            /// # Examples
            /// See [here](super::shl_round#shl_round_assign).
            #[inline]
            fn shl_round_assign(&mut self, bits: $t, rm: RoundingMode) {
                shl_round_assign_i(self, bits, rm);
            }
        }
    };
}
apply_to_signeds!(impl_shl_round_signed);

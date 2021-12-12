use num::arithmetic::traits::{
    ArithmeticCheckedShl, RoundToMultipleOfPowerOf2, RoundToMultipleOfPowerOf2Assign, ShrRound,
};
use rounding_modes::RoundingMode;

fn round_to_multiple_of_power_of_2<
    T: ArithmeticCheckedShl<u64, Output = T> + ShrRound<u64, Output = T>,
>(
    x: T,
    pow: u64,
    rm: RoundingMode,
) -> T {
    x.shr_round(pow, rm).arithmetic_checked_shl(pow).unwrap()
}

macro_rules! impl_round_to_multiple_of_power_of_2 {
    ($t:ident) => {
        impl RoundToMultipleOfPowerOf2<u64> for $t {
            type Output = $t;

            /// Rounds `self` to a multiple of a power of 2, according to a specified rounding mode.
            ///
            /// The only rounding mode that is guaranteed to return without a panic is `Down`.
            ///
            /// Let $q = \frac{x}{2^p}$:
            ///
            /// $f(x, p, \mathrm{Down}) = 2^p \operatorname{sgn}(q) \lfloor |q| \rfloor.$
            ///
            /// $f(x, p, \mathrm{Up}) = 2^p \operatorname{sgn}(q) \lceil |q| \rceil.$
            ///
            /// $f(x, p, \mathrm{Floor}) = 2^p \lfloor q \rfloor.$
            ///
            /// $f(x, p, \mathrm{Ceiling}) = 2^p \lceil q \rceil.$
            ///
            /// $$
            /// f(x, p, \mathrm{Nearest}) = \begin{cases}
            ///     2^p \lfloor q \rfloor & q - \lfloor q \rfloor < \frac{1}{2} \\\\
            ///     2^p \lceil q \rceil & q - \lfloor q \rfloor > \frac{1}{2} \\\\
            ///     2^p \lfloor q \rfloor &
            ///     q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is even} \\\\
            ///     2^p \lceil q \rceil &
            ///     q - \lfloor q \rfloor = \frac{1}{2} \\ \text{and} \\ \lfloor q \rfloor
            ///     \\ \text{is odd.}
            /// \end{cases}
            /// $$
            ///
            /// $f(x, p, \mathrm{Exact}) = 2^p q$, but panics if $q \notin \Z$.
            ///
            /// The following two expressions are equivalent:
            ///
            /// `x.round_to_multiple_of_power_of_2(pow, RoundingMode::Exact)`
            ///
            /// `{ assert!(x.divisible_by_power_of_2(pow)); x }`
            ///
            /// but the latter should be used as it is clearer and more efficient.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// - If `rm` is `Exact`, but `self` is not a multiple of the power of 2.
            /// - If `rm` is `Floor`, but `self` is negative with a too-large absolute value to
            ///   round to the next lowest multiple.
            /// - If `rm` is `Ceiling`, but `self` is too large to round to the next highest
            ///   multiple.
            /// - If `rm` is `Up`, but `self` has too large an absolute value to round to the next
            ///   multiple with a greater absolute value.
            /// - If `rm` is `Nearest`, but the nearest multiple is outside the representable range.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::round_to_multiple_of_power_of_2`
            /// module.
            #[inline]
            fn round_to_multiple_of_power_of_2(self, pow: u64, rm: RoundingMode) -> $t {
                round_to_multiple_of_power_of_2(self, pow, rm)
            }
        }

        impl RoundToMultipleOfPowerOf2Assign<u64> for $t {
            /// Rounds `self` to a multiple of a power of 2 in place, according to a specified
            /// rounding mode.
            ///
            /// The only rounding mode that is guaranteed to return without a panic is `Down`.
            ///
            /// See the `RoundToMultipleOfPowerOf2` documentation for details.
            ///
            /// The following two expressions are equivalent:
            ///
            /// `x.round_to_multiple_of_power_of_2_assign(pow, RoundingMode::Exact);`
            ///
            /// `assert!(x.divisible_by_power_of_2(pow));`
            ///
            /// but the latter should be used as it is clearer and more efficient.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// - If `rm` is `Exact`, but `self` is not a multiple of the power of 2.
            /// - If `rm` is `Floor`, but `self` is negative with a too-large absolute value to
            ///   round to the next lowest multiple.
            /// - If `rm` is `Ceiling`, but `self` is too large to round to the next highest
            ///   multiple.
            /// - If `rm` is `Up`, but `self` has too large an absolute value to round to the next
            ///   multiple with a greater absolute value.
            /// - If `rm` is `Nearest`, but the nearest multiple is outside the representable range.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::round_to_multiple_of_power_of_2`
            /// module.
            #[inline]
            fn round_to_multiple_of_power_of_2_assign(&mut self, pow: u64, rm: RoundingMode) {
                *self = self.round_to_multiple_of_power_of_2(pow, rm);
            }
        }
    };
}
apply_to_primitive_ints!(impl_round_to_multiple_of_power_of_2);

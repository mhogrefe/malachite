use num::arithmetic::traits::{ShrRound, ShrRoundAssign, UnsignedAbs};
use num::basic::integers::PrimitiveInt;
use num::basic::signeds::PrimitiveSigned;
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::WrappingFrom;
use rounding_modes::RoundingMode;
use std::ops::{Shl, ShlAssign, Shr, ShrAssign};

fn shr_round_unsigned_unsigned<
    T: PrimitiveUnsigned + Shl<U, Output = T> + Shr<U, Output = T>,
    U: PrimitiveUnsigned,
>(
    x: T,
    bits: U,
    rm: RoundingMode,
) -> T {
    if bits == U::ZERO || x == T::ZERO {
        return x;
    }
    let width = U::wrapping_from(T::WIDTH);
    match rm {
        RoundingMode::Down | RoundingMode::Floor if bits >= width => T::ZERO,
        RoundingMode::Down | RoundingMode::Floor => x >> bits,
        RoundingMode::Up | RoundingMode::Ceiling if bits >= width => T::ONE,
        RoundingMode::Up | RoundingMode::Ceiling => {
            let shifted = x >> bits;
            if shifted << bits == x {
                shifted
            } else {
                shifted + T::ONE
            }
        }
        RoundingMode::Nearest if bits == width && x > T::power_of_2(T::WIDTH - 1) => T::ONE,
        RoundingMode::Nearest if bits >= width => T::ZERO,
        RoundingMode::Nearest => {
            let mostly_shifted = x >> (bits - U::ONE);
            if mostly_shifted.even() {
                // round down
                mostly_shifted >> 1
            } else if mostly_shifted << (bits - U::ONE) != x {
                // round up
                (mostly_shifted >> 1) + T::ONE
            } else {
                // result is half-integer; round to even
                let shifted: T = mostly_shifted >> 1;
                if shifted.even() {
                    shifted
                } else {
                    shifted + T::ONE
                }
            }
        }
        RoundingMode::Exact if bits >= width => {
            panic!("Right shift is not exact: {} >> {}", x, bits);
        }
        RoundingMode::Exact => {
            let shifted = x >> bits;
            if shifted << bits != x {
                panic!("Right shift is not exact: {} >> {}", x, bits);
            }
            shifted
        }
    }
}

fn shr_round_assign_unsigned_unsigned<
    T: PrimitiveUnsigned + Shl<U, Output = T> + ShrAssign<U>,
    U: PrimitiveUnsigned,
>(
    x: &mut T,
    bits: U,
    rm: RoundingMode,
) {
    if bits == U::ZERO || *x == T::ZERO {
        return;
    }
    let width = U::wrapping_from(T::WIDTH);
    match rm {
        RoundingMode::Down | RoundingMode::Floor if bits >= width => *x = T::ZERO,
        RoundingMode::Down | RoundingMode::Floor => *x >>= bits,
        RoundingMode::Up | RoundingMode::Ceiling if bits >= width => *x = T::ONE,
        RoundingMode::Up | RoundingMode::Ceiling => {
            let original = *x;
            *x >>= bits;
            if *x << bits != original {
                *x += T::ONE;
            }
        }
        RoundingMode::Nearest if bits == width && *x > T::power_of_2(T::WIDTH - 1) => {
            *x = T::ONE;
        }
        RoundingMode::Nearest if bits >= width => *x = T::ZERO,
        RoundingMode::Nearest => {
            let original = *x;
            *x >>= bits - U::ONE;
            let old_x = *x;
            *x >>= 1;
            if old_x.even() {
                // round down
            } else if old_x << (bits - U::ONE) != original {
                // round up
                *x += T::ONE;
            } else {
                // result is half-integer; round to even
                if x.odd() {
                    *x += T::ONE;
                }
            }
        }
        RoundingMode::Exact if bits >= width => {
            panic!("Right shift is not exact: {} >>= {}", *x, bits);
        }
        RoundingMode::Exact => {
            let original = *x;
            *x >>= bits;
            if *x << bits != original {
                panic!("Right shift is not exact: {} >>= {}", original, bits);
            }
        }
    }
}

macro_rules! impl_shr_round_unsigned_unsigned {
    ($t:ident) => {
        macro_rules! impl_shr_round_unsigned_unsigned_inner {
            ($u:ident) => {
                impl ShrRound<$u> for $t {
                    type Output = $t;

                    /// Shifts a number right (divides it by a power of 2) and rounds according to
                    /// the specified rounding mode.
                    ///
                    /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to using
                    /// `>>`. To test whether `RoundingMode::Exact` can be passed, use
                    /// `self.divisible_by_power_of_2(bits)`.
                    ///
                    /// Let $q = \frac{x}{2^k}$:
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
                    /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
                    /// $2^b$.
                    ///
                    /// # Examples
                    /// See [here](super::shr_round#shr_round).
                    #[inline]
                    fn shr_round(self, bits: $u, rm: RoundingMode) -> $t {
                        shr_round_unsigned_unsigned(self, bits, rm)
                    }
                }

                impl ShrRoundAssign<$u> for $t {
                    /// Shifts a number right (divides it by a power of 2) and rounds according to
                    /// the specified rounding mode, in place.
                    ///
                    /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to
                    /// using `>>`. To test whether `RoundingMode::Exact` can be passed, use
                    /// `self.divisible_by_power_of_2(bits)`.
                    ///
                    /// See the [`ShrRound`](super::traits::ShrRound) documentation for details.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
                    /// $2^b$.
                    ///
                    /// # Examples
                    /// See [here](super::shr_round#shr_round_assign).
                    #[inline]
                    fn shr_round_assign(&mut self, bits: $u, rm: RoundingMode) {
                        shr_round_assign_unsigned_unsigned(self, bits, rm);
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_shr_round_unsigned_unsigned_inner);
    };
}
apply_to_unsigneds!(impl_shr_round_unsigned_unsigned);

fn shr_round_signed_unsigned<
    U: PrimitiveUnsigned + ShrRound<B, Output = U>,
    S: PrimitiveSigned + UnsignedAbs<Output = U> + WrappingFrom<U>,
    B,
>(
    x: S,
    bits: B,
    rm: RoundingMode,
) -> S {
    let abs = x.unsigned_abs();
    if x >= S::ZERO {
        S::wrapping_from(abs.shr_round(bits, rm))
    } else {
        let abs_shifted = abs.shr_round(bits, -rm);
        if abs_shifted == U::ZERO {
            S::ZERO
        } else if abs_shifted == S::MIN.unsigned_abs() {
            S::MIN
        } else {
            -S::wrapping_from(abs_shifted)
        }
    }
}

macro_rules! impl_shr_round_signed_unsigned {
    ($t:ident) => {
        macro_rules! impl_shr_round_signed_unsigned_inner {
            ($u:ident) => {
                impl ShrRound<$u> for $t {
                    type Output = $t;

                    /// Shifts a number right (divides it by a power of 2) and rounds according to
                    /// the specified rounding mode.
                    ///
                    /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to
                    /// using `>>`. To test whether `RoundingMode::Exact` can be passed, use
                    /// `self.divisible_by_power_of_2(bits)`.
                    ///
                    /// Let $q = \frac{x}{2^p}$:
                    ///
                    /// $f(x, p, \mathrm{Down}) = f(x, y, \mathrm{Floor}) = \lfloor q \rfloor.$
                    ///
                    /// $f(x, p, \mathrm{Up}) = f(x, y, \mathrm{Ceiling}) = \lceil q \rceil.$
                    ///
                    /// $$
                    /// f(x, p, \mathrm{Nearest}) = \begin{cases}
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
                    /// $f(x, p, \mathrm{Exact}) = q$, but panics if $q \notin \N$.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
                    /// $2^b$.
                    ///
                    /// # Examples
                    /// See [here](super::shr_round#shr_round).
                    #[inline]
                    fn shr_round(self, bits: $u, rm: RoundingMode) -> $t {
                        shr_round_signed_unsigned(self, bits, rm)
                    }
                }

                impl ShrRoundAssign<$u> for $t {
                    /// Shifts a number right (divides it by a power of 2) and rounds according to
                    /// the specified rounding mode, in place.
                    ///
                    /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to
                    /// using `>>`. To test whether `RoundingMode::Exact` can be passed, use
                    /// `self.divisible_by_power_of_2(bits)`.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `rm` is `RoundingMode::Exact` but `self` is not divisible by
                    /// $2^b$.
                    ///
                    /// # Examples
                    /// See [here](super::shr_round#shr_round_assign).
                    #[inline]
                    fn shr_round_assign(&mut self, bits: $u, rm: RoundingMode) {
                        *self = self.shr_round(bits, rm);
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_shr_round_signed_unsigned_inner);
    };
}
apply_to_signeds!(impl_shr_round_signed_unsigned);

fn shr_round_primitive_signed<
    T: PrimitiveInt + Shl<U, Output = T> + ShrRound<U, Output = T>,
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: T,
    bits: S,
    rm: RoundingMode,
) -> T {
    if bits >= S::ZERO {
        x.shr_round(bits.unsigned_abs(), rm)
    } else {
        let abs = bits.unsigned_abs();
        if abs >= U::wrapping_from(T::WIDTH) {
            T::ZERO
        } else {
            x << bits.unsigned_abs()
        }
    }
}

fn shr_round_assign_primitive_signed<
    T: PrimitiveInt + ShlAssign<U> + ShrRoundAssign<U>,
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + UnsignedAbs<Output = U>,
>(
    x: &mut T,
    bits: S,
    rm: RoundingMode,
) {
    if bits >= S::ZERO {
        x.shr_round_assign(bits.unsigned_abs(), rm);
    } else {
        let abs = bits.unsigned_abs();
        if abs >= U::wrapping_from(T::WIDTH) {
            *x = T::ZERO;
        } else {
            *x <<= bits.unsigned_abs();
        }
    }
}

macro_rules! impl_shr_round_primitive_signed {
    ($t:ident) => {
        macro_rules! impl_shr_round_primitive_signed_inner {
            ($u:ident) => {
                impl ShrRound<$u> for $t {
                    type Output = $t;

                    /// Shifts a number right (divides it by a power of 2) and rounds according to
                    /// the specified rounding mode.
                    ///
                    /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to
                    /// using `>>`. To test whether `RoundingMode::Exact` can be passed, use
                    /// `self.divisible_by_power_of_2(bits)`. Rounding might only be necessary if
                    /// `bits` is non-negative.
                    ///
                    /// Let $q = \frac{x}{2^p}$:
                    ///
                    /// $f(x, p, \mathrm{Down}) = \operatorname{sgn}(q) \lfloor |q| \rfloor.$
                    ///
                    /// $f(x, p, \mathrm{Up}) = \operatorname{sgn}(q) \lceil |q| \rceil.$
                    ///
                    /// $f(x, p, \mathrm{Floor}) = \lfloor q \rfloor.$
                    ///
                    /// $f(x, p, \mathrm{Ceiling}) = \lceil q \rceil.$
                    ///
                    /// $$
                    /// f(x, p, \mathrm{Nearest}) = \begin{cases}
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
                    /// $f(x, p, \mathrm{Exact}) = q$, but panics if $q \notin \Z$.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `bits` is positive and `rm` is `RoundingMode::Exact` but `self` is
                    /// not divisible by $2^b$.
                    ///
                    /// # Examples
                    /// See [here](super::shr_round#shr_round).
                    #[inline]
                    fn shr_round(self, bits: $u, rm: RoundingMode) -> $t {
                        shr_round_primitive_signed(self, bits, rm)
                    }
                }

                impl ShrRoundAssign<$u> for $t {
                    /// Shifts a number right (divides it by a power of 2) and rounds according to
                    /// the specified rounding mode, in place.
                    ///
                    /// Passing `RoundingMode::Floor` or `RoundingMode::Down` is equivalent to
                    /// using `>>`. To test whether `RoundingMode::Exact` can be passed, use
                    /// `self.divisible_by_power_of_2(bits)`. Rounding might only be necessary if
                    /// `bits` is non-negative.
                    ///
                    /// See the [`ShrRound`](super::traits::ShrRound) documentation for details.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Panics
                    /// Panics if `bits` is positive and `rm` is `RoundingMode::Exact` but `self` is
                    /// not divisible by $2^b$.
                    ///
                    /// # Examples
                    /// See [here](super::shr_round#shr_round_assign).
                    #[inline]
                    fn shr_round_assign(&mut self, bits: $u, rm: RoundingMode) {
                        shr_round_assign_primitive_signed(self, bits, rm)
                    }
                }
            };
        }
        apply_to_signeds!(impl_shr_round_primitive_signed_inner);
    };
}
apply_to_primitive_ints!(impl_shr_round_primitive_signed);

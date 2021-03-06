use std::ops::{Neg, Shl, Shr};

use comparison::traits::Min;
use num::arithmetic::traits::{ArithmeticCheckedShl, UnsignedAbs};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::Zero;
use num::conversion::traits::{CheckedFrom, WrappingFrom};

fn _arithmetic_checked_shl_unsigned_unsigned<
    T: PrimitiveInt + Shl<U, Output = T> + Shr<U, Output = T>,
    U: Copy + Ord + WrappingFrom<u64>,
>(
    x: T,
    bits: U,
) -> Option<T> {
    if x == T::ZERO {
        Some(x)
    } else if bits >= U::wrapping_from(T::WIDTH) {
        None
    } else {
        let result = x << bits;
        if result >> bits == x {
            Some(result)
        } else {
            None
        }
    }
}

macro_rules! impl_arithmetic_checked_shl_unsigned_unsigned {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shl_unsigned_unsigned_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShl<$u> for $t {
                    type Output = $t;

                    /// Shifts `self` left (multiplies it by a power of 2). If the result is too
                    /// large to fit in a `$t`, `None` is returned.
                    ///
                    /// Zero may be shifted by any amount.
                    ///
                    /// $$
                    /// f(x, b) = \\begin{cases}
                    ///     \operatorname{Some}(2^b x) & 2^b x < 2^W \\\\
                    ///     \operatorname{None} & 2^b x \geq 2^W,
                    /// \\end{cases}
                    /// $$
                    /// where $W$ is `$t::WIDTH`.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::arithmetic::arithmetic_checked_shl`
                    /// module.
                    #[inline]
                    fn arithmetic_checked_shl(self, bits: $u) -> Option<$t> {
                        _arithmetic_checked_shl_unsigned_unsigned(self, bits)
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_arithmetic_checked_shl_unsigned_unsigned_inner);
    };
}
apply_to_unsigneds!(impl_arithmetic_checked_shl_unsigned_unsigned);

fn _arithmetic_checked_shl_unsigned_signed<
    T: ArithmeticCheckedShl<U, Output = T> + PrimitiveInt + Shr<U, Output = T>,
    U: Ord + WrappingFrom<u64>,
    S: Copy + Ord + UnsignedAbs<Output = U> + Zero,
>(
    x: T,
    bits: S,
) -> Option<T> {
    if bits >= S::ZERO {
        x.arithmetic_checked_shl(bits.unsigned_abs())
    } else {
        let abs_bits = bits.unsigned_abs();
        Some(if abs_bits >= U::wrapping_from(T::WIDTH) {
            T::ZERO
        } else {
            x >> abs_bits
        })
    }
}

macro_rules! impl_arithmetic_checked_shl_unsigned_signed {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shl_unsigned_signed_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShl<$u> for $t {
                    type Output = $t;

                    /// Shifts `self` left (multiplies it by a power of 2). If the result is too
                    /// large to fit in a `$t`, `None` is returned.
                    ///
                    /// Zero may be shifted by any amount, and any number may be shifted by any
                    /// negative amount; shifting by a negative amount with a high absolute value
                    /// returns `Some(0)`.
                    ///
                    /// $$
                    /// f(x, b) = \\begin{cases}
                    ///     \operatorname{Some}(2^b x) & b \geq 0 \\ \mathrm{and}\\ 2^b x < 2^W \\\\
                    ///     \operatorname{None} & b \geq 0 \\ \mathrm{and} \\ 2^b x \geq 2^W \\\\
                    ///     \operatorname{Some}(\lfloor x/2^{-b} \rfloor) & b < 0,
                    /// \\end{cases}
                    /// $$
                    /// where $W$ is `$t::WIDTH`.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::arithmetic::arithmetic_checked_shl`
                    /// module.
                    #[inline]
                    fn arithmetic_checked_shl(self, bits: $u) -> Option<$t> {
                        _arithmetic_checked_shl_unsigned_signed(self, bits)
                    }
                }
            };
        }
        apply_to_signeds!(impl_arithmetic_checked_shl_unsigned_signed_inner);
    };
}
apply_to_unsigneds!(impl_arithmetic_checked_shl_unsigned_signed);

fn _arithmetic_checked_shl_signed_unsigned<
    U: ArithmeticCheckedShl<B, Output = U> + Eq,
    S: CheckedFrom<U> + Copy + Min + Neg<Output = S> + Ord + UnsignedAbs<Output = U> + Zero,
    B,
>(
    x: S,
    bits: B,
) -> Option<S> {
    let abs = x.unsigned_abs();
    if x >= S::ZERO {
        abs.arithmetic_checked_shl(bits).and_then(S::checked_from)
    } else {
        abs.arithmetic_checked_shl(bits).and_then(|x| {
            if x == S::MIN.unsigned_abs() {
                Some(S::MIN)
            } else {
                S::checked_from(x).map(|y| -y)
            }
        })
    }
}

macro_rules! impl_arithmetic_checked_shl_signed_unsigned {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shl_signed_unsigned_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShl<$u> for $t {
                    type Output = $t;

                    /// Shifts `self` left (multiplies it by a power of 2). If the result is does
                    /// not fit in a `$t`, `None` is returned.
                    ///
                    /// Zero may be shifted by any amount.
                    ///
                    /// $$
                    /// f(x, b) = \\begin{cases}
                    ///     \operatorname{Some}(2^b x) & -2^{W-1} \leq 2^b x < 2^{W-1} \\\\
                    ///     \operatorname{None} &
                    ///         2^b x < -2^{W-1} \\ \mathrm{or} \\ 2^b x \geq 2^{W-1}, \\\\
                    /// \\end{cases}
                    /// $$
                    /// where $W$ is `$t::WIDTH`.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::arithmetic::arithmetic_checked_shl`
                    /// module.
                    #[inline]
                    fn arithmetic_checked_shl(self, bits: $u) -> Option<$t> {
                        _arithmetic_checked_shl_signed_unsigned(self, bits)
                    }
                }
            };
        }
        apply_to_unsigneds!(impl_arithmetic_checked_shl_signed_unsigned_inner);
    };
}
apply_to_signeds!(impl_arithmetic_checked_shl_signed_unsigned);

fn _arithmetic_checked_shl_signed_signed<
    T: ArithmeticCheckedShl<U, Output = T> + Neg<Output = T> + PrimitiveInt + Shr<U, Output = T>,
    U: Copy + Ord + WrappingFrom<u64> + Zero,
    S: Copy + Ord + UnsignedAbs<Output = U> + Zero,
>(
    x: T,
    bits: S,
) -> Option<T> {
    if bits >= S::ZERO {
        x.arithmetic_checked_shl(bits.unsigned_abs())
    } else {
        let width = U::wrapping_from(T::WIDTH);
        let abs_bits = bits.unsigned_abs();
        Some(if width != U::ZERO && abs_bits >= width {
            -T::iverson(x < T::ZERO)
        } else {
            x >> abs_bits
        })
    }
}

macro_rules! impl_arithmetic_checked_shl_signed_signed {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shl_signed_signed_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShl<$u> for $t {
                    type Output = $t;

                    /// Shifts `self` left (multiplies it by a power of 2). If the result is too
                    /// large to fit in a `$t`, `None` is returned.
                    ///
                    /// Zero may be shifted by any amount, and any number may be shifted by any
                    /// negative amount; shifting by a negative amount with a high absolute value
                    /// returns `Some(0)` if `self` is positive, and `Some(-1)` if `self` is
                    /// negative.
                    ///
                    /// $$
                    /// f(x, b) = \\begin{cases}
                    ///     \operatorname{Some}(2^b x) &
                    ///         b \geq 0 \\ \mathrm{and}\\ -2^{W-1} \leq 2^b x < 2^{W-1} \\\\
                    ///     \operatorname{None} &
                    ///         b \geq 0 \\ \mathrm{and} \\ (2^b x < -2^{W-1} \\ \mathrm{or}
                    ///         \\ 2^b x \geq 2^{W-1}) \\\\
                    ///     \operatorname{Some}(\lfloor x/2^{-b} \rfloor) & b < 0,
                    /// \\end{cases}
                    /// $$
                    /// where $W$ is `$t::WIDTH`.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::arithmetic::arithmetic_checked_shl`
                    /// module.
                    fn arithmetic_checked_shl(self, bits: $u) -> Option<$t> {
                        _arithmetic_checked_shl_signed_signed(self, bits)
                    }
                }
            };
        }
        apply_to_signeds!(impl_arithmetic_checked_shl_signed_signed_inner);
    };
}
apply_to_signeds!(impl_arithmetic_checked_shl_signed_signed);

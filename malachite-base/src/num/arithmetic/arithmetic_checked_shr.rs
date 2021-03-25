use num::arithmetic::traits::{ArithmeticCheckedShl, ArithmeticCheckedShr, UnsignedAbs};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::Zero;
use num::conversion::traits::WrappingFrom;
use std::ops::{Neg, Shr};

fn _arithmetic_checked_shr_unsigned_signed<
    T: ArithmeticCheckedShl<U, Output = T> + PrimitiveInt + Shr<U, Output = T>,
    U: Ord + WrappingFrom<u64>,
    S: Copy + Ord + UnsignedAbs<Output = U> + Zero,
>(
    x: T,
    bits: S,
) -> Option<T> {
    if bits < S::ZERO {
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

macro_rules! impl_arithmetic_checked_shr_unsigned_signed {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shr_unsigned_signed_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShr<$u> for $t {
                    type Output = $t;

                    /// Shifts `self` right (divides it by a power of 2). If the result is too large
                    /// to fit in a `$t`, `None` is returned.
                    ///
                    /// Zero may be shifted by any amount, and any number may be shifted by any non-
                    /// negative amount; shifting by a large amount returns `Some(0)`.
                    ///
                    /// $$
                    /// f(x, b) = \\begin{cases}
                    ///     \operatorname{Some}(\lfloor x/2^b \rfloor) & b \geq 0 \\\\
                    ///     \operatorname{Some}(2^{-b} x) &
                    ///         b < 0 \\ \mathrm{and} \\ 2^{-b} x < 2^W \\\\
                    ///     \operatorname{None} & b < 0 \\ \mathrm{and} \\ 2^{-b} x \geq 2^W, \\\\
                    /// \\end{cases}
                    /// $$
                    /// where $W$ is `$t::WIDTH`.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::arithmetic::arithmetic_checked_shr`
                    /// module.
                    #[inline]
                    fn arithmetic_checked_shr(self, bits: $u) -> Option<$t> {
                        _arithmetic_checked_shr_unsigned_signed(self, bits)
                    }
                }
            };
        }
        apply_to_signeds!(impl_arithmetic_checked_shr_unsigned_signed_inner);
    };
}
apply_to_unsigneds!(impl_arithmetic_checked_shr_unsigned_signed);

fn _arithmetic_checked_shr_signed_signed<
    T: ArithmeticCheckedShl<U, Output = T> + Neg<Output = T> + PrimitiveInt + Shr<U, Output = T>,
    U: Copy + Ord + WrappingFrom<u64> + Zero,
    S: Copy + Ord + UnsignedAbs<Output = U> + Zero,
>(
    x: T,
    bits: S,
) -> Option<T> {
    if bits < S::ZERO {
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

macro_rules! impl_arithmetic_checked_shr_signed_signed {
    ($t:ident) => {
        macro_rules! impl_arithmetic_checked_shr_signed_signed_inner {
            ($u:ident) => {
                impl ArithmeticCheckedShr<$u> for $t {
                    type Output = $t;

                    /// Shifts `self` right (divides it by a power of 2). If the result is too large
                    /// to fit in a `$t`, `None` is returned.
                    ///
                    /// Zero may be shifted by any amount, and any number may be shifted by any non-
                    /// negative amount; shifting by a large amount returns `Some(0)` if `self` is
                    /// positive, and `Some(-1)` if `self` is negative.
                    ///
                    /// $$
                    /// f(x, b) = \\begin{cases}
                    ///     \operatorname{Some}(\lfloor x/2^b \rfloor) & b \geq 0 \\\\
                    ///     \operatorname{Some}(2^{-b} x) &
                    ///         b < 0 \\ \mathrm{and}\\ -2^{W-1} \leq 2^{-b} x < 2^{W-1} \\\\
                    ///     \operatorname{None} &
                    ///         b < 0 \\ \mathrm{and} \\ (2^{-b} x < -2^{W-1} \\ \mathrm{or}
                    ///         \\ 2^{-b} x \geq 2^{W-1}), \\\\
                    /// \\end{cases}
                    /// $$
                    /// where $W$ is `$t::WIDTH`.
                    ///
                    /// # Worst-case complexity
                    /// Constant time and additional memory.
                    ///
                    /// # Examples
                    /// See the documentation of the `num::arithmetic::arithmetic_checked_shr`
                    /// module.
                    #[inline]
                    fn arithmetic_checked_shr(self, bits: $u) -> Option<$t> {
                        _arithmetic_checked_shr_signed_signed(self, bits)
                    }
                }
            };
        }
        apply_to_signeds!(impl_arithmetic_checked_shr_signed_signed_inner);
    };
}
apply_to_signeds!(impl_arithmetic_checked_shr_signed_signed);

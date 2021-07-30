use comparison::traits::{Max, Min};
use named::Named;
use num::arithmetic::traits::{ArithmeticCheckedShl, ShrRoundAssign};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::Zero;
use num::conversion::mantissa_and_exponent::sci_mantissa_and_exponent_with_rounding;
use num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, IntegerMantissaAndExponent, IsInteger, OverflowingFrom,
    RoundingFrom, SaturatingFrom, SciMantissaAndExponent, WrappingFrom,
};
use num::float::PrimitiveFloat;
use num::logic::traits::SignificantBits;
use rounding_modes::RoundingMode;
use std::ops::Neg;

/// This macro defines conversions from a type to itself.
macro_rules! identity_conversion {
    ($t:ty) => {
        impl CheckedFrom<$t> for $t {
            /// Converts a value to its own type. Since this conversion is always valid and always
            /// leaves the value unchanged, `None` is never returned.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::from` module.
            #[inline]
            fn checked_from(value: $t) -> Option<$t> {
                Some(value)
            }
        }

        impl WrappingFrom<$t> for $t {
            /// Converts a value to its own type. This conversion is always valid and always leaves
            /// the value unchanged.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::from` module.
            #[inline]
            fn wrapping_from(value: $t) -> $t {
                value
            }
        }

        impl SaturatingFrom<$t> for $t {
            /// Converts a value to its own type. This conversion is always valid and always leaves
            /// the value unchanged.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::from` module.
            #[inline]
            fn saturating_from(value: $t) -> $t {
                value
            }
        }

        impl OverflowingFrom<$t> for $t {
            /// Converts a value to its own type. Since this conversion is always valid and always
            /// leaves the value unchanged, the second component of the result is always false (no
            /// overflow).
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::from` module.
            #[inline]
            fn overflowing_from(value: $t) -> ($t, bool) {
                (value, false)
            }
        }

        impl ConvertibleFrom<$t> for $t {
            /// Checks whether a value is convertible to its own type. The result is always `true`.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::conversion::from` module.
            #[inline]
            fn convertible_from(_: $t) -> bool {
                true
            }
        }
    };
}

/// This macro defines conversions from type $a to type $b, where every value of type $a is
/// representable by a value of type $b.
macro_rules! lossless_conversion {
    ($a:ty, $b:ident) => {
        /// Converts a value to another type. Since this conversion is always lossless and leaves
        /// the value unchanged, `None` is never returned.
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        ///
        /// # Examples
        /// See the documentation of the `num::conversion::from` module.
        impl CheckedFrom<$a> for $b {
            #[inline]
            fn checked_from(value: $a) -> Option<$b> {
                Some($b::from(value))
            }
        }

        /// Converts a value to another type. This conversion is always valid and always leaves the
        /// value unchanged.
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        ///
        /// # Examples
        /// See the documentation of the `num::conversion::from` module.
        impl WrappingFrom<$a> for $b {
            #[inline]
            fn wrapping_from(value: $a) -> $b {
                $b::from(value)
            }
        }

        /// Converts a value to another type. This conversion is always valid and always leaves the
        /// value unchanged.
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        ///
        /// # Examples
        /// See the documentation of the `num::conversion::from` module.
        impl SaturatingFrom<$a> for $b {
            #[inline]
            fn saturating_from(value: $a) -> $b {
                $b::from(value)
            }
        }

        /// Converts a value to the value's type. Since this conversion is always valid and always
        /// leaves the value unchanged, the second component of the result is always false (no
        /// overflow).
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        ///
        /// # Examples
        /// See the documentation of the `num::conversion::from` module.
        impl OverflowingFrom<$a> for $b {
            #[inline]
            fn overflowing_from(value: $a) -> ($b, bool) {
                ($b::from(value), false)
            }
        }

        /// Checks whether a value is convertible to a different type. The result is always `true`.
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        ///
        /// # Examples
        /// See the documentation of the `num::conversion::from` module.
        impl ConvertibleFrom<$a> for $b {
            #[inline]
            fn convertible_from(_: $a) -> bool {
                true
            }
        }
    };
}

fn _checked_from_lossy<
    A: Copy + Ord + WrappingFrom<B> + Zero,
    B: Copy + Ord + WrappingFrom<A> + Zero,
>(
    value: A,
) -> Option<B> {
    let result = B::wrapping_from(value);
    if (result >= B::ZERO) == (value >= A::ZERO) && A::wrapping_from(result) == value {
        Some(result)
    } else {
        None
    }
}

fn _saturating_from_lossy<A: CheckedFrom<B> + Ord, B: Max + Min + WrappingFrom<A>>(value: A) -> B {
    if let Some(b_max) = A::checked_from(B::MAX) {
        if value >= b_max {
            return B::MAX;
        }
    }
    if let Some(b_min) = A::checked_from(B::MIN) {
        if value <= b_min {
            return B::MIN;
        }
    }
    B::wrapping_from(value)
}

fn _overflowing_from_lossy<
    A: Copy + Ord + WrappingFrom<B> + Zero,
    B: Copy + Ord + WrappingFrom<A> + Zero,
>(
    value: A,
) -> (B, bool) {
    let result = B::wrapping_from(value);
    (
        result,
        (result >= B::ZERO) != (value >= A::ZERO) || A::wrapping_from(result) != value,
    )
}

fn _convertible_from_lossy<
    A: Copy + Ord + WrappingFrom<B> + Zero,
    B: Copy + Ord + WrappingFrom<A> + Zero,
>(
    value: A,
) -> bool {
    let result = B::wrapping_from(value);
    (result >= B::ZERO) == (value >= A::ZERO) && A::wrapping_from(result) == value
}

/// This macro defines conversions from type $a to type $b, where not every value of type $a is
/// representable by a value of type $b.
macro_rules! lossy_conversion {
    ($a:ident, $b:ident) => {
        /// Converts a value to another type. If the value cannot be represented in the new type,
        /// `None` is returned.
        ///
        /// Let $W$ be `$b::WIDTH`.
        ///
        /// If the target type `$b` is unsigned,
        /// $$
        /// f_W(n) = \\begin{cases}
        ///     \operatorname{Some}(n) & 0 \leq n < 2^W \\\\
        ///     \operatorname{None} & \\text{otherwise}.
        /// \\end{cases}
        /// $$
        ///
        /// If the target type is signed,
        /// $$
        /// f_W(n) = \\begin{cases}
        ///     \operatorname{Some}(n) & -2^{W-1} \leq n < 2^{W-1}-1 \\\\
        ///     \operatorname{None} & \\text{otherwise}.
        /// \\end{cases}
        /// $$
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        ///
        /// # Examples
        /// See the documentation of the `num::conversion::from` module.
        impl CheckedFrom<$a> for $b {
            #[inline]
            fn checked_from(value: $a) -> Option<$b> {
                _checked_from_lossy(value)
            }
        }

        /// Converts a value to another type. If the value cannot be represented in the new type, it
        /// is wrapped.
        ///
        /// Let $W$ be `$b::WIDTH`.
        ///
        /// If the target type `$b` is unsigned,
        /// $f_W(n) = m$, where $m < 2^W$ and $n + 2^W k = m$ for some $k \in \Z$.
        ///
        /// If the target type is signed,
        /// $f_W(n) = m$, where $-2^{W-1} \leq m < 2^{W-1}$ and $n + 2^W k = m$ for some
        /// $k \in \Z$.
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        ///
        /// # Examples
        /// See the documentation of the `num::conversion::from` module.
        #[allow(clippy::cast_lossless)]
        impl WrappingFrom<$a> for $b {
            #[inline]
            fn wrapping_from(value: $a) -> $b {
                value as $b
            }
        }

        /// Converts a value to another type. If the value cannot be represented in the new type,
        /// the maximum or minimum value of the new type, whichever is closer, is returned.
        ///
        /// Let $W$ be `$b::WIDTH`.
        ///
        /// If the target type `$b` is unsigned,
        /// $$
        /// f_W(n) = \\begin{cases}
        ///     0 & n < 0 \\\\
        ///     2^W-1 & n \geq 2^W \\\\
        ///     n & \\text{otherwise}.
        /// \\end{cases}
        /// $$
        ///
        /// If the target type is signed,
        /// $$
        /// f_W(n) = \\begin{cases}
        ///     -2^{W-1} & n < -2^{W-1} \\\\
        ///     2^{W-1}-1 & n \geq 2^{W-1} \\\\
        ///     n & \\text{otherwise}.
        /// \\end{cases}
        /// $$
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        ///
        /// # Examples
        /// See the documentation of the `num::conversion::from` module.
        impl SaturatingFrom<$a> for $b {
            #[inline]
            fn saturating_from(value: $a) -> $b {
                _saturating_from_lossy(value)
            }
        }

        /// Converts a value to another type. If the value cannot be represented in the new type, it
        /// is wrapped. The second component of the result indicates whether overflow occurred.
        ///
        /// Let $W$ be `$b::WIDTH`.
        ///
        /// If the target type `$b` is unsigned,
        /// $f_W(n) = (m, k \neq 0)$, where $m < 2^W$ and $n + 2^W k = m$ for some $k \in \Z$.
        ///
        /// If the target type is signed,
        /// $f_W(n) = (m, k \neq 0)$, where $-2^{W-1} \leq m < 2^{W-1}$ and $n + 2^W k = m$ for some
        /// $k \in \Z$.
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        ///
        /// # Examples
        /// See the documentation of the `num::conversion::from` module.
        impl OverflowingFrom<$a> for $b {
            #[inline]
            fn overflowing_from(value: $a) -> ($b, bool) {
                _overflowing_from_lossy(value)
            }
        }

        /// Determines whether a value is convertible to a different type.
        ///
        /// Let $W$ be `$b::WIDTH`.
        ///
        /// If the target type `$b` is unsigned,
        /// $$
        /// f_W(n) = (0 \leq n < 2^W).
        /// $$
        ///
        /// If the target type is signed,
        /// $$
        /// f_W(n) = (-2^{W-1} \leq n < 2^{W-1}-1).
        /// $$
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        ///
        /// # Examples
        /// See the documentation of the `num::conversion::from` module.
        impl ConvertibleFrom<$a> for $b {
            #[inline]
            fn convertible_from(value: $a) -> bool {
                _convertible_from_lossy::<$a, $b>(value)
            }
        }
    };
}

/// This macro defines conversions from type $a to type $b, where the set of values representable by
/// type $a is a proper subset of the set of values representable by type $b.
macro_rules! proper_subset_conversion {
    ($a:ident, $b:ident) => {
        lossless_conversion!($a, $b);
        lossy_conversion!($b, $a);
    };
}

/// This macro defines conversions from type $a to type $b, where the set of values representable by
/// type $a is neither a subset nor a superset of the set of values representable by type $b.
macro_rules! no_containment_conversion {
    ($a:ident, $b:ident) => {
        lossy_conversion!($a, $b);
        lossy_conversion!($b, $a);
    };
}

apply_to_primitive_ints!(identity_conversion);

proper_subset_conversion!(u8, u16);
proper_subset_conversion!(u8, u32);
proper_subset_conversion!(u8, u64);
proper_subset_conversion!(u8, u128);
proper_subset_conversion!(u8, usize);
proper_subset_conversion!(u8, i16);
proper_subset_conversion!(u8, i32);
proper_subset_conversion!(u8, i64);
proper_subset_conversion!(u8, i128);
proper_subset_conversion!(u8, isize);
proper_subset_conversion!(u16, u32);
proper_subset_conversion!(u16, u64);
proper_subset_conversion!(u16, u128);
proper_subset_conversion!(u16, usize);
proper_subset_conversion!(u16, i32);
proper_subset_conversion!(u16, i64);
proper_subset_conversion!(u16, i128);
proper_subset_conversion!(u32, u64);
proper_subset_conversion!(u32, u128);
proper_subset_conversion!(u32, i64);
proper_subset_conversion!(u32, i128);
proper_subset_conversion!(u64, u128);
proper_subset_conversion!(u64, i128);
proper_subset_conversion!(i8, i16);
proper_subset_conversion!(i8, i32);
proper_subset_conversion!(i8, i64);
proper_subset_conversion!(i8, i128);
proper_subset_conversion!(i8, isize);
proper_subset_conversion!(i16, i32);
proper_subset_conversion!(i16, i64);
proper_subset_conversion!(i16, i128);
proper_subset_conversion!(i16, isize);
proper_subset_conversion!(i32, i64);
proper_subset_conversion!(i32, i128);
proper_subset_conversion!(i64, i128);

no_containment_conversion!(u8, i8);
no_containment_conversion!(u16, i8);
no_containment_conversion!(u16, i16);
no_containment_conversion!(u16, isize);
no_containment_conversion!(u32, usize);
no_containment_conversion!(u32, i8);
no_containment_conversion!(u32, i16);
no_containment_conversion!(u32, i32);
no_containment_conversion!(u32, isize);
no_containment_conversion!(u64, usize);
no_containment_conversion!(u64, i8);
no_containment_conversion!(u64, i16);
no_containment_conversion!(u64, i32);
no_containment_conversion!(u64, i64);
no_containment_conversion!(u64, isize);
no_containment_conversion!(u128, usize);
no_containment_conversion!(u128, i8);
no_containment_conversion!(u128, i16);
no_containment_conversion!(u128, i32);
no_containment_conversion!(u128, i64);
no_containment_conversion!(u128, i128);
no_containment_conversion!(u128, isize);
no_containment_conversion!(usize, i8);
no_containment_conversion!(usize, i16);
no_containment_conversion!(usize, i32);
no_containment_conversion!(usize, i64);
no_containment_conversion!(usize, i128);
no_containment_conversion!(usize, isize);
no_containment_conversion!(i32, isize);
no_containment_conversion!(i64, isize);
no_containment_conversion!(i128, isize);

macro_rules! impl_from_float_unsigned {
    ($u:ident) => {
        macro_rules! impl_from_float_unsigned_inner {
            ($f:ident) => {
                impl RoundingFrom<$u> for $f {
                    #[inline]
                    fn rounding_from(value: $u, rm: RoundingMode) -> $f {
                        if value == 0 {
                            return 0.0;
                        }
                        let (mantissa, exponent) =
                            sci_mantissa_and_exponent_with_rounding(value, rm).unwrap();
                        if let Some(f) = $f::from_sci_mantissa_and_exponent(
                            mantissa,
                            i64::wrapping_from(exponent),
                        ) {
                            f
                        } else {
                            match rm {
                                RoundingMode::Exact => {
                                    panic!("Value cannot be represented exactly as an {}", $f::NAME)
                                }
                                RoundingMode::Floor
                                | RoundingMode::Down
                                | RoundingMode::Nearest => $f::MAX_FINITE,
                                _ => $f::POSITIVE_INFINITY,
                            }
                        }
                    }
                }

                impl RoundingFrom<$f> for $u {
                    #[inline]
                    fn rounding_from(value: $f, rm: RoundingMode) -> $u {
                        assert!(!value.is_nan());
                        if value.is_infinite() {
                            let limit = if value > 0.0 { $u::MAX } else { 0 };
                            return match rm {
                                RoundingMode::Exact => {
                                    panic!("Value cannot be represented exactly as a {}", $u::NAME)
                                }
                                RoundingMode::Down | RoundingMode::Nearest => limit,
                                RoundingMode::Floor if value > 0.0 => limit,
                                RoundingMode::Ceiling if value < 0.0 => limit,
                                _ => panic!("Cannot round away from extreme value"),
                            };
                        }
                        if value == 0.0 {
                            return 0;
                        }
                        if value < 0.0 {
                            return match rm {
                                RoundingMode::Exact => {
                                    panic!("Value cannot be represented exactly as a {}", $u::NAME)
                                }
                                RoundingMode::Ceiling
                                | RoundingMode::Down
                                | RoundingMode::Nearest => 0,
                                _ => panic!("Value is less than 0 and rounding mode is {}", rm),
                            };
                        }
                        let (mut mantissa, exponent) = value.integer_mantissa_and_exponent();
                        let result = if exponent <= 0 {
                            mantissa.shr_round_assign(-exponent, rm);
                            $u::checked_from(mantissa)
                        } else {
                            $u::checked_from(mantissa)
                                .and_then(|n| n.arithmetic_checked_shl(exponent))
                        };
                        if let Some(n) = result {
                            n
                        } else {
                            match rm {
                                RoundingMode::Exact => {
                                    panic!("Value cannot be represented exactly as a {}", $u::NAME)
                                }
                                RoundingMode::Floor
                                | RoundingMode::Down
                                | RoundingMode::Nearest => $u::MAX,
                                _ => panic!(
                                    "Value is greater than {}::MAX and rounding mode is {}",
                                    $u::NAME,
                                    rm
                                ),
                            }
                        }
                    }
                }

                impl CheckedFrom<$u> for $f {
                    #[inline]
                    fn checked_from(value: $u) -> Option<$f> {
                        if value == 0 {
                            return Some(0.0);
                        }
                        let (mantissa, exponent) =
                            sci_mantissa_and_exponent_with_rounding(value, RoundingMode::Exact)?;
                        $f::from_sci_mantissa_and_exponent(mantissa, i64::wrapping_from(exponent))
                    }
                }

                impl CheckedFrom<$f> for $u {
                    #[inline]
                    fn checked_from(value: $f) -> Option<$u> {
                        if !value.is_finite() {
                            None
                        } else if value == 0.0 {
                            Some(0)
                        } else if value < 0.0 {
                            None
                        } else {
                            let (mantissa, exponent) = value.integer_mantissa_and_exponent();
                            if exponent < 0 {
                                None
                            } else {
                                $u::checked_from(mantissa)
                                    .and_then(|n| n.arithmetic_checked_shl(exponent))
                            }
                        }
                    }
                }

                impl ConvertibleFrom<$u> for $f {
                    #[inline]
                    fn convertible_from(value: $u) -> bool {
                        if value == 0 {
                            return true;
                        }
                        let precision = (value >> value.trailing_zeros()).significant_bits();
                        precision <= $f::MANTISSA_WIDTH + 1
                            && i64::wrapping_from(SciMantissaAndExponent::<$f, u64>::sci_exponent(
                                value,
                            )) <= $f::MAX_EXPONENT
                    }
                }

                impl ConvertibleFrom<$f> for $u {
                    #[inline]
                    fn convertible_from(value: $f) -> bool {
                        value >= 0.0
                            && value.is_integer()
                            && (value == 0.0
                                || value.sci_exponent() < i64::wrapping_from($u::WIDTH))
                    }
                }
            };
        }
        apply_to_primitive_floats!(impl_from_float_unsigned_inner);
    };
}
apply_to_unsigneds!(impl_from_float_unsigned);

macro_rules! impl_from_float_signed {
    ($u:ident, $i:ident) => {
        macro_rules! impl_from_float_signed_inner {
            ($f:ident) => {
                impl RoundingFrom<$i> for $f {
                    #[inline]
                    fn rounding_from(value: $i, rm: RoundingMode) -> $f {
                        let abs = value.unsigned_abs();
                        if value >= 0 {
                            $f::rounding_from(abs, rm)
                        } else {
                            -$f::rounding_from(abs, -rm)
                        }
                    }
                }

                impl RoundingFrom<$f> for $i {
                    #[inline]
                    fn rounding_from(value: $f, rm: RoundingMode) -> $i {
                        if value.is_infinite() {
                            let limit = if value > 0.0 { $i::MAX } else { $i::MIN };
                            return match rm {
                                RoundingMode::Exact => {
                                    panic!("Value cannot be represented exactly as a {}", $u::NAME)
                                }
                                RoundingMode::Down | RoundingMode::Nearest => limit,
                                RoundingMode::Floor if value > 0.0 => limit,
                                RoundingMode::Ceiling if value < 0.0 => limit,
                                _ => panic!("Cannot round away from extreme value"),
                            };
                        }
                        if value >= 0.0 {
                            let abs = $u::rounding_from(value, rm);
                            if let Some(n) = $i::checked_from(abs) {
                                n
                            } else {
                                match rm {
                                    RoundingMode::Exact => {
                                        panic!(
                                            "Value cannot be represented exactly as an {}",
                                            $i::NAME
                                        )
                                    }
                                    RoundingMode::Floor
                                    | RoundingMode::Down
                                    | RoundingMode::Nearest => $i::MAX,
                                    _ => panic!(
                                        "Value is greater than {}::MAX and rounding mode is {}",
                                        $i::NAME,
                                        rm
                                    ),
                                }
                            }
                        } else {
                            let abs = $u::rounding_from(-value, -rm);
                            let n = if abs == $i::MIN.unsigned_abs() {
                                Some($i::MIN)
                            } else {
                                $i::checked_from(abs).map(Neg::neg)
                            };
                            if let Some(n) = n {
                                n
                            } else {
                                match rm {
                                    RoundingMode::Exact => {
                                        panic!(
                                            "Value cannot be represented exactly as an {}",
                                            $i::NAME
                                        )
                                    }
                                    RoundingMode::Ceiling
                                    | RoundingMode::Down
                                    | RoundingMode::Nearest => $i::MIN,
                                    _ => panic!(
                                        "Value is smaller than {}::MIN and rounding mode is {}",
                                        $u::NAME,
                                        rm
                                    ),
                                }
                            }
                        }
                    }
                }

                impl CheckedFrom<$i> for $f {
                    #[inline]
                    fn checked_from(value: $i) -> Option<$f> {
                        let abs = value.unsigned_abs();
                        if value >= 0 {
                            $f::checked_from(abs)
                        } else {
                            $f::checked_from(abs).map(Neg::neg)
                        }
                    }
                }

                impl CheckedFrom<$f> for $i {
                    #[inline]
                    fn checked_from(value: $f) -> Option<$i> {
                        if !value.is_finite() {
                            return None;
                        }
                        if value >= 0.0 {
                            $i::checked_from($u::checked_from(value)?)
                        } else {
                            let abs = $u::checked_from(-value)?;
                            if abs == $i::MIN.unsigned_abs() {
                                Some($i::MIN)
                            } else {
                                $i::checked_from(abs).map(Neg::neg)
                            }
                        }
                    }
                }

                impl ConvertibleFrom<$i> for $f {
                    #[inline]
                    fn convertible_from(value: $i) -> bool {
                        $f::convertible_from(value.unsigned_abs())
                    }
                }

                impl ConvertibleFrom<$f> for $i {
                    #[allow(clippy::float_cmp)]
                    #[inline]
                    fn convertible_from(value: $f) -> bool {
                        if !value.is_integer() {
                            return false;
                        }
                        if value >= 0.0 {
                            value == 0.0 || value.sci_exponent() < i64::wrapping_from($u::WIDTH) - 1
                        } else {
                            let exponent = value.sci_exponent();
                            let limit = i64::wrapping_from($u::WIDTH) - 1;
                            value == 0.0
                                || exponent < limit
                                || exponent == limit
                                    && value
                                        == -$f::from_sci_mantissa_and_exponent(1.0, exponent)
                                            .unwrap()
                        }
                    }
                }
            };
        }
        apply_to_primitive_floats!(impl_from_float_signed_inner);
    };
}
apply_to_unsigned_signed_pairs!(impl_from_float_signed);

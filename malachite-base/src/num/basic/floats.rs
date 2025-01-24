// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::comparison::traits::{Max, Min};
use crate::named::Named;
use crate::num::arithmetic::traits::{
    Abs, AbsAssign, AddMul, AddMulAssign, Ceiling, CeilingAssign, CeilingLogBase2,
    CeilingLogBasePowerOf2, CheckedLogBase2, CheckedLogBasePowerOf2, Floor, FloorAssign,
    FloorLogBase2, FloorLogBasePowerOf2, IsPowerOf2, Ln, NegAssign, NextPowerOf2,
    NextPowerOf2Assign, Pow, PowAssign, PowerOf2, Reciprocal, ReciprocalAssign, Sign, Sqrt,
    SqrtAssign, Square, SquareAssign, SubMul, SubMulAssign,
};
use crate::num::basic::traits::{
    Infinity, NaN, NegativeInfinity, NegativeOne, NegativeZero, One, OneHalf, PrimeConstant,
    ThueMorseConstant, Two, Zero,
};
use crate::num::comparison::traits::PartialOrdAbs;
use crate::num::conversion::traits::{
    ConvertibleFrom, ExactInto, IntegerMantissaAndExponent, IsInteger, RawMantissaAndExponent,
    RoundingFrom, RoundingInto, SciMantissaAndExponent, WrappingFrom,
};
use crate::num::float::FmtRyuString;
use crate::num::logic::traits::{BitAccess, LowMask, SignificantBits, TrailingZeros};
use core::cmp::Ordering::*;
use core::fmt::{Debug, Display, LowerExp, UpperExp};
use core::iter::{Product, Sum};
use core::num::FpCategory;
use core::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use core::panic::RefUnwindSafe;
use core::str::FromStr;

/// This trait defines functions on primitive float types: [`f32`] and [`f64`].
///
/// Many of the functions here concern exponents and mantissas. We define three ways to express a
/// float, each with its own exponent and mantissa. In the following, let $x$ be an arbitrary
/// positive, finite, non-zero, non-NaN float. Let $M$ and $E$ be the mantissa width and exponent
/// width of the floating point type; for [`f32`]s, this is 23 and 8, and for [`f64`]s it's 52 and
/// 11.
///
/// In the following we assume that $x$ is positive, but you can easily extend these definitions to
/// negative floats by first taking their absolute value.
///
/// # raw form
/// The raw exponent and raw mantissa are the actual bit patterns used to represent the components
/// of $x$. The raw exponent $e_r$ is an integer in $[0, 2^E-2]$ and the raw mantissa $m_r$ is an
/// integer in $[0, 2^M-1]$. Since we are dealing with a nonzero $x$, we forbid $e_r$ and $m_r$ from
/// both being zero. We have
/// $$
/// x = \\begin{cases}
///     2^{2-2^{E-1}-M}m_r & \text{if} \quad e_r = 0, \\\\
///     2^{e_r-2^{E-1}+1}(2^{-M}m_r+1) & \textrm{otherwise},
/// \\end{cases}
/// $$
/// $$
/// e_r = \\begin{cases}
///     0 & \text{if} \quad x < 2^{2-2^{E-1}}, \\\\
///     \lfloor \log_2 x \rfloor + 2^{E-1} - 1 & \textrm{otherwise},
/// \\end{cases}
/// $$
/// $$
/// m_r = \\begin{cases}
///     2^{M+2^{E-1}-2}x & \text{if} \quad x < 2^{2-2^{E-1}}, \\\\
///     2^M \left ( \frac{x}{2^{\lfloor \log_2 x \rfloor}}-1\right ) & \textrm{otherwise}.
/// \\end{cases}
/// $$
///
/// # scientific form
/// We can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and $m_s$ is a rational number with $1
/// \leq m_s < 2$. If $x$ is a valid float, the scientific mantissa $m_s$ is always exactly
/// representable as a float of the same type. We have
/// $$
/// x = 2^{e_s}m_s,
/// $$
/// $$
/// e_s = \lfloor \log_2 x \rfloor,
/// $$
/// $$
/// m_s = \frac{x}{2^{\lfloor \log_2 x \rfloor}}.
/// $$
///
/// # integer form
/// We can also write $x = 2^{e_i}m_i$, where $e_i$ is an integer and $m_i$ is an odd integer. We
/// have
/// $$
/// x = 2^{e_i}m_i,
/// $$
/// $e_i$ is the unique integer such that $x/2^{e_i}$is an odd integer, and
/// $$
/// m_i = \frac{x}{2^{e_i}}.
/// $$
pub trait PrimitiveFloat:
    'static
    + Abs<Output = Self>
    + AbsAssign
    + Add<Output = Self>
    + AddAssign<Self>
    + AddMul<Output = Self>
    + AddMulAssign<Self, Self>
    + Ceiling<Output = Self>
    + CeilingAssign
    + CeilingLogBase2<Output = i64>
    + CeilingLogBasePowerOf2<u64, Output = i64>
    + CheckedLogBase2<Output = i64>
    + CheckedLogBasePowerOf2<u64, Output = i64>
    + ConvertibleFrom<u8>
    + ConvertibleFrom<u16>
    + ConvertibleFrom<u32>
    + ConvertibleFrom<u64>
    + ConvertibleFrom<u128>
    + ConvertibleFrom<usize>
    + ConvertibleFrom<i8>
    + ConvertibleFrom<i16>
    + ConvertibleFrom<i32>
    + ConvertibleFrom<i64>
    + ConvertibleFrom<i128>
    + ConvertibleFrom<isize>
    + Copy
    + Debug
    + Default
    + Display
    + Div<Output = Self>
    + DivAssign
    + Floor<Output = Self>
    + FloorAssign
    + FloorLogBase2<Output = i64>
    + FloorLogBasePowerOf2<u64, Output = i64>
    + FmtRyuString
    + From<f32>
    + FromStr
    + Infinity
    + IntegerMantissaAndExponent<u64, i64>
    + Into<f64>
    + IsInteger
    + IsPowerOf2
    + Ln
    + LowerExp
    + Min
    + Max
    + Mul<Output = Self>
    + MulAssign<Self>
    + Named
    + NaN
    + NegativeInfinity
    + NegativeZero
    + Neg<Output = Self>
    + NegAssign
    + NegativeOne
    + NextPowerOf2<Output = Self>
    + NextPowerOf2Assign
    + One
    + PartialEq<Self>
    + PartialOrd<Self>
    + PartialOrdAbs<Self>
    + Pow<i64, Output = Self>
    + Pow<Self, Output = Self>
    + PowAssign<i64>
    + PowAssign<Self>
    + PowerOf2<i64>
    + PrimeConstant
    + Product
    + RawMantissaAndExponent<u64, u64>
    + Reciprocal<Output = Self>
    + ReciprocalAssign
    + RefUnwindSafe
    + Rem<Output = Self>
    + RemAssign<Self>
    + RoundingFrom<u8>
    + RoundingFrom<u16>
    + RoundingFrom<u32>
    + RoundingFrom<u64>
    + RoundingFrom<u128>
    + RoundingFrom<usize>
    + RoundingFrom<i8>
    + RoundingFrom<i16>
    + RoundingFrom<i32>
    + RoundingFrom<i64>
    + RoundingFrom<i128>
    + RoundingFrom<isize>
    + RoundingInto<u8>
    + RoundingInto<u16>
    + RoundingInto<u32>
    + RoundingInto<u64>
    + RoundingInto<u128>
    + RoundingInto<usize>
    + RoundingInto<i8>
    + RoundingInto<i16>
    + RoundingInto<i32>
    + RoundingInto<i64>
    + RoundingInto<i128>
    + RoundingInto<isize>
    + SciMantissaAndExponent<Self, i64>
    + Sign
    + Sized
    + Sqrt<Output = Self>
    + SqrtAssign
    + Square<Output = Self>
    + SquareAssign
    + Sub<Output = Self>
    + SubAssign<Self>
    + SubMul<Output = Self>
    + SubMulAssign<Self, Self>
    + Sum<Self>
    + ThueMorseConstant
    + Two
    + UpperExp
    + Zero
{
    /// The number of bits taken up by the type.
    ///
    /// This is $M+E+1$. The three terms in the sum correspond to the width of the mantissa, the
    /// width of the exponent, and the sign bit.
    /// - For [`f32`]s, this is 32.
    /// - For [`f64`]s, this is 64.
    const WIDTH: u64;
    /// The number of bits taken up by the exponent.
    /// - For [`f32`]s, this is 8.
    /// - For [`f64`]s, this is 11.
    const EXPONENT_WIDTH: u64 = Self::WIDTH - Self::MANTISSA_WIDTH - 1;
    /// The number of bits taken up by the mantissa.
    /// - For [`f32`]s, this is 23.
    /// - For [`f64`]s, this is 52.
    const MANTISSA_WIDTH: u64;
    /// The smallest possible exponent of a float in the normal range. Any floats with smaller
    /// exponents are subnormal and thus have reduced precision. This is $2-2^{E-1}$.
    /// - For [`f32`]s, this is -126.
    /// - For [`f64`]s, this is -1022.
    const MIN_NORMAL_EXPONENT: i64 = -(1 << (Self::EXPONENT_WIDTH - 1)) + 2;
    /// The smallest possible exponent of a float. This is $2-2^{E-1}-M$.
    /// - For [`f32`]s, this is -149.
    /// - For [`f64`]s, this is -1074.
    const MIN_EXPONENT: i64 = Self::MIN_NORMAL_EXPONENT - (Self::MANTISSA_WIDTH as i64);
    /// The largest possible exponent of a float. This is $2^{E-1}-1$.
    /// - For [`f32`]s, this is 127.
    /// - For [`f64`]s, this is 1023.
    const MAX_EXPONENT: i64 = (1 << (Self::EXPONENT_WIDTH - 1)) - 1;
    /// The smallest positive float. This is $2^{2-2^{E-1}-M}$.
    /// - For [`f32`]s, this is $2^{-149}$, or `1.0e-45`.
    /// - For [`f64`]s, this is $2^{-1074}$, or `5.0e-324`.
    const MIN_POSITIVE_SUBNORMAL: Self;
    /// The largest float in the subnormal range. This is $2^{2-2^{E-1}-M}(2^M-1)$.
    /// - For [`f32`]s, this is $2^{-149}(2^{23}-1)$, or `1.1754942e-38`.
    /// - For [`f64`]s, this is $2^{-1074}(2^{52}-1)$, or `2.225073858507201e-308`.
    const MAX_SUBNORMAL: Self;
    /// The smallest positive normal float. This is $2^{2-2^{E-1}}$.
    /// - For [`f32`]s, this is $2^{-126}$, or `1.1754944e-38`.
    /// - For [`f64`]s, this is $2^{-1022}$, or `2.2250738585072014e-308`.
    const MIN_POSITIVE_NORMAL: Self;
    /// The largest finite float. This is $2^{2^{E-1}-1}(2-2^{-M})$.
    /// - For [`f32`]s, this is $2^{127}(2-2^{-23})$, or `3.4028235e38`.
    /// - For [`f64`]s, this is $2^{1023}(2-2^{-52})$, or `1.7976931348623157e308`.
    const MAX_FINITE: Self;
    /// The smallest positive integer that cannot be represented as a float. This is $2^{M+1}+1$.
    /// - For [`f32`]s, this is $2^{24}+1$, or 16777217.
    /// - For [`f64`]s, this is $2^{53}+1$, or 9007199254740993.
    const SMALLEST_UNREPRESENTABLE_UINT: u64;
    /// If you list all floats in increasing order, excluding NaN and giving negative and positive
    /// zero separate adjacent spots, this will be index of the last element, positive infinity. It
    /// is $2^{M+1}(2^E-1)+1$.
    /// - For [`f32`]s, this is $2^{32}-2^{24}+1$, or 4278190081.
    /// - For [`f64`]s, this is $2^{64}-2^{53}+1$, or 18437736874454810625.
    const LARGEST_ORDERED_REPRESENTATION: u64;

    fn is_nan(self) -> bool;

    fn is_infinite(self) -> bool;

    fn is_finite(self) -> bool;

    fn is_normal(self) -> bool;

    fn is_sign_positive(self) -> bool;

    fn is_sign_negative(self) -> bool;

    fn classify(self) -> FpCategory;

    fn to_bits(self) -> u64;

    fn from_bits(v: u64) -> Self;

    /// Tests whether `self` is negative zero.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::floats::PrimitiveFloat;
    ///
    /// assert!((-0.0).is_negative_zero());
    /// assert!(!0.0.is_negative_zero());
    /// assert!(!1.0.is_negative_zero());
    /// assert!(!f32::NAN.is_negative_zero());
    /// assert!(!f32::INFINITY.is_negative_zero());
    /// ```
    #[inline]
    fn is_negative_zero(self) -> bool {
        self.sign() == Less && self == Self::ZERO
    }

    /// If `self` is negative zero, returns positive zero; otherwise, returns `self`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::floats::PrimitiveFloat;
    /// use malachite_base::num::float::NiceFloat;
    ///
    /// assert_eq!(NiceFloat((-0.0).abs_negative_zero()), NiceFloat(0.0));
    /// assert_eq!(NiceFloat(0.0.abs_negative_zero()), NiceFloat(0.0));
    /// assert_eq!(NiceFloat(1.0.abs_negative_zero()), NiceFloat(1.0));
    /// assert_eq!(NiceFloat((-1.0).abs_negative_zero()), NiceFloat(-1.0));
    /// assert_eq!(NiceFloat(f32::NAN.abs_negative_zero()), NiceFloat(f32::NAN));
    /// ```
    #[inline]
    fn abs_negative_zero(self) -> Self {
        if self == Self::ZERO {
            Self::ZERO
        } else {
            self
        }
    }

    /// If `self` is negative zero, replaces it with positive zero; otherwise, leaves `self`
    /// unchanged.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::floats::PrimitiveFloat;
    /// use malachite_base::num::float::NiceFloat;
    ///
    /// let mut f = -0.0;
    /// f.abs_negative_zero_assign();
    /// assert_eq!(NiceFloat(f), NiceFloat(0.0));
    ///
    /// let mut f = 0.0;
    /// f.abs_negative_zero_assign();
    /// assert_eq!(NiceFloat(f), NiceFloat(0.0));
    ///
    /// let mut f = 1.0;
    /// f.abs_negative_zero_assign();
    /// assert_eq!(NiceFloat(f), NiceFloat(1.0));
    ///
    /// let mut f = -1.0;
    /// f.abs_negative_zero_assign();
    /// assert_eq!(NiceFloat(f), NiceFloat(-1.0));
    ///
    /// let mut f = f32::NAN;
    /// f.abs_negative_zero_assign();
    /// assert_eq!(NiceFloat(f), NiceFloat(f32::NAN));
    /// ```
    #[inline]
    fn abs_negative_zero_assign(&mut self) {
        if *self == Self::ZERO {
            *self = Self::ZERO;
        }
    }

    /// Returns the smallest float larger than `self`.
    ///
    /// Passing `-0.0` returns `0.0`; passing `NaN` or positive infinity panics.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is `NaN` or positive infinity.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::floats::PrimitiveFloat;
    /// use malachite_base::num::float::NiceFloat;
    ///
    /// assert_eq!(NiceFloat((-0.0f32).next_higher()), NiceFloat(0.0));
    /// assert_eq!(NiceFloat(0.0f32.next_higher()), NiceFloat(1.0e-45));
    /// assert_eq!(NiceFloat(1.0f32.next_higher()), NiceFloat(1.0000001));
    /// assert_eq!(NiceFloat((-1.0f32).next_higher()), NiceFloat(-0.99999994));
    /// ```
    fn next_higher(self) -> Self {
        assert!(!self.is_nan());
        if self.sign() == Greater {
            assert_ne!(self, Self::INFINITY);
            Self::from_bits(self.to_bits() + 1)
        } else if self == Self::ZERO {
            // negative zero -> positive zero
            Self::ZERO
        } else {
            Self::from_bits(self.to_bits() - 1)
        }
    }

    /// Returns the largest float smaller than `self`.
    ///
    /// Passing `0.0` returns `-0.0`; passing `NaN` or negative infinity panics.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is `NaN` or negative infinity.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::floats::PrimitiveFloat;
    /// use malachite_base::num::float::NiceFloat;
    ///
    /// assert_eq!(NiceFloat(0.0f32.next_lower()), NiceFloat(-0.0));
    /// assert_eq!(NiceFloat((-0.0f32).next_lower()), NiceFloat(-1.0e-45));
    /// assert_eq!(NiceFloat(1.0f32.next_lower()), NiceFloat(0.99999994));
    /// assert_eq!(NiceFloat((-1.0f32).next_lower()), NiceFloat(-1.0000001));
    /// ```
    fn next_lower(self) -> Self {
        assert!(!self.is_nan());
        if self.sign() == Less {
            assert_ne!(self, Self::NEGATIVE_INFINITY);
            Self::from_bits(self.to_bits() + 1)
        } else if self == Self::ZERO {
            // positive zero -> negative zero
            Self::NEGATIVE_ZERO
        } else {
            Self::from_bits(self.to_bits() - 1)
        }
    }

    /// Maps `self` to an integer. The map preserves ordering, and adjacent floats are mapped to
    /// adjacent integers.
    ///
    /// Negative infinity is mapped to 0, and positive infinity is mapped to the largest value,
    /// [`LARGEST_ORDERED_REPRESENTATION`](PrimitiveFloat::LARGEST_ORDERED_REPRESENTATION). Negative
    /// and positive zero are mapped to distinct adjacent values. Passing in `NaN` panics.
    ///
    /// The inverse operation is
    /// [`from_ordered_representation`](PrimitiveFloat::from_ordered_representation).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is `NaN`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::floats::PrimitiveFloat;
    /// use malachite_base::num::basic::traits::NegativeInfinity;
    ///
    /// assert_eq!(f32::NEGATIVE_INFINITY.to_ordered_representation(), 0);
    /// assert_eq!((-0.0f32).to_ordered_representation(), 2139095040);
    /// assert_eq!(0.0f32.to_ordered_representation(), 2139095041);
    /// assert_eq!(1.0f32.to_ordered_representation(), 3204448257);
    /// assert_eq!(f32::INFINITY.to_ordered_representation(), 4278190081);
    /// ```
    fn to_ordered_representation(self) -> u64 {
        assert!(!self.is_nan());
        let bits = self.to_bits();
        if self.sign() == Greater {
            (u64::low_mask(Self::EXPONENT_WIDTH) << Self::MANTISSA_WIDTH) + bits + 1
        } else {
            (u64::low_mask(Self::EXPONENT_WIDTH + 1) << Self::MANTISSA_WIDTH) - bits
        }
    }

    /// Maps a non-negative integer, less than or equal to
    /// [`LARGEST_ORDERED_REPRESENTATION`](PrimitiveFloat::LARGEST_ORDERED_REPRESENTATION), to a
    /// float. The map preserves ordering, and adjacent integers are mapped to adjacent floats.
    ///
    /// Zero is mapped to negative infinity, and
    /// [`LARGEST_ORDERED_REPRESENTATION`](PrimitiveFloat::LARGEST_ORDERED_REPRESENTATION) is mapped
    /// to positive infinity. Negative and positive zero are produced by two distinct adjacent
    /// integers. `NaN` is never produced.
    ///
    /// The inverse operation is
    /// [`to_ordered_representation`](PrimitiveFloat::to_ordered_representation).
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is greater than
    /// [`LARGEST_ORDERED_REPRESENTATION`](PrimitiveFloat::LARGEST_ORDERED_REPRESENTATION).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::floats::PrimitiveFloat;
    /// use malachite_base::num::basic::traits::NegativeInfinity;
    ///
    /// assert_eq!(f32::from_ordered_representation(0), f32::NEGATIVE_INFINITY);
    /// assert_eq!(f32::from_ordered_representation(2139095040), -0.0f32);
    /// assert_eq!(f32::from_ordered_representation(2139095041), 0.0f32);
    /// assert_eq!(f32::from_ordered_representation(3204448257), 1.0f32);
    /// assert_eq!(f32::from_ordered_representation(4278190081), f32::INFINITY);
    /// ```
    fn from_ordered_representation(n: u64) -> Self {
        let zero_exp = u64::low_mask(Self::EXPONENT_WIDTH) << Self::MANTISSA_WIDTH;
        let f = if n <= zero_exp {
            Self::from_bits((u64::low_mask(Self::EXPONENT_WIDTH + 1) << Self::MANTISSA_WIDTH) - n)
        } else {
            let f = Self::from_bits(n - zero_exp - 1);
            assert_eq!(f.sign(), Greater);
            f
        };
        assert!(!f.is_nan());
        f
    }

    /// Returns the precision of a nonzero finite floating-point number.
    ///
    /// The precision is the number of significant bits of the integer mantissa. For example, the
    /// floats with precision 1 are the powers of 2, those with precision 2 are 3 times a power of
    /// 2, those with precision 3 are 5 or 7 times a power of 2, and so on.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is zero, infinite, or `NaN`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::floats::PrimitiveFloat;
    ///
    /// assert_eq!(1.0.precision(), 1);
    /// assert_eq!(2.0.precision(), 1);
    /// assert_eq!(3.0.precision(), 2);
    /// assert_eq!(1.5.precision(), 2);
    /// assert_eq!(1.234f32.precision(), 23);
    /// ```
    fn precision(self) -> u64 {
        assert!(self.is_finite());
        assert!(self != Self::ZERO);
        let (mut mantissa, exponent) = self.raw_mantissa_and_exponent();
        if exponent == 0 {
            mantissa.significant_bits() - TrailingZeros::trailing_zeros(mantissa)
        } else {
            mantissa.set_bit(Self::MANTISSA_WIDTH);
            Self::MANTISSA_WIDTH + 1 - TrailingZeros::trailing_zeros(mantissa)
        }
    }

    /// Given a scientific exponent, returns the largest possible precision for a float with that
    /// exponent.
    ///
    /// See the documentation of the [`precision`](PrimitiveFloat::precision) function for a
    /// definition of precision.
    ///
    /// For exponents greater than or equal to
    /// [`MIN_NORMAL_EXPONENT`](PrimitiveFloat::MIN_NORMAL_EXPONENT), the maximum precision is one
    /// more than the mantissa width. For smaller exponents (corresponding to the subnormal range),
    /// the precision is lower.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `exponent` is less than [`MIN_EXPONENT`](PrimitiveFloat::MIN_EXPONENT) or greater
    /// than [`MAX_EXPONENT`](PrimitiveFloat::MAX_EXPONENT).
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::basic::floats::PrimitiveFloat;
    ///
    /// assert_eq!(f32::max_precision_for_sci_exponent(0), 24);
    /// assert_eq!(f32::max_precision_for_sci_exponent(127), 24);
    /// assert_eq!(f32::max_precision_for_sci_exponent(-149), 1);
    /// assert_eq!(f32::max_precision_for_sci_exponent(-148), 2);
    /// assert_eq!(f32::max_precision_for_sci_exponent(-147), 3);
    /// ```
    fn max_precision_for_sci_exponent(exponent: i64) -> u64 {
        assert!(exponent >= Self::MIN_EXPONENT);
        assert!(exponent <= Self::MAX_EXPONENT);
        if exponent >= Self::MIN_NORMAL_EXPONENT {
            Self::MANTISSA_WIDTH + 1
        } else {
            u64::wrapping_from(exponent - Self::MIN_EXPONENT) + 1
        }
    }
}

/// Defines basic trait implementations for floating-point types.
macro_rules! impl_basic_traits_primitive_float {
    (
        $t: ident,
        $width: expr,
        $min_positive_subnormal: expr,
        $max_subnormal: expr,
        $min_positive_normal: expr,
        $thue_morse_constant: expr,
        $prime_constant: expr
    ) => {
        impl PrimitiveFloat for $t {
            const WIDTH: u64 = $width;
            const MANTISSA_WIDTH: u64 = ($t::MANTISSA_DIGITS as u64) - 1;

            const MAX_FINITE: Self = $t::MAX;
            const MIN_POSITIVE_SUBNORMAL: Self = $min_positive_subnormal;
            const MAX_SUBNORMAL: Self = $max_subnormal;
            const MIN_POSITIVE_NORMAL: Self = $min_positive_normal;
            const SMALLEST_UNREPRESENTABLE_UINT: u64 = (1 << (Self::MANTISSA_WIDTH + 1)) + 1;
            // We can't shift by $width when $width is 64, so we shift by $width - 1 and then by 1
            const LARGEST_ORDERED_REPRESENTATION: u64 = (1u64 << ($width - 1) << 1)
                .wrapping_sub(((1 << Self::MANTISSA_WIDTH) - 1) << 1)
                - 1;

            #[inline]
            fn is_nan(self) -> bool {
                $t::is_nan(self)
            }

            #[inline]
            fn is_infinite(self) -> bool {
                $t::is_infinite(self)
            }

            #[inline]
            fn is_finite(self) -> bool {
                $t::is_finite(self)
            }

            #[inline]
            fn is_normal(self) -> bool {
                $t::is_normal(self)
            }

            #[inline]
            fn is_sign_positive(self) -> bool {
                $t::is_sign_positive(self)
            }

            #[inline]
            fn is_sign_negative(self) -> bool {
                $t::is_sign_negative(self)
            }

            #[inline]
            fn classify(self) -> FpCategory {
                $t::classify(self)
            }

            #[inline]
            fn to_bits(self) -> u64 {
                u64::wrapping_from($t::to_bits(self))
            }

            #[inline]
            fn from_bits(v: u64) -> $t {
                $t::from_bits(v.exact_into())
            }
        }

        impl_named!($t);

        /// The constant 0.
        impl Zero for $t {
            const ZERO: $t = 0.0;
        }

        /// The constant 1.
        impl One for $t {
            const ONE: $t = 1.0;
        }

        /// The constant 2.
        impl Two for $t {
            const TWO: $t = 2.0;
        }

        /// The constant 1/2.
        impl OneHalf for $t {
            const ONE_HALF: $t = 0.5;
        }

        /// The constant -1.0 for primitive floating-point types.
        impl NegativeOne for $t {
            const NEGATIVE_ONE: $t = -1.0;
        }

        /// The constant -0.0 for primitive floating-point types.
        impl NegativeZero for $t {
            const NEGATIVE_ZERO: $t = -0.0;
        }

        /// The constant Infinity for primitive floating-point types.
        impl Infinity for $t {
            const INFINITY: $t = $t::INFINITY;
        }

        /// The constant -Infinity for primitive floating-point types.
        impl NegativeInfinity for $t {
            const NEGATIVE_INFINITY: $t = $t::NEG_INFINITY;
        }

        /// The constant NaN for primitive floating-point types.
        impl NaN for $t {
            const NAN: $t = $t::NAN;
        }

        /// The lowest value representable by this type, negative infinity.
        impl Min for $t {
            const MIN: $t = $t::NEGATIVE_INFINITY;
        }

        /// The highest value representable by this type, positive infinity.
        impl Max for $t {
            const MAX: $t = $t::INFINITY;
        }

        /// The Thue-Morse constant.
        impl ThueMorseConstant for $t {
            const THUE_MORSE_CONSTANT: $t = $thue_morse_constant;
        }

        /// The prime constant.
        impl PrimeConstant for $t {
            const PRIME_CONSTANT: $t = $prime_constant;
        }
    };
}
impl_basic_traits_primitive_float!(
    f32,
    32,
    1.0e-45,
    1.1754942e-38,
    1.1754944e-38,
    0.41245404,
    0.4146825
);
impl_basic_traits_primitive_float!(
    f64,
    64,
    5.0e-324,
    2.225073858507201e-308,
    2.2250738585072014e-308,
    0.4124540336401076,
    0.41468250985111166
);

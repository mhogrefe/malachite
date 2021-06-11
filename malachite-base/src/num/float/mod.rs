use comparison::traits::{Max, Min};
use named::Named;
use num::arithmetic::traits::{ModPowerOf2, NegAssign};
use num::basic::integers::PrimitiveInt;
use num::basic::traits::{NegativeOne, One, Two, Zero};
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::{ExactFrom, ExactInto, WrappingFrom};
use num::float::nice_float::FmtRyuString;
use num::logic::traits::{BitAccess, LowMask, SignificantBits, TrailingZeros};
use std::fmt::{Debug, Display, LowerExp, UpperExp};
use std::iter::{Product, Sum};
use std::num::FpCategory;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use std::str::FromStr;

/// This trait defines functions on primitive float types: `f32` and `f64`.
///
/// Many of the functions here concern exponents and mantissas. We define three ways to express a
/// float, each with its own exponent and mantissa. In the following, let $x$ be an arbitrary
/// positive, finite, non-zero, non-NaN float. Let $M$ and $E$ be the mantissa width and exponent
/// width of the floating point type; for `f32`s, this is 23 and 8, and for `f64`s it's 52 and 11.
///
/// These mantissas and exponents are defined for negative numbers too; just work with the absolute
/// value.
///
/// # raw form
/// The raw exponent and raw mantissa are the actual bit patterns used to represent the components
/// of $x$. The raw exponent $e_r$ is an integer in $[0, 2^E-2]$ and the raw mantissa $m_r$ is an
/// integer in $[0, 2^M-1]$. Since we are dealing with a nonzero $x$, we forbid $e_r$ and $m_r$
/// from both being zero. We have
/// $$
/// x = \\begin{cases}
///     2^{2-2^{E-1}-M}m_r & e_r = 0 \\\\
///     2^{e_r-2^{E-1}+1}(2^{-M}m_r+1) & \textrm{otherwise},
/// \\end{cases}
/// $$
/// $$
/// e_r = \\begin{cases}
///     0 & x < 2^{2-2^{E-1}} \\\\
///     \lfloor \log_2 x \rfloor + 2^{E-1} - 1 & \textrm{otherwise},
/// \\end{cases}
/// $$
/// $$
/// m_r = \\begin{cases}
///     2^{M+2^{E-1}-2}x & x < 2^{2-2^{E-1}} \\\\
///     2^M \left ( \frac{x}{2^{\lfloor \log_2 x \rfloor}}-1\right ) & \textrm{otherwise}.
/// \\end{cases}
/// $$
///
/// # scientific form
/// We can write $x = 2^{e_s}m_s$, where $e_s$ is an integer and $m_s$ is a rational number with
/// $1 \leq m_s < 2$. If $x$ is a valid float, the scientific mantissa $m_s$ is always exactly
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
    + Add<Output = Self>
    + AddAssign<Self>
    + Copy
    + Debug
    + Default
    + Display
    + Div<Output = Self>
    + DivAssign
    + Display
    + FmtRyuString
    + From<f32>
    + FromStr
    + Into<f64>
    + LowerExp
    + Min
    + Max
    + Mul<Output = Self>
    + MulAssign<Self>
    + Named
    + Neg<Output = Self>
    + NegAssign
    + NegativeOne
    + One
    + PartialEq<Self>
    + PartialOrd<Self>
    + Product
    + Rem<Output = Self>
    + RemAssign<Self>
    + Sized
    + Sub<Output = Self>
    + SubAssign<Self>
    + Sum<Self>
    + Two
    + UpperExp
    + Zero
{
    type UnsignedOfEqualWidth: PrimitiveUnsigned;

    /// $M+E+1$
    /// - For `f32`, this is 32.
    /// - For `f64`, this is 64.
    const WIDTH: u64 = Self::UnsignedOfEqualWidth::WIDTH;
    /// - For `f32`, this is 8.
    /// - For `f64`, this is 11.
    const EXPONENT_WIDTH: u64 = Self::WIDTH - Self::MANTISSA_WIDTH - 1;
    /// - For `f32`, this is 23.
    /// - For `f64`, this is 52.
    const MANTISSA_WIDTH: u64;
    /// $2-2^{E-1}$
    /// - For `f32`, this is -126.
    /// - For `f64`, this is -1022.
    const MIN_NORMAL_EXPONENT: i64 = -(1 << (Self::EXPONENT_WIDTH - 1)) + 2;
    /// $2-2^{E-1}-M$
    /// - For `f32`, this is -149.
    /// - For `f64`, this is -1074.
    const MIN_EXPONENT: i64 = Self::MIN_NORMAL_EXPONENT - (Self::MANTISSA_WIDTH as i64);
    /// $2^{E-1}-1$
    /// - For `f32`, this is 127.
    /// - For `f64`, this is 1023.
    const MAX_EXPONENT: i64 = (1 << (Self::EXPONENT_WIDTH - 1)) - 1;
    /// $2^{2-2^{E-1}-M}$
    /// - For `f32`, this is $2^{-149}$, or `1.0e-45`.
    /// - For `f64`, this is $2^{-1074}$, or `5.0e-324`.
    const MIN_POSITIVE_SUBNORMAL: Self;
    /// $2^{2-2^{E-1}-M}(2^M-1)$
    /// - For `f32`, this is $2^{-149}(2^{23}-1)$, or `1.1754942e-38`.
    /// - For `f64`, this is $2^{-1074}(2^{52}-1)$, or `2.225073858507201e-308`.
    const MAX_SUBNORMAL: Self;
    /// $2^{2-2^{E-1}}$
    /// - For `f32`, this is $2^{-126}$, or `1.1754944e-38`.
    /// - For `f64`, this is $2^{-1022}$, or `2.2250738585072014e-308`.
    const MIN_POSITIVE_NORMAL: Self;
    /// $2^{2^{E-1}-1}(2-2^{-M})$
    /// - For `f32`, this is $2^{127}(2-2^{-23})$, or `3.4028235e38`.
    /// - For `f64`, this is $2^{1023}(2-2^{-52})$, or `1.7976931348623157e308`.
    const MAX_FINITE: Self;
    const NEGATIVE_ZERO: Self;
    const POSITIVE_INFINITY: Self;
    const NEGATIVE_INFINITY: Self;
    const NAN: Self;
    /// $2^{M+1}+1$
    /// - For `f32`, this is $2^{24}+1$, or 16777217.
    /// - For `f64`, this is $2^{53}+1$, or 9007199254740993.
    const SMALLEST_UNREPRESENTABLE_UINT: Self::UnsignedOfEqualWidth;
    /// $2^{M+1}(2^E-1)+1$
    /// - For `f32`, this is $2^{32}-2^{24}+1$, or 4278190081.
    /// - For `f64`, this is $2^{64}-2^{53}+1$, or 18437736874454810625.
    const LARGEST_ORDERED_REPRESENTATION: Self::UnsignedOfEqualWidth;

    fn is_nan(self) -> bool;

    fn is_infinite(self) -> bool;

    fn is_finite(self) -> bool;

    fn is_normal(self) -> bool;

    fn classify(self) -> FpCategory;

    fn is_sign_positive(self) -> bool;

    fn is_sign_negative(self) -> bool;

    fn to_bits(self) -> Self::UnsignedOfEqualWidth;

    fn from_bits(v: Self::UnsignedOfEqualWidth) -> Self;

    fn floor(self) -> Self;

    fn ceil(self) -> Self;

    /// Tests whether `self` is negative zero.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::float::PrimitiveFloat;
    ///
    /// assert!((-0.0).is_negative_zero());
    /// assert!(!0.0.is_negative_zero());
    /// assert!(!1.0.is_negative_zero());
    /// assert!(!f32::NAN.is_negative_zero());
    /// assert!(!f32::POSITIVE_INFINITY.is_negative_zero());
    /// ```
    #[inline]
    fn is_negative_zero(self) -> bool {
        self.is_sign_negative() && self == Self::ZERO
    }

    /// If `self` is negative zero, returns positive zero; otherwise, returns `self`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::float::nice_float::NiceFloat;
    /// use malachite_base::num::float::PrimitiveFloat;
    ///
    /// assert_eq!(NiceFloat((-0.0).abs_negative_zero()), NiceFloat(0.0));
    /// assert_eq!(NiceFloat(0.0.abs_negative_zero()), NiceFloat(0.0));
    /// assert_eq!(NiceFloat(1.0.abs_negative_zero()), NiceFloat(1.0));
    /// assert_eq!(NiceFloat((-1.0).abs_negative_zero()), NiceFloat(-1.0));
    /// assert_eq!(NiceFloat(f32::NAN.abs_negative_zero()), NiceFloat(f32::NAN));
    /// ```
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
    /// use malachite_base::num::float::nice_float::NiceFloat;
    /// use malachite_base::num::float::PrimitiveFloat;
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
    /// use malachite_base::num::float::nice_float::NiceFloat;
    /// use malachite_base::num::float::PrimitiveFloat;
    ///
    /// assert_eq!(NiceFloat((-0.0f32).next_higher()), NiceFloat(0.0));
    /// assert_eq!(NiceFloat(0.0f32.next_higher()), NiceFloat(1.0e-45));
    /// assert_eq!(NiceFloat(1.0f32.next_higher()), NiceFloat(1.0000001));
    /// assert_eq!(NiceFloat((-1.0f32).next_higher()), NiceFloat(-0.99999994));
    /// ```
    fn next_higher(self) -> Self {
        assert!(!self.is_nan());
        if self.is_sign_positive() {
            assert_ne!(self, Self::POSITIVE_INFINITY);
            Self::from_bits(self.to_bits() + Self::UnsignedOfEqualWidth::ONE)
        } else if self == Self::ZERO {
            // negative zero -> positive zero
            Self::ZERO
        } else {
            Self::from_bits(self.to_bits() - Self::UnsignedOfEqualWidth::ONE)
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
    /// use malachite_base::num::float::nice_float::NiceFloat;
    /// use malachite_base::num::float::PrimitiveFloat;
    ///
    /// assert_eq!(NiceFloat(0.0f32.next_lower()), NiceFloat(-0.0));
    /// assert_eq!(NiceFloat((-0.0f32).next_lower()), NiceFloat(-1.0e-45));
    /// assert_eq!(NiceFloat(1.0f32.next_lower()), NiceFloat(0.99999994));
    /// assert_eq!(NiceFloat((-1.0f32).next_lower()), NiceFloat(-1.0000001));
    /// ```
    fn next_lower(self) -> Self {
        assert!(!self.is_nan());
        if self.is_sign_negative() {
            assert_ne!(self, Self::NEGATIVE_INFINITY);
            Self::from_bits(self.to_bits() + Self::UnsignedOfEqualWidth::ONE)
        } else if self == Self::ZERO {
            // positive zero -> negative zero
            Self::NEGATIVE_ZERO
        } else {
            Self::from_bits(self.to_bits() - Self::UnsignedOfEqualWidth::ONE)
        }
    }

    /// Maps `self` to an integer. The map preserves ordering, and adjacent floats are mapped to
    /// adjacent integers.
    ///
    /// Negative infinity is mapped to 0, and positive infinity is mapped to the largest value,
    /// `LARGEST_ORDERED_REPRESENTATION`. Negative and positive zero are mapped to distinct
    /// adjacent values. Passing in `NaN` panics.
    ///
    /// The inverse operation is `from_ordered_representation`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is `NaN`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::float::PrimitiveFloat;
    ///
    /// assert_eq!(f32::NEGATIVE_INFINITY.to_ordered_representation(), 0);
    /// assert_eq!((-0.0f32).to_ordered_representation(), 2139095040);
    /// assert_eq!(0.0f32.to_ordered_representation(), 2139095041);
    /// assert_eq!(1.0f32.to_ordered_representation(), 3204448257);
    /// assert_eq!(f32::POSITIVE_INFINITY.to_ordered_representation(), 4278190081);
    /// ```
    fn to_ordered_representation(self) -> Self::UnsignedOfEqualWidth {
        assert!(!self.is_nan());
        let bits = self.to_bits();
        if self.is_sign_positive() {
            (Self::UnsignedOfEqualWidth::low_mask(Self::EXPONENT_WIDTH) << Self::MANTISSA_WIDTH)
                + bits
                + Self::UnsignedOfEqualWidth::ONE
        } else {
            (Self::UnsignedOfEqualWidth::low_mask(Self::EXPONENT_WIDTH + 1) << Self::MANTISSA_WIDTH)
                - bits
        }
    }

    /// Maps a non-negative integer, less than or equal to `LARGEST_ORDERED_REPRESENTATION`, to a
    /// float. The map preserves ordering, and adjacent integers are mapped to adjacent floats.
    ///
    /// Zero is mapped to negative infinity, and `LARGEST_ORDERED_REPRESENTATION` is mapped to
    /// positive infinity. Negative and positive zero are produced by two distinct adjacent
    /// integers. `NaN` is never produced.
    ///
    /// The inverse operation is `to_ordered_representation`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Panics
    /// Panics if `self` is greater than `LARGEST_ORDERED_REPRESENTATION`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::float::PrimitiveFloat;
    ///
    /// assert_eq!(f32::from_ordered_representation(0), f32::NEGATIVE_INFINITY);
    /// assert_eq!(f32::from_ordered_representation(2139095040), -0.0f32);
    /// assert_eq!(f32::from_ordered_representation(2139095041), 0.0f32);
    /// assert_eq!(f32::from_ordered_representation(3204448257), 1.0f32);
    /// assert_eq!(f32::from_ordered_representation(4278190081), f32::POSITIVE_INFINITY);
    /// ```
    fn from_ordered_representation(n: Self::UnsignedOfEqualWidth) -> Self {
        let offset =
            Self::UnsignedOfEqualWidth::low_mask(Self::EXPONENT_WIDTH) << Self::MANTISSA_WIDTH;
        let f = if n <= offset {
            Self::from_bits(
                (Self::UnsignedOfEqualWidth::low_mask(Self::EXPONENT_WIDTH + 1)
                    << Self::MANTISSA_WIDTH)
                    - n,
            )
        } else {
            let f = Self::from_bits(n - offset - Self::UnsignedOfEqualWidth::ONE);
            assert!(f.is_sign_positive());
            f
        };
        assert!(!f.is_nan());
        f
    }

    fn raw_exponent(self) -> u64 {
        let exponent: u64 = (self.to_bits() >> Self::MANTISSA_WIDTH).exact_into();
        exponent.mod_power_of_2(Self::EXPONENT_WIDTH)
    }

    fn exponentz(self) -> i64 {
        assert!(self.is_finite());
        assert!(self != Self::ZERO);
        i64::exact_from(self.raw_exponent()) - Self::MAX_EXPONENT
    }

    fn raw_mantissa_and_exponent(self) -> (Self::UnsignedOfEqualWidth, u64) {
        let bits = self.to_bits();
        let mantissa = bits.mod_power_of_2(Self::MANTISSA_WIDTH);
        let exponent: u64 = (bits >> Self::MANTISSA_WIDTH).exact_into();
        let exponent = exponent.mod_power_of_2(Self::EXPONENT_WIDTH);
        (mantissa, exponent)
    }

    fn integer_mantissa_and_exponent(self) -> (Self::UnsignedOfEqualWidth, i64) {
        assert!(self.is_finite());
        assert!(self != Self::ZERO);
        let (raw_mantissa, raw_exponent) = self.raw_mantissa_and_exponent();
        let mut mantissa;
        let exponent;
        if raw_exponent == 0 {
            let trailing_zeros = raw_mantissa.trailing_zeros();
            mantissa = raw_mantissa >> trailing_zeros;
            exponent = Self::MIN_EXPONENT + i64::wrapping_from(trailing_zeros);
        } else {
            mantissa = raw_mantissa;
            mantissa.set_bit(Self::MANTISSA_WIDTH);
            let trailing_zeros = mantissa.trailing_zeros();
            mantissa >>= trailing_zeros;
            exponent = i64::wrapping_from(raw_exponent)
                + i64::wrapping_from(trailing_zeros)
                + Self::MIN_EXPONENT
                - 1;
        }
        (mantissa, exponent)
    }

    fn from_raw_mantissa_and_exponent(
        raw_mantissa: Self::UnsignedOfEqualWidth,
        raw_exponent: u64,
    ) -> Self {
        assert!(raw_mantissa.significant_bits() <= Self::MANTISSA_WIDTH);
        assert!(raw_exponent.significant_bits() <= Self::EXPONENT_WIDTH);
        Self::from_bits(
            (Self::UnsignedOfEqualWidth::exact_from(raw_exponent) << Self::MANTISSA_WIDTH)
                | raw_mantissa,
        )
    }

    fn from_integer_mantissa_and_exponent(
        integer_mantissa: Self::UnsignedOfEqualWidth,
        integer_exponent: i64,
    ) -> Option<Self> {
        if integer_mantissa == Self::UnsignedOfEqualWidth::ZERO {
            return Some(Self::ZERO);
        }
        let trailing_zeros = integer_mantissa.trailing_zeros();
        let (integer_mantissa, adjusted_exponent) = (
            integer_mantissa >> trailing_zeros,
            integer_exponent + i64::wrapping_from(trailing_zeros),
        );
        let mantissa_bits = integer_mantissa.significant_bits();
        let sci_exponent = adjusted_exponent
            .checked_add(i64::exact_from(mantissa_bits))
            .unwrap()
            - 1;
        let mut raw_mantissa;
        let raw_exponent;
        if sci_exponent < Self::MIN_EXPONENT || sci_exponent > Self::MAX_EXPONENT {
            return None;
        } else if sci_exponent < Self::MIN_NORMAL_EXPONENT {
            if adjusted_exponent < Self::MIN_EXPONENT {
                return None;
            } else {
                raw_exponent = 0;
                raw_mantissa = integer_mantissa << (adjusted_exponent - Self::MIN_EXPONENT);
            }
        } else if mantissa_bits > Self::MANTISSA_WIDTH + 1 {
            return None;
        } else {
            raw_exponent = u64::exact_from(sci_exponent + i64::low_mask(Self::EXPONENT_WIDTH - 1));
            raw_mantissa = integer_mantissa << (Self::MANTISSA_WIDTH + 1 - mantissa_bits);
            raw_mantissa.clear_bit(Self::MANTISSA_WIDTH);
        }
        Some(Self::from_raw_mantissa_and_exponent(
            raw_mantissa,
            raw_exponent,
        ))
    }

    fn max_precision_for_sci_exponent(exponent: i64) -> u64 {
        assert!(exponent >= Self::MIN_EXPONENT);
        assert!(exponent <= Self::MAX_EXPONENT);
        if exponent >= Self::MIN_NORMAL_EXPONENT {
            Self::MANTISSA_WIDTH + 1
        } else {
            u64::wrapping_from(exponent - Self::MIN_EXPONENT) + 1
        }
    }

    fn is_integer(self) -> bool {
        !self.is_nan() && self.is_finite() && self == Self::ZERO
            || self.integer_mantissa_and_exponent().1 >= 0
    }
}

pub mod basic;
/// This module contains `NiceFloat`, a wrapper around primitive float types that provides nicer
/// `Eq`, `Ord`, `Hash`, `Display`, and `FromStr` instances.
pub mod nice_float;

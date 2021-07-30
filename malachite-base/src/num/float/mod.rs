use comparison::traits::{Max, Min};
use named::Named;
use num::arithmetic::traits::{Abs, AbsAssign, NegAssign, Sign, Square, SquareAssign};
use num::basic::traits::{NegativeOne, One, Two, Zero};
use num::conversion::traits::{
    CheckedFrom, CheckedInto, ConvertibleFrom, IntegerMantissaAndExponent, IsInteger,
    RawMantissaAndExponent, RoundingFrom, RoundingInto, SciMantissaAndExponent, WrappingFrom,
};
use num::logic::traits::{BitAccess, LowMask, SignificantBits, TrailingZeros};
use std::cmp::Ordering;
use std::fmt::{self, Debug, Display, Formatter, LowerExp, UpperExp};
use std::hash::{Hash, Hasher};
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
    + Abs<Output = Self>
    + AbsAssign
    + CheckedFrom<u8>
    + CheckedFrom<u16>
    + CheckedFrom<u32>
    + CheckedFrom<u64>
    + CheckedFrom<u128>
    + CheckedFrom<usize>
    + CheckedFrom<i8>
    + CheckedFrom<i16>
    + CheckedFrom<i32>
    + CheckedFrom<i64>
    + CheckedFrom<i128>
    + CheckedFrom<isize>
    + CheckedInto<u8>
    + CheckedInto<u16>
    + CheckedInto<u32>
    + CheckedInto<u64>
    + CheckedInto<u128>
    + CheckedInto<usize>
    + CheckedInto<i8>
    + CheckedInto<i16>
    + CheckedInto<i32>
    + CheckedInto<i64>
    + CheckedInto<i128>
    + CheckedInto<isize>
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
    + Display
    + FmtRyuString
    + From<f32>
    + FromStr
    + IntegerMantissaAndExponent<u64, i64>
    + Into<f64>
    + IsInteger
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
    + RawMantissaAndExponent<u64, u64>
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
    + Square<Output = Self>
    + SquareAssign
    + Sub<Output = Self>
    + SubAssign<Self>
    + Sum<Self>
    + Two
    + UpperExp
    + Zero
{
    /// $M+E+1$
    /// - For `f32`, this is 32.
    /// - For `f64`, this is 64.
    const WIDTH: u64;
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
    const SMALLEST_UNREPRESENTABLE_UINT: u64;
    /// $2^{M+1}(2^E-1)+1$
    /// - For `f32`, this is $2^{32}-2^{24}+1$, or 4278190081.
    /// - For `f64`, this is $2^{64}-2^{53}+1$, or 18437736874454810625.
    const LARGEST_ORDERED_REPRESENTATION: u64;

    fn is_nan(self) -> bool;

    fn is_infinite(self) -> bool;

    fn is_finite(self) -> bool;

    fn is_normal(self) -> bool;

    fn classify(self) -> FpCategory;

    fn to_bits(self) -> u64;

    fn from_bits(v: u64) -> Self;

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
        self.sign() == Ordering::Less && self == Self::ZERO
    }

    /// If `self` is negative zero, returns positive zero; otherwise, returns `self`.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::float::NiceFloat;
    /// use malachite_base::num::float::PrimitiveFloat;
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
    /// use malachite_base::num::float::NiceFloat;
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
    /// use malachite_base::num::float::NiceFloat;
    /// use malachite_base::num::float::PrimitiveFloat;
    ///
    /// assert_eq!(NiceFloat((-0.0f32).next_higher()), NiceFloat(0.0));
    /// assert_eq!(NiceFloat(0.0f32.next_higher()), NiceFloat(1.0e-45));
    /// assert_eq!(NiceFloat(1.0f32.next_higher()), NiceFloat(1.0000001));
    /// assert_eq!(NiceFloat((-1.0f32).next_higher()), NiceFloat(-0.99999994));
    /// ```
    fn next_higher(self) -> Self {
        assert!(!self.is_nan());
        if self.sign() == Ordering::Greater {
            assert_ne!(self, Self::POSITIVE_INFINITY);
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
    /// use malachite_base::num::float::NiceFloat;
    /// use malachite_base::num::float::PrimitiveFloat;
    ///
    /// assert_eq!(NiceFloat(0.0f32.next_lower()), NiceFloat(-0.0));
    /// assert_eq!(NiceFloat((-0.0f32).next_lower()), NiceFloat(-1.0e-45));
    /// assert_eq!(NiceFloat(1.0f32.next_lower()), NiceFloat(0.99999994));
    /// assert_eq!(NiceFloat((-1.0f32).next_lower()), NiceFloat(-1.0000001));
    /// ```
    fn next_lower(self) -> Self {
        assert!(!self.is_nan());
        if self.sign() == Ordering::Less {
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
    fn to_ordered_representation(self) -> u64 {
        assert!(!self.is_nan());
        let bits = self.to_bits();
        if self.sign() == Ordering::Greater {
            (u64::low_mask(Self::EXPONENT_WIDTH) << Self::MANTISSA_WIDTH) + bits + 1
        } else {
            (u64::low_mask(Self::EXPONENT_WIDTH + 1) << Self::MANTISSA_WIDTH) - bits
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
    fn from_ordered_representation(n: u64) -> Self {
        let zero_exp = u64::low_mask(Self::EXPONENT_WIDTH) << Self::MANTISSA_WIDTH;
        let f = if n <= zero_exp {
            Self::from_bits((u64::low_mask(Self::EXPONENT_WIDTH + 1) << Self::MANTISSA_WIDTH) - n)
        } else {
            let f = Self::from_bits(n - zero_exp - 1);
            assert_eq!(f.sign(), Ordering::Greater);
            f
        };
        assert!(!f.is_nan());
        f
    }

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

    fn max_precision_for_sci_exponent(exponent: i64) -> u64 {
        assert!(exponent >= Self::MIN_EXPONENT);
        assert!(exponent <= Self::MAX_EXPONENT);
        if exponent >= Self::MIN_NORMAL_EXPONENT {
            Self::MANTISSA_WIDTH + 1
        } else {
            u64::wrapping_from(exponent - Self::MIN_EXPONENT) + 1
        }
    }

    fn powf(self, exponent: Self) -> Self;
}

pub mod basic;

/// `NiceFloat` is a wrapper around primitive float types that provides nicer `Eq`, `Ord`, `Hash`,
/// `Display`, and `FromStr` instances.
///
/// It's well-known that in most languages, floats behave weirdly due to the IEEE 754 standard. The
/// `NiceFloat` type ignores this standard.
/// - Using `NiceFloat`, `NaN`s are equal to themselves. There is a single, unique `NaN`; there's no
///   concept of signalling `NaN`s. Positive and negative zero are two distinct values, not equal to
///   each other.
/// - The `NiceFloat` hash respects this equality.
/// - Using `NiceFloat`, there is a somewhat-arbitrary total order on floats. These are the classes
///   of floats, in ascending order:
///   - Negative infinity
///   - Negative nonzero finite floats
///   - Negative zero
///   - NaN
///   - Positive zero
///   - Positive nonzero finite floats
///   - Positive floats
/// - `NiceFloat` uses a different `Display` implementation than floats do by default in Rust. For
///   example, Rust will convert `f32::MIN_POSITIVE_SUBNORMAL` to something with many zeros, but
///   `NiceFloat(f32::MIN_POSITIVE_SUBNORMAL)` just converts to `1.0e-45`. The conversion function
///   uses David Tolnay's `ryu` crate, with a few modifications:
///   - All finite floats have a decimal point. For example, Ryu by itself would convert
///     `f32::MIN_POSITIVE_SUBNORMAL` to `1e-45`.
///   - Positive infinity, negative infinity, and NaN are converted to the strings `"Infinity"`,
///     `"-Infinity"`, and "`NaN`", respectively. This is just a personal preference.
/// - `FromStr` accepts these strings.
#[derive(Clone, Copy, Default)]
pub struct NiceFloat<T: PrimitiveFloat>(pub T);

#[derive(Eq, Ord, PartialEq, PartialOrd)]
enum FloatType {
    NegativeInfinity,
    NegativeFinite,
    NegativeZero,
    NaN,
    PositiveZero,
    PositiveFinite,
    PositiveInfinity,
}

impl<T: PrimitiveFloat> NiceFloat<T> {
    fn float_type(self) -> FloatType {
        let f = self.0;
        if f.is_nan() {
            FloatType::NaN
        } else if f.sign() == Ordering::Greater {
            if f == T::ZERO {
                FloatType::PositiveZero
            } else if f.is_finite() {
                FloatType::PositiveFinite
            } else {
                FloatType::PositiveInfinity
            }
        } else if f == T::ZERO {
            FloatType::NegativeZero
        } else if f.is_finite() {
            FloatType::NegativeFinite
        } else {
            FloatType::NegativeInfinity
        }
    }
}

impl<T: PrimitiveFloat> PartialEq<NiceFloat<T>> for NiceFloat<T> {
    /// Compares two `NiceFloat`s for equality.
    ///
    /// This implementation ignores the IEEE 754 standard in favor of a comparison operation that
    /// respects the expected properties of antisymmetry, reflexivity, and transitivity. Using
    /// `NiceFloat`, there is a somewhat-arbitrary total order on floats. These are the classes
    ///   of floats, in ascending order:
    ///   - Negative infinity
    ///   - Negative nonzero finite floats
    ///   - Negative zero
    ///   - NaN
    ///   - Positive zero
    ///   - Positive nonzero finite floats
    ///   - Positive floats
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::float::NiceFloat;
    ///
    /// assert_eq!(NiceFloat(0.0), NiceFloat(0.0));
    /// assert_eq!(NiceFloat(f32::NAN), NiceFloat(f32::NAN));
    /// assert_ne!(NiceFloat(f32::NAN), NiceFloat(0.0));
    /// assert_ne!(NiceFloat(0.0), NiceFloat(-0.0));
    /// assert_eq!(NiceFloat(1.0), NiceFloat(1.0));
    /// ```
    #[inline]
    fn eq(&self, other: &NiceFloat<T>) -> bool {
        let f = self.0;
        let g = other.0;
        f.to_bits() == g.to_bits() || f.is_nan() && g.is_nan()
    }
}

impl<T: PrimitiveFloat> Eq for NiceFloat<T> {}

impl<T: PrimitiveFloat> Hash for NiceFloat<T> {
    /// Computes a hash of a `NiceFloat`.
    ///
    /// The hash is compatible with `NiceFloat` equality: all `NaN`s hash to the same value.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    fn hash<H: Hasher>(&self, state: &mut H) {
        let f = self.0;
        if f.is_nan() {
            "NaN".hash(state)
        } else {
            f.to_bits().hash(state)
        }
    }
}

impl<T: PrimitiveFloat> Ord for NiceFloat<T> {
    /// Compares two `NiceFloat`s.
    ///
    /// This implementation ignores the IEEE 754 standard in favor of an equality operation that
    /// respects the expected properties of symmetry, reflexivity, and transitivity. Using
    /// `NiceFloat`, `NaN`s are equal to themselves. There is a single, unique `NaN`; there's no
    /// concept of signalling `NaN`s. Positive and negative zero are two distinct values, not equal
    /// to each other.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::float::NiceFloat;
    /// use malachite_base::num::float::PrimitiveFloat;
    ///
    /// assert!(NiceFloat(0.0) > NiceFloat(-0.0));
    /// assert!(NiceFloat(f32::NAN) < NiceFloat(0.0));
    /// assert!(NiceFloat(f32::NAN) > NiceFloat(-0.0));
    /// assert!(NiceFloat(f32::POSITIVE_INFINITY) > NiceFloat(f32::NAN));
    /// assert!(NiceFloat(f32::NAN) < NiceFloat(1.0));
    /// ```
    fn cmp(&self, other: &NiceFloat<T>) -> Ordering {
        let self_type = self.float_type();
        let other_type = other.float_type();
        self_type.cmp(&other_type).then_with(|| {
            if self_type == FloatType::PositiveFinite || self_type == FloatType::NegativeFinite {
                self.0.partial_cmp(&other.0).unwrap()
            } else {
                Ordering::Equal
            }
        })
    }
}

impl<T: PrimitiveFloat> PartialOrd<NiceFloat<T>> for NiceFloat<T> {
    /// Compares a `NiceFloat` to another `NiceFloat`.
    ///
    /// See the documentation for the `Ord` implementation.
    #[inline]
    fn partial_cmp(&self, other: &NiceFloat<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[doc(hidden)]
pub trait FmtRyuString: Copy {
    fn fmt_ryu_string(self, f: &mut Formatter<'_>) -> fmt::Result;
}

macro_rules! impl_fmt_ryu_string {
    ($f: ident) => {
        impl FmtRyuString for $f {
            #[inline]
            fn fmt_ryu_string(self, f: &mut Formatter<'_>) -> fmt::Result {
                let mut buffer = ryu::Buffer::new();
                let printed = buffer.format_finite(self);
                // Convert e.g. "1e100" to "1.0e100".
                // `printed` is ASCII, so we can manipulate bytes rather than chars.
                let mut e_index = None;
                let mut found_dot = false;
                for (i, &b) in printed.as_bytes().iter().enumerate() {
                    match b {
                        b'.' => {
                            found_dot = true;
                            break; // If there's a '.', we don't need to do anything
                        }
                        b'e' => {
                            e_index = Some(i);
                            break; // OK to break since there won't be a '.' after an 'e'
                        }
                        _ => {}
                    }
                }
                if !found_dot {
                    if let Some(e_index) = e_index {
                        let mut out_bytes = vec![0; printed.len() + 2];
                        let (in_bytes_lo, in_bytes_hi) = printed.as_bytes().split_at(e_index);
                        let (out_bytes_lo, out_bytes_hi) = out_bytes.split_at_mut(e_index);
                        out_bytes_lo.copy_from_slice(in_bytes_lo);
                        out_bytes_hi[0] = b'.';
                        out_bytes_hi[1] = b'0';
                        out_bytes_hi[2..].copy_from_slice(in_bytes_hi);
                        f.write_str(std::str::from_utf8(&out_bytes).unwrap())
                    } else {
                        panic!("Unexpected Ryu string: {}", printed);
                    }
                } else {
                    f.write_str(printed)
                }
            }
        }
    };
}
impl_fmt_ryu_string!(f32);
impl_fmt_ryu_string!(f64);

impl<T: PrimitiveFloat> Display for NiceFloat<T> {
    /// Converts a `NiceFloat` to a `String`.
    ///
    /// `NiceFloat` uses a different `Display` implementation than floats do by default in Rust. For
    /// example, Rust will convert `f32::MIN_POSITIVE_SUBNORMAL` to something with many zeros, but
    /// `NiceFloat(f32::MIN_POSITIVE_SUBNORMAL)` just converts to `1.0e-45`. The conversion function
    /// uses David Tolnay's `ryu` crate, with a few modifications:
    /// - All finite floats have a decimal point. For example, Ryu by itself would convert
    ///   `f32::MIN_POSITIVE_SUBNORMAL` to `1e-45`.
    /// - Positive infinity, negative infinity, and NaN are converted to the strings `"Infinity"`,
    ///   `"-Infinity"`, and "`NaN`", respectively. This is just a personal preference.
    ///
    /// # Worst-case complexity
    /// Constant time and additional memory.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::float::NiceFloat;
    /// use malachite_base::num::float::PrimitiveFloat;
    ///
    /// assert_eq!(NiceFloat(0.0).to_string(), "0.0");
    /// assert_eq!(NiceFloat(-0.0).to_string(), "-0.0");
    /// assert_eq!(NiceFloat(f32::POSITIVE_INFINITY).to_string(), "Infinity");
    /// assert_eq!(NiceFloat(f32::NEGATIVE_INFINITY).to_string(), "-Infinity");
    /// assert_eq!(NiceFloat(f32::NAN).to_string(), "NaN");
    ///
    /// assert_eq!(NiceFloat(1.0).to_string(), "1.0");
    /// assert_eq!(NiceFloat(-1.0).to_string(), "-1.0");
    /// assert_eq!(NiceFloat(f32::MIN_POSITIVE_SUBNORMAL).to_string(), "1.0e-45");
    /// assert_eq!(NiceFloat(std::f64::consts::E).to_string(), "2.718281828459045");
    /// assert_eq!(NiceFloat(std::f64::consts::PI).to_string(), "3.141592653589793");
    /// ```
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.0.is_nan() {
            f.write_str("NaN")
        } else if self.0.is_infinite() {
            if self.0.sign() == Ordering::Greater {
                f.write_str("Infinity")
            } else {
                f.write_str("-Infinity")
            }
        } else {
            self.0.fmt_ryu_string(f)
        }
    }
}

impl<T: PrimitiveFloat> Debug for NiceFloat<T> {
    /// Converts a `NiceFloat` to a `String`.
    ///
    /// This is identical to the `Display::fmt` implementation.
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        Display::fmt(self, f)
    }
}

impl<T: PrimitiveFloat> FromStr for NiceFloat<T> {
    type Err = <T as FromStr>::Err;

    /// Converts a `&str` to a `NiceFloat`.
    ///
    /// If the `&str` does not represent a valid `NiceFloat`, an `Err` is returned.
    ///
    /// # Worst-case complexity
    /// $T(n) = O(n)$
    ///
    /// $M(n) = O(1)$
    ///
    /// where $T$ is time, $M$ is additional memory, and $n$ = `src.len()`.
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::float::NiceFloat;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(NiceFloat::from_str("NaN").unwrap(), NiceFloat(f32::NAN));
    /// assert_eq!(NiceFloat::from_str("-0.00").unwrap(), NiceFloat(-0.0f64));
    /// assert_eq!(NiceFloat::from_str(".123").unwrap(), NiceFloat(0.123f32));
    /// ```
    #[inline]
    fn from_str(src: &str) -> Result<NiceFloat<T>, <T as FromStr>::Err> {
        match src {
            "NaN" => Ok(T::NAN),
            "Infinity" => Ok(T::POSITIVE_INFINITY),
            "-Infinity" => Ok(T::NEGATIVE_INFINITY),
            src => T::from_str(src),
        }
        .map(NiceFloat)
    }
}

use std::fmt::{Debug, Display, LowerExp, UpperExp};
use std::iter::{Product, Sum};
use std::num::FpCategory;
use std::ops::{
    Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign,
};
use std::str::FromStr;

use comparison::traits::{Max, Min};
use named::Named;
use num::arithmetic::traits::{ModPowerOfTwo, ModPowerOfTwoNeg, NegAssign, PowerOfTwo};
use num::basic::integers::PrimitiveInt;
use num::basic::signeds::PrimitiveSigned;
use num::basic::traits::{NegativeOne, One, Two, Zero};
use num::basic::unsigneds::PrimitiveUnsigned;
use num::conversion::traits::{ExactFrom, ExactInto};

//TODO docs
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
    + FromStr
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
    type SignedOfEqualWidth: PrimitiveSigned;

    const WIDTH: u64 = Self::UnsignedOfEqualWidth::WIDTH;
    const EXPONENT_WIDTH: u64 = Self::WIDTH - Self::MANTISSA_WIDTH - 1;
    const MANTISSA_WIDTH: u64;
    const MIN_NORMAL_EXPONENT: i64 = -((1 << (Self::EXPONENT_WIDTH - 1)) - 2);
    const MIN_EXPONENT: i64 = Self::MIN_NORMAL_EXPONENT - (Self::MANTISSA_WIDTH as i64);
    const MAX_EXPONENT: u64 = (1 << (Self::EXPONENT_WIDTH - 1)) - 1;
    const MIN_POSITIVE_SUBNORMAL: Self;
    const MAX_SUBNORMAL: Self;
    const MIN_POSITIVE_NORMAL: Self;
    const POSITIVE_INFINITY: Self;
    const NEGATIVE_INFINITY: Self;
    const NEGATIVE_ZERO: Self;
    const NAN: Self;
    const MAX_FINITE: Self;
    const MIN_FINITE: Self;
    const SMALLEST_UNREPRESENTABLE_UINT: Self::UnsignedOfEqualWidth;

    fn is_nan(self) -> bool;

    fn is_infinite(self) -> bool;

    fn is_finite(self) -> bool;

    fn is_normal(self) -> bool;

    fn classify(self) -> FpCategory;

    fn is_sign_positive(self) -> bool;

    fn is_sign_negative(self) -> bool;

    fn to_bits(self) -> Self::UnsignedOfEqualWidth;

    fn from_bits(v: Self::UnsignedOfEqualWidth) -> Self;

    fn abs_negative_zeros(self) -> Self;

    fn abs_assign_negative_zeros(&mut self);

    fn to_ordered_representation(self) -> Self::UnsignedOfEqualWidth;

    fn from_ordered_representation(n: Self::UnsignedOfEqualWidth) -> Self;

    fn to_adjusted_mantissa_and_exponent(self) -> (Self::UnsignedOfEqualWidth, u64) {
        let bits = self.to_bits();
        let mantissa = bits.mod_power_of_two(Self::MANTISSA_WIDTH);
        let exponent: u64 = (bits >> Self::MANTISSA_WIDTH).exact_into();
        let exponent = exponent.mod_power_of_two(Self::EXPONENT_WIDTH);
        (mantissa, exponent)
    }

    fn adjusted_exponent(self) -> u64 {
        let bits = self.to_bits();
        let exponent: u64 = (bits >> Self::MANTISSA_WIDTH).exact_into();
        exponent.mod_power_of_two(Self::EXPONENT_WIDTH)
    }

    fn from_adjusted_mantissa_and_exponent(
        mantissa: Self::UnsignedOfEqualWidth,
        exponent: u64,
    ) -> Self {
        Self::from_bits(
            (Self::UnsignedOfEqualWidth::exact_from(exponent) << Self::MANTISSA_WIDTH) + mantissa,
        )
    }

    fn exponent(self) -> i64 {
        i64::exact_from(self.adjusted_exponent()) - i64::exact_from(Self::MAX_EXPONENT)
    }
}

//TODO docs
macro_rules! float_traits {
    (
        $t: ident,
        $u: ident,
        $min_positive_subnormal: expr,
        $max_subnormal: expr,
        $min_positive_normal: expr
    ) => {
        //TODO docs
        impl PrimitiveFloat for $t {
            type UnsignedOfEqualWidth = $u;
            type SignedOfEqualWidth = <$u as PrimitiveUnsigned>::SignedOfEqualWidth;
            const MANTISSA_WIDTH: u64 = (std::$t::MANTISSA_DIGITS as u64) - 1;

            const POSITIVE_INFINITY: Self = std::$t::INFINITY;
            const NEGATIVE_INFINITY: Self = std::$t::NEG_INFINITY;
            const NEGATIVE_ZERO: Self = -0.0;
            const NAN: Self = std::$t::NAN;
            const MAX_FINITE: Self = std::$t::MAX;
            const MIN_FINITE: Self = std::$t::MIN;
            const MIN_POSITIVE_SUBNORMAL: Self = $min_positive_subnormal;
            const MAX_SUBNORMAL: Self = $max_subnormal;
            const MIN_POSITIVE_NORMAL: Self = $min_positive_normal;
            const SMALLEST_UNREPRESENTABLE_UINT: $u = (1 << (Self::MANTISSA_WIDTH + 1)) + 1;

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
            fn classify(self) -> FpCategory {
                $t::classify(self)
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
            fn to_bits(self) -> $u {
                $t::to_bits(self)
            }

            #[inline]
            fn from_bits(v: $u) -> $t {
                $t::from_bits(v)
            }

            fn abs_negative_zeros(self) -> $t {
                if self == 0.0 {
                    0.0
                } else {
                    self
                }
            }

            fn abs_assign_negative_zeros(&mut self) {
                if *self == 0.0 {
                    *self = 0.0;
                }
            }

            fn to_ordered_representation(self) -> $u {
                if self.is_nan() {
                    panic!("float cannot be NaN.");
                }
                if self >= 0.0 {
                    $u::power_of_two($u::WIDTH - 1) + self.abs_negative_zeros().to_bits()
                } else {
                    (-self).to_bits().mod_power_of_two_neg($u::WIDTH - 1)
                }
            }

            fn from_ordered_representation(n: $u) -> $t {
                let f = if n.get_highest_bit() {
                    $t::from_bits(n - $u::power_of_two($u::WIDTH - 1))
                } else {
                    -$t::from_bits(n.mod_power_of_two_neg($u::WIDTH - 1))
                };
                if f.is_nan() {
                    panic!("invalid ordered representation");
                }
                f
            }
        }

        impl_named!($t);

        impl Min for $t {
            const MIN: $t = $t::NEGATIVE_INFINITY;
        }

        impl Max for $t {
            const MAX: $t = $t::POSITIVE_INFINITY;
        }

        impl NegAssign for $t {
            #[inline]
            fn neg_assign(&mut self) {
                *self = -*self;
            }
        }
    };
}

float_traits!(f32, u32, 1.0e-45, 1.175_494_2e-38, 1.175_494_4e-38);
float_traits!(
    f64,
    u64,
    5.0e-324,
    2.225_073_858_507_201e-308,
    2.225_073_858_507_201_4e-308
);

pub fn increment_float<T: PrimitiveFloat>(f: &mut T) {
    f.abs_assign_negative_zeros();
    if *f == T::POSITIVE_INFINITY {
        panic!("Can't increment positive infinity");
    }
    *f = T::from_ordered_representation(
        f.to_ordered_representation() + T::UnsignedOfEqualWidth::ONE,
    );
}

pub fn decrement_float<T: PrimitiveFloat>(f: &mut T) {
    f.abs_assign_negative_zeros();
    if *f == T::NEGATIVE_INFINITY {
        panic!("Can't decrement positive infinity");
    }
    *f = T::from_ordered_representation(
        f.to_ordered_representation() - T::UnsignedOfEqualWidth::ONE,
    );
}

/// Implements the constants 0, 1, 2, and -1 for primitive floating-point types.
macro_rules! impl01_float {
    ($t:ty) => {
        /// The constant 0.0 for primitive floating-point types.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl Zero for $t {
            const ZERO: $t = 0.0;
        }

        /// The constant 1.0 for primitive floating-point types.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl One for $t {
            const ONE: $t = 1.0;
        }

        /// The constant 2.0 for primitive floating-point types.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl Two for $t {
            const TWO: $t = 2.0;
        }

        /// The constant -1.0 for primitive floating-point types.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        impl NegativeOne for $t {
            const NEGATIVE_ONE: $t = -1.0;
        }
    };
}

impl01_float!(f32);
impl01_float!(f64);

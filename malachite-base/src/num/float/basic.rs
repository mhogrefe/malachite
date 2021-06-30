use comparison::traits::{Max, Min};
use named::Named;
use num::arithmetic::traits::NegAssign;
use num::basic::traits::{NegativeOne, One, Two, Zero};
use num::conversion::traits::{ExactInto, WrappingFrom};
use num::float::PrimitiveFloat;
use std::num::FpCategory;

//TODO docs
macro_rules! float_traits {
    (
        $t: ident,
        $width: expr,
        $min_positive_subnormal: expr,
        $max_subnormal: expr,
        $min_positive_normal: expr
    ) => {
        impl PrimitiveFloat for $t {
            const WIDTH: u64 = $width;
            const MANTISSA_WIDTH: u64 = (std::$t::MANTISSA_DIGITS as u64) - 1;

            const POSITIVE_INFINITY: Self = std::$t::INFINITY;
            const NEGATIVE_INFINITY: Self = std::$t::NEG_INFINITY;
            const NEGATIVE_ZERO: Self = -0.0;
            const NAN: Self = std::$t::NAN;
            const MAX_FINITE: Self = std::$t::MAX;
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

            #[inline]
            fn floor(self) -> Self {
                $t::floor(self)
            }

            #[inline]
            fn ceil(self) -> Self {
                $t::ceil(self)
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
float_traits!(f32, 32, 1.0e-45, 1.1754942e-38, 1.1754944e-38);
float_traits!(
    f64,
    64,
    5.0e-324,
    2.225073858507201e-308,
    2.2250738585072014e-308
);

/// Implements the constants 0, 1, 2, and -1 for primitive floating-point types.
macro_rules! impl01float {
    ($t:ty) => {
        /// The constant 0.0 for primitive floating-point types.
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        impl Zero for $t {
            const ZERO: $t = 0.0;
        }

        /// The constant 1.0 for primitive floating-point types.
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        impl One for $t {
            const ONE: $t = 1.0;
        }

        /// The constant 2.0 for primitive floating-point types.
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        impl Two for $t {
            const TWO: $t = 2.0;
        }

        /// The constant -1.0 for primitive floating-point types.
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        impl NegativeOne for $t {
            const NEGATIVE_ONE: $t = -1.0;
        }
    };
}
impl01float!(f32);
impl01float!(f64);

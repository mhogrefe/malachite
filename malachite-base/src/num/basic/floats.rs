use comparison::traits::{Max, Min};
use named::Named;
use num::basic::traits::{NegativeOne, One, Two, Zero};
use num::float::PrimitiveFloat;

/// This macro defines basic trait implementations for floating-point types.
macro_rules! impl_basic_traits_primitive_float {
    ($t:ident) => {
        impl_named!($t);

        /// The constant 0.
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        impl Zero for $t {
            const ZERO: $t = 0.0;
        }

        /// The constant 1.
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        impl One for $t {
            const ONE: $t = 1.0;
        }

        /// The constant 2.
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

        /// The lowest value representable by this type, negative infinity.
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        impl Min for $t {
            const MIN: $t = $t::NEGATIVE_INFINITY;
        }

        /// The highest value representable by this type, negative infinity.
        ///
        /// # Worst-case complexity
        /// Constant time and additional memory.
        impl Max for $t {
            const MAX: $t = $t::POSITIVE_INFINITY;
        }
    };
}
impl_basic_traits_primitive_float!(f32);
impl_basic_traits_primitive_float!(f64);

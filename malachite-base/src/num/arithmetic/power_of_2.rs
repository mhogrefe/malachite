use num::arithmetic::traits::PowerOf2;
use num::basic::integers::PrimitiveInt;
use num::conversion::traits::IntegerMantissaAndExponent;

fn power_of_2_unsigned<T: PrimitiveInt>(pow: u64) -> T {
    assert!(pow < T::WIDTH);
    T::ONE << pow
}

macro_rules! impl_power_of_2_unsigned {
    ($t:ident) => {
        impl PowerOf2<u64> for $t {
            /// Computes $2^p$.
            ///
            /// $f(p) = 2^p$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the result would be too large to be represented.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::power_of_2` module.
            #[inline]
            fn power_of_2(pow: u64) -> $t {
                power_of_2_unsigned(pow)
            }
        }
    };
}
apply_to_unsigneds!(impl_power_of_2_unsigned);

fn power_of_2_signed<T: PrimitiveInt>(pow: u64) -> T {
    assert!(pow < T::WIDTH - 1);
    T::ONE << pow
}

macro_rules! impl_power_of_2_signed {
    ($t:ident) => {
        impl PowerOf2<u64> for $t {
            /// Computes $2^p$.
            ///
            /// $f(p) = 2^p$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the result would be too large to be represented.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::power_of_2` module.
            #[inline]
            fn power_of_2(pow: u64) -> $t {
                power_of_2_signed(pow)
            }
        }
    };
}
apply_to_signeds!(impl_power_of_2_signed);

macro_rules! impl_power_of_2_primitive_float {
    ($t:ident) => {
        impl PowerOf2<i64> for $t {
            /// Computes $2^p$.
            ///
            /// $f(p) = 2^p$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the power is smaller than `Self::MIN_EXPONENT` or greater than
            /// `Self::MAX_EXPONENT`.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::power_of_2` module.
            #[inline]
            fn power_of_2(pow: i64) -> $t {
                $t::from_integer_mantissa_and_exponent(1, pow).unwrap()
            }
        }
    };
}
apply_to_primitive_floats!(impl_power_of_2_primitive_float);

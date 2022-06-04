use num::arithmetic::traits::IsPowerOf2;
use num::conversion::traits::IntegerMantissaAndExponent;

macro_rules! impl_is_power_of_2_unsigned {
    ($t:ident) => {
        impl IsPowerOf2 for $t {
            /// This is a wrapper over the `is_power_of_two` functions in the standard library, for
            /// example [this one](u32::is_power_of_two).
            #[inline]
            fn is_power_of_2(&self) -> bool {
                $t::is_power_of_two(*self)
            }
        }
    };
}
apply_to_unsigneds!(impl_is_power_of_2_unsigned);

macro_rules! impl_is_power_of_2_primitive_float {
    ($t:ident) => {
        impl IsPowerOf2 for $t {
            /// Determines whether a number is an integer power of 2.
            ///
            /// $f(x) = (\exists n \in \Z : 2^n = x)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::is_power_of_2#is_power_of_2).
            #[inline]
            fn is_power_of_2(&self) -> bool {
                self.is_finite() && *self > 0.0 && self.integer_mantissa() == 1
            }
        }
    };
}
apply_to_primitive_floats!(impl_is_power_of_2_primitive_float);

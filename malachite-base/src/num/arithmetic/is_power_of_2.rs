use num::arithmetic::traits::IsPowerOf2;
use num::conversion::traits::IntegerMantissaAndExponent;

macro_rules! impl_is_power_of_2_unsigned {
    ($t:ident) => {
        impl IsPowerOf2 for $t {
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
            /// Determines whether `self` is an integer power of 2.
            ///
            /// $f(x) = (\exists n \in \Z : 2^n = x)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::is_power_of_2` module.
            #[inline]
            fn is_power_of_2(&self) -> bool {
                self.is_finite() && *self > 0.0 && self.integer_mantissa() == 1
            }
        }
    };
}
apply_to_primitive_floats!(impl_is_power_of_2_primitive_float);

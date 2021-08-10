use num::arithmetic::traits::{Pow, PowAssign};
use num::conversion::traits::ExactFrom;

macro_rules! impl_pow_primitive_int {
    ($t:ident) => {
        impl Pow<u64> for $t {
            type Output = $t;

            #[inline]
            fn pow(self, exp: u64) -> $t {
                $t::pow(self, u32::exact_from(exp))
            }
        }

        impl PowAssign<u64> for $t {
            /// Replaces `self` with `self` raised to the power of `exp`.
            ///
            /// $x \gets x^p$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::pow` module.
            #[inline]
            fn pow_assign(&mut self, exp: u64) {
                *self = $t::pow(*self, u32::exact_from(exp));
            }
        }
    };
}
apply_to_primitive_ints!(impl_pow_primitive_int);

macro_rules! impl_pow_primitive_float {
    ($t:ident) => {
        impl Pow<i64> for $t {
            type Output = $t;

            #[inline]
            fn pow(self, exp: i64) -> $t {
                self.powi(i32::exact_from(exp))
            }
        }

        impl PowAssign<i64> for $t {
            /// Replaces `self` with `self` raised to the power of `exp`.
            ///
            /// $x \gets x^p$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::pow` module.
            #[inline]
            fn pow_assign(&mut self, exp: i64) {
                *self = self.powi(i32::exact_from(exp));
            }
        }

        impl Pow<$t> for $t {
            type Output = $t;

            #[inline]
            fn pow(self, exp: $t) -> $t {
                self.powf(exp)
            }
        }

        impl PowAssign<$t> for $t {
            /// Replaces `self` with `self` raised to the power of `exp`.
            ///
            /// $x \gets x^p$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::pow` module.
            #[inline]
            fn pow_assign(&mut self, exp: $t) {
                *self = self.powf(exp);
            }
        }
    };
}
apply_to_primitive_floats!(impl_pow_primitive_float);

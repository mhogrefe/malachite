use num::arithmetic::traits::{DivisibleByPowerOf2, ModPowerOf2};
use num::conversion::traits::WrappingFrom;

macro_rules! impl_divisible_by_power_of_2_unsigned {
    ($t:ident) => {
        impl DivisibleByPowerOf2 for $t {
            /// Returns whether `self` is divisible by $2^p$.
            ///
            /// $f(x, p) = (2^p|x)$.
            ///
            /// $f(x, p) = (\exists k \in \N \ x = k2^p)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::divisible_by_power_of_2` module.
            #[inline]
            fn divisible_by_power_of_2(self, pow: u64) -> bool {
                self.mod_power_of_2(pow) == 0
            }
        }
    };
}
apply_to_unsigneds!(impl_divisible_by_power_of_2_unsigned);

macro_rules! impl_divisible_by_power_of_2_signed {
    ($u:ident, $s:ident) => {
        impl DivisibleByPowerOf2 for $s {
            /// Returns whether `self` is divisible by $2^p$.
            ///
            /// $f(x, p) = (2^p|x)$.
            ///
            /// $f(x, p) = (\exists k \in \N \ x = k2^p)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::divisible_by_power_of_2` module.
            #[inline]
            fn divisible_by_power_of_2(self, pow: u64) -> bool {
                $u::wrapping_from(self).divisible_by_power_of_2(pow)
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_divisible_by_power_of_2_signed);

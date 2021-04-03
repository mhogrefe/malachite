use num::arithmetic::traits::{DivisibleByPowerOfTwo, ModPowerOfTwo};
use num::conversion::traits::WrappingFrom;

macro_rules! impl_divisible_by_power_of_two_unsigned {
    ($t:ident) => {
        impl DivisibleByPowerOfTwo for $t {
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
            /// See the documentation of the `num::arithmetic::divisible_by_power_of_two` module.
            #[inline]
            fn divisible_by_power_of_two(self, pow: u64) -> bool {
                self.mod_power_of_two(pow) == 0
            }
        }
    };
}
apply_to_unsigneds!(impl_divisible_by_power_of_two_unsigned);

macro_rules! impl_divisible_by_power_of_two_signed {
    ($u:ident, $s:ident) => {
        impl DivisibleByPowerOfTwo for $s {
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
            /// See the documentation of the `num::arithmetic::divisible_by_power_of_two` module.
            #[inline]
            fn divisible_by_power_of_two(self, pow: u64) -> bool {
                $u::wrapping_from(self).divisible_by_power_of_two(pow)
            }
        }
    };
}
apply_to_unsigned_signed_pair!(impl_divisible_by_power_of_two_signed);

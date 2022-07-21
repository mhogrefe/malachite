use crate::num::arithmetic::traits::{DivisibleByPowerOf2, EqModPowerOf2};

macro_rules! impl_eq_mod_power_of_2 {
    ($t:ident) => {
        impl EqModPowerOf2<$t> for $t {
            /// Returns whether one number is equal to another modulo $2^k$.
            ///
            /// $f(x, y, k) = (x \equiv y \mod 2^k)$.
            ///
            /// $f(x, y, k) = (\exists n \in \Z : x - y = n2^k)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::eq_mod_power_of_2#eq_mod_power_of_2).
            #[inline]
            fn eq_mod_power_of_2(self, other: $t, pow: u64) -> bool {
                (self ^ other).divisible_by_power_of_2(pow)
            }
        }
    };
}
apply_to_primitive_ints!(impl_eq_mod_power_of_2);

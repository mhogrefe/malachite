use num::arithmetic::traits::{DivisibleByPowerOfTwo, EqModPowerOfTwo};

macro_rules! impl_eq_mod_power_of_two {
    ($t:ident) => {
        impl EqModPowerOfTwo<$t> for $t {
            /// Returns whether `self` is equal to `other` mod $2^p$.
            ///
            /// $f(x, y, p) = (x \equiv y \mod 2^p)$.
            ///
            /// $f(x, y, p) = (\exists k \in \Z \ x - y = k2^p)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::eq_mod_power_of_two` module.
            #[inline]
            fn eq_mod_power_of_two(self, other: $t, pow: u64) -> bool {
                (self ^ other).divisible_by_power_of_two(pow)
            }
        }
    };
}
apply_to_primitive_ints!(impl_eq_mod_power_of_two);

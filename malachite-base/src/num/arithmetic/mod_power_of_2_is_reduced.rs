use num::arithmetic::traits::ModPowerOf2IsReduced;
use num::logic::traits::SignificantBits;

macro_rules! impl_mod_power_of_2_is_reduced {
    ($t:ident) => {
        impl ModPowerOf2IsReduced for $t {
            /// Returns whether `self` is reduced mod $2^p$; in other words, whether it has no more
            /// than `pow` significant bits.
            ///
            /// $f(x, p) = (x < 2^p)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_power_of_2_is_reduced` module.
            #[inline]
            fn mod_power_of_2_is_reduced(&self, pow: u64) -> bool {
                self.significant_bits() <= pow
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_power_of_2_is_reduced);

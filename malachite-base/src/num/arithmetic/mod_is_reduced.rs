use num::arithmetic::traits::ModIsReduced;

macro_rules! impl_mod_is_reduced {
    ($t:ident) => {
        impl ModIsReduced for $t {
            /// Returns whether `self` is reduced mod `m`; in other words, whether it is less than
            /// `m`. `m` cannot be zero.
            ///
            /// $f(x, m) = (x < m)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `m` is 0.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_is_reduced` module.
            #[inline]
            fn mod_is_reduced(&self, m: &$t) -> bool {
                assert_ne!(*m, 0);
                self < m
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_is_reduced);

use crate::num::arithmetic::traits::ModIsReduced;

macro_rules! impl_mod_is_reduced {
    ($t:ident) => {
        impl ModIsReduced for $t {
            /// Returns whether a number is reduced modulo another number $m$; in other words,
            /// whether it is less than $m$. $m$ cannot be zero.
            ///
            /// $f(x, m) = (x < m)$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if $m$ is 0.
            ///
            /// # Examples
            /// See [here](super::mod_is_reduced#mod_is_reduced).
            #[inline]
            fn mod_is_reduced(&self, m: &$t) -> bool {
                assert_ne!(*m, 0);
                self < m
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_is_reduced);

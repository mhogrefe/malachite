use num::arithmetic::traits::ModIsReduced;

macro_rules! impl_mod_is_reduced {
    ($t:ident) => {
        impl ModIsReduced for $t {
            /// Returns whether `self` is reduced mod `m`; in other words whether it is less than
            /// `m`. `m` cannot be zero.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if `m` is 0.
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModIsReduced;
            ///
            /// assert_eq!(0u8.mod_is_reduced(&5), true);
            /// assert_eq!(100u64.mod_is_reduced(&100), false);
            /// assert_eq!(100u16.mod_is_reduced(&101), true);
            /// ```
            #[inline]
            fn mod_is_reduced(&self, m: &$t) -> bool {
                assert_ne!(*m, 0);
                self < m
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_is_reduced);

use num::logic::traits::NotAssign;

macro_rules! impl_not {
    ($t:ident) => {
        impl NotAssign for $t {
            /// Replace a number with its bitwise negation.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::NotAssign;
            ///
            /// let mut x = 123u16;
            /// x.not_assign();
            /// assert_eq!(x, 65_412);
            /// ```
            #[inline]
            fn not_assign(&mut self) {
                *self = !*self;
            }
        }
    };
}
apply_to_primitive_ints!(impl_not);

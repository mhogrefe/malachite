use num::arithmetic::traits::CheckedSquare;

macro_rules! impl_checked_square {
    ($t:ident) => {
        impl CheckedSquare for $t {
            type Output = $t;

            /// Squares `self`, returning `None` if there is no valid result.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::CheckedSquare;
            ///
            /// assert_eq!(3u8.checked_square(), Some(9));
            /// assert_eq!((-1000i32).checked_square(), Some(1000000));
            /// assert_eq!((1000u16).checked_square(), None);
            /// ```
            #[inline]
            fn checked_square(self) -> Option<$t> {
                self.checked_mul(self)
            }
        }
    };
}
apply_to_primitive_ints!(impl_checked_square);

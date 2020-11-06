use num::arithmetic::traits::{Square, SquareAssign};

macro_rules! impl_square {
    ($t:ident) => {
        impl Square for $t {
            type Output = $t;

            /// Squares `self`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::Square;
            ///
            /// assert_eq!(3u8.square(), 9);
            /// assert_eq!((-1000i32).square(), 1000000);
            /// ```
            #[inline]
            fn square(self) -> $t {
                self * self
            }
        }

        impl SquareAssign for $t {
            /// Replaces `self` with `self ^ 2`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::SquareAssign;
            ///
            /// let mut x = 3u8;
            /// x.square_assign();
            /// assert_eq!(x, 9);
            ///
            /// let mut x = -1000i32;
            /// x.square_assign();
            /// assert_eq!(x, 1000000);
            /// ```
            #[inline]
            fn square_assign(&mut self) {
                *self *= *self;
            }
        }
    };
}
apply_to_primitive_ints!(impl_square);

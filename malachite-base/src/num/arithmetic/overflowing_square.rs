use num::arithmetic::traits::{OverflowingMulAssign, OverflowingSquare, OverflowingSquareAssign};

macro_rules! impl_overflowing_square {
    ($t:ident) => {
        impl OverflowingSquare for $t {
            type Output = $t;

            /// Squares `self`, overflowing around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::OverflowingSquare;
            ///
            /// assert_eq!(3u8.overflowing_square(), (9, false));
            /// assert_eq!((-1000i32).overflowing_square(), (1000000, false));
            /// assert_eq!(1000u16.overflowing_square(), (16960, true));
            /// ```
            #[inline]
            fn overflowing_square(self) -> ($t, bool) {
                self.overflowing_mul(self)
            }
        }

        impl OverflowingSquareAssign for $t {
            /// Replaces `self` with `self ^ 2`, overflowing around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::OverflowingSquareAssign;
            ///
            /// let mut x = 3u8;
            /// assert_eq!(x.overflowing_square_assign(), false);
            /// assert_eq!(x, 9);
            ///
            /// let mut x = -1000i32;
            /// assert_eq!(x.overflowing_square_assign(), false);
            /// assert_eq!(x, 1000000);
            ///
            /// let mut x = 1000u16;
            /// assert_eq!(x.overflowing_square_assign(), true);
            /// assert_eq!(x, 16960);
            /// ```
            #[inline]
            fn overflowing_square_assign(&mut self) -> bool {
                self.overflowing_mul_assign(*self)
            }
        }
    };
}
apply_to_primitive_ints!(impl_overflowing_square);

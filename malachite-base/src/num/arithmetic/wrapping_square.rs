use num::arithmetic::traits::{WrappingMulAssign, WrappingSquare, WrappingSquareAssign};

macro_rules! impl_wrapping_square {
    ($t:ident) => {
        impl WrappingSquare for $t {
            type Output = $t;

            /// Squares `self`, wrapping around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingSquare;
            ///
            /// assert_eq!(3u8.wrapping_square(), 9);
            /// assert_eq!((-1_000i32).wrapping_square(), 1_000_000);
            /// assert_eq!(1_000u16.wrapping_square(), 16_960);
            /// ```
            #[inline]
            fn wrapping_square(self) -> $t {
                self.wrapping_mul(self)
            }
        }

        impl WrappingSquareAssign for $t {
            /// Replaces `self` with `self ^ 2`, wrapping around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingSquareAssign;
            ///
            /// let mut x = 3u8;
            /// x.wrapping_square_assign();
            /// assert_eq!(x, 9);
            ///
            /// let mut x = -1_000i32;
            /// x.wrapping_square_assign();
            /// assert_eq!(x, 1_000_000);
            ///
            /// let mut x = 1_000u16;
            /// x.wrapping_square_assign();
            /// assert_eq!(x, 16_960);
            /// ```
            #[inline]
            fn wrapping_square_assign(&mut self) {
                self.wrapping_mul_assign(*self);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_square);

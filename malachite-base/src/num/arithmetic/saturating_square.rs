use num::arithmetic::traits::{SaturatingMulAssign, SaturatingSquare, SaturatingSquareAssign};

macro_rules! impl_saturating_square {
    ($t:ident) => {
        impl SaturatingSquare for $t {
            type Output = $t;

            /// Squares `self`, saturating at the numeric bounds instead of overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingSquare;
            ///
            /// assert_eq!(3u8.saturating_square(), 9);
            /// assert_eq!((-1000i32).saturating_square(), 1000000);
            /// assert_eq!(1000u16.saturating_square(), u16::MAX);
            /// ```
            #[inline]
            fn saturating_square(self) -> $t {
                self.saturating_mul(self)
            }
        }

        impl SaturatingSquareAssign for $t {
            /// Replaces `self` with `self ^ 2`, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingSquareAssign;
            ///
            /// let mut x = 3u8;
            /// x.saturating_square_assign();
            /// assert_eq!(x, 9);
            ///
            /// let mut x = -1000i32;
            /// x.saturating_square_assign();
            /// assert_eq!(x, 1000000);
            ///
            /// let mut x = 1000u16;
            /// x.saturating_square_assign();
            /// assert_eq!(x, u16::MAX);
            /// ```
            #[inline]
            fn saturating_square_assign(&mut self) {
                self.saturating_mul_assign(*self);
            }
        }
    };
}
apply_to_primitive_ints!(impl_saturating_square);

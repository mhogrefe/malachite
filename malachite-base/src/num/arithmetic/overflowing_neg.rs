use num::arithmetic::traits::{OverflowingNeg, OverflowingNegAssign};

macro_rules! impl_overflowing_neg {
    ($t:ident) => {
        impl OverflowingNeg for $t {
            type Output = $t;

            #[inline]
            fn overflowing_neg(self) -> ($t, bool) {
                $t::overflowing_neg(self)
            }
        }

        impl OverflowingNegAssign for $t {
            /// Replaces `self` with its negative.
            ///
            /// Returns a boolean indicating whether an arithmetic overflow would occur. If an
            /// overflow would have occurred then the wrapped value is assigned.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::OverflowingNegAssign;
            ///
            /// let mut x = 0i8;
            /// assert_eq!(x.overflowing_neg_assign(), false);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 100u64;
            /// assert_eq!(x.overflowing_neg_assign(), true);
            /// assert_eq!(x, 18446744073709551516);
            ///
            /// let mut x = -100i64;
            /// assert_eq!(x.overflowing_neg_assign(), false);
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -128i8;
            /// assert_eq!(x.overflowing_neg_assign(), true);
            /// assert_eq!(x, -128);
            /// ```
            #[inline]
            fn overflowing_neg_assign(&mut self) -> bool {
                let (result, overflow) = self.overflowing_neg();
                *self = result;
                overflow
            }
        }
    };
}
apply_to_primitive_ints!(impl_overflowing_neg);

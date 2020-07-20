use num::arithmetic::traits::{SaturatingNeg, SaturatingNegAssign};

macro_rules! impl_saturating_neg {
    ($t:ident) => {
        impl SaturatingNeg for $t {
            type Output = $t;

            #[inline]
            fn saturating_neg(self) -> $t {
                $t::saturating_neg(self)
            }
        }

        impl SaturatingNegAssign for $t {
            /// Replaces `self` with its negative, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingNegAssign;
            ///
            /// let mut x = 0i8;
            /// x.saturating_neg_assign();
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 100i64;
            /// x.saturating_neg_assign();
            /// assert_eq!(x, -100);
            ///
            /// let mut x = -100i64;
            /// x.saturating_neg_assign();
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -128i8;
            /// x.saturating_neg_assign();
            /// assert_eq!(x, 127);
            /// ```
            #[inline]
            fn saturating_neg_assign(&mut self) {
                *self = self.saturating_neg();
            }
        }
    };
}
apply_to_signeds!(impl_saturating_neg);

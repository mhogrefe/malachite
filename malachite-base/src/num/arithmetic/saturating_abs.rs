use num::arithmetic::traits::{SaturatingAbs, SaturatingAbsAssign};

macro_rules! impl_saturating_abs {
    ($t:ident) => {
        impl SaturatingAbs for $t {
            type Output = $t;

            #[inline]
            fn saturating_abs(self) -> $t {
                $t::saturating_abs(self)
            }
        }

        impl SaturatingAbsAssign for $t {
            /// Replace `self` with its absolute value, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingAbsAssign;
            ///
            /// let mut x = 0i8;
            /// x.saturating_abs_assign();
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 100i64;
            /// x.saturating_abs_assign();
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -100i64;
            /// x.saturating_abs_assign();
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -128i8;
            /// x.saturating_abs_assign();
            /// assert_eq!(x, 127);
            /// ```
            #[inline]
            fn saturating_abs_assign(&mut self) {
                *self = self.saturating_abs();
            }
        }
    };
}
apply_to_signeds!(impl_saturating_abs);

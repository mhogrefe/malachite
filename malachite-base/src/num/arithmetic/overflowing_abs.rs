use num::arithmetic::traits::{OverflowingAbs, OverflowingAbsAssign};

macro_rules! impl_overflowing_abs {
    ($t:ident) => {
        impl OverflowingAbs for $t {
            type Output = $t;

            #[inline]
            fn overflowing_abs(self) -> ($t, bool) {
                $t::overflowing_abs(self)
            }
        }

        impl OverflowingAbsAssign for $t {
            /// Replaces `self` with its absolute value.
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
            /// use malachite_base::num::arithmetic::traits::OverflowingAbsAssign;
            ///
            /// let mut x = 0i8;
            /// assert_eq!(x.overflowing_abs_assign(), false);
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 100i64;
            /// assert_eq!(x.overflowing_abs_assign(), false);
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -100i64;
            /// assert_eq!(x.overflowing_abs_assign(), false);
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -128i8;
            /// assert_eq!(x.overflowing_abs_assign(), true);
            /// assert_eq!(x, -128);
            /// ```
            #[inline]
            fn overflowing_abs_assign(&mut self) -> bool {
                let (result, overflow) = self.overflowing_abs();
                *self = result;
                overflow
            }
        }
    };
}
apply_to_signeds!(impl_overflowing_abs);

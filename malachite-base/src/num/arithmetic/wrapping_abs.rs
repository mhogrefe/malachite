use num::arithmetic::traits::{WrappingAbs, WrappingAbsAssign};

macro_rules! impl_wrapping_abs {
    ($t:ident) => {
        impl WrappingAbs for $t {
            type Output = $t;

            #[inline]
            fn wrapping_abs(self) -> $t {
                $t::wrapping_abs(self)
            }
        }

        impl WrappingAbsAssign for $t {
            /// Replaces `self` with its absolute value, wrapping around at the boundary of the
            /// type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingAbsAssign;
            ///
            /// let mut x = 0i8;
            /// x.wrapping_abs_assign();
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 100i64;
            /// x.wrapping_abs_assign();
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -100i64;
            /// x.wrapping_abs_assign();
            /// assert_eq!(x, 100);
            ///
            /// let mut x = -128i8;
            /// x.wrapping_abs_assign();
            /// assert_eq!(x, -128);
            /// ```
            #[inline]
            fn wrapping_abs_assign(&mut self) {
                *self = self.wrapping_abs();
            }
        }
    };
}

impl_wrapping_abs!(i8);
impl_wrapping_abs!(i16);
impl_wrapping_abs!(i32);
impl_wrapping_abs!(i64);
impl_wrapping_abs!(i128);
impl_wrapping_abs!(isize);

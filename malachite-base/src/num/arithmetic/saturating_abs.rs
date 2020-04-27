use num::arithmetic::traits::{SaturatingAbs, SaturatingAbsAssign};

macro_rules! impl_saturating_abs {
    ($t:ident) => {
        impl SaturatingAbs for $t {
            type Output = $t;

            /// Computes the absolute value of `self`, saturating at the numeric bounds instead of
            /// overflowing. For signed types, that means that this is ordinary `abs`, except that
            /// the absolute value of the smallest representable value is the largest representable
            /// value.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingAbs;
            ///
            /// assert_eq!(0i8.saturating_abs(), 0);
            /// assert_eq!(100i64.saturating_abs(), 100);
            /// assert_eq!((-100i64).saturating_abs(), 100);
            /// assert_eq!((-128i8).saturating_abs(), 127);
            /// ```
            #[inline]
            fn saturating_abs(self) -> $t {
                if self >= 0 {
                    self
                } else if self == $t::MIN {
                    $t::MAX
                } else {
                    -self
                }
            }
        }

        #[allow(unstable_name_collisions)]
        impl SaturatingAbsAssign for $t {
            /// Replace `self` with its absolute value, saturating at the numeric bounds instead of
            /// overflowing.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
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

impl_saturating_abs!(i8);
impl_saturating_abs!(i16);
impl_saturating_abs!(i32);
impl_saturating_abs!(i64);
impl_saturating_abs!(i128);
impl_saturating_abs!(isize);

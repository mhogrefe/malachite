use num::arithmetic::traits::{SaturatingNeg, SaturatingNegAssign};

macro_rules! impl_saturating_neg {
    ($t:ident) => {
        impl SaturatingNeg for $t {
            type Output = $t;

            /// Computes `-self`, saturating at the numeric bounds instead of overflowing. For
            /// signed types, that means that this is ordinary negation, except that the negative
            /// of the smallest representable value is the largest representable value.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::SaturatingNeg;
            ///
            /// assert_eq!(0i8.saturating_neg(), 0);
            /// assert_eq!(100i64.saturating_neg(), -100);
            /// assert_eq!((-100i64).saturating_neg(), 100);
            /// assert_eq!((-128i8).saturating_neg(), 127);
            /// ```
            #[inline]
            fn saturating_neg(self) -> $t {
                if self == $t::MIN {
                    $t::MAX
                } else {
                    -self
                }
            }
        }

        #[allow(unstable_name_collisions)]
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

impl_saturating_neg!(i8);
impl_saturating_neg!(i16);
impl_saturating_neg!(i32);
impl_saturating_neg!(i64);
impl_saturating_neg!(i128);
impl_saturating_neg!(isize);

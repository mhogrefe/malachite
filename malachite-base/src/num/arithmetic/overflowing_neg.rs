use num::arithmetic::traits::{OverflowingNeg, OverflowingNegAssign};

/// This macro defines trait implementations that are the same for unsigned and signed types.
macro_rules! impl_arithmetic_traits {
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
            /// # Example
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

impl_arithmetic_traits!(u8);
impl_arithmetic_traits!(u16);
impl_arithmetic_traits!(u32);
impl_arithmetic_traits!(u64);
impl_arithmetic_traits!(u128);
impl_arithmetic_traits!(usize);
impl_arithmetic_traits!(i8);
impl_arithmetic_traits!(i16);
impl_arithmetic_traits!(i32);
impl_arithmetic_traits!(i64);
impl_arithmetic_traits!(i128);
impl_arithmetic_traits!(isize);

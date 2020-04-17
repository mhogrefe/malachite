use num::arithmetic::traits::{OverflowingAdd, OverflowingAddAssign};

macro_rules! impl_overflowing_add {
    ($t:ident) => {
        impl OverflowingAdd<$t> for $t {
            type Output = $t;

            #[inline]
            fn overflowing_add(self, rhs: $t) -> ($t, bool) {
                $t::overflowing_add(self, rhs)
            }
        }

        impl OverflowingAddAssign for $t {
            /// Replaces `self` with `self + rhs`.
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
            /// use malachite_base::num::arithmetic::traits::OverflowingAddAssign;
            ///
            /// let mut x = 123u16;
            /// assert_eq!(x.overflowing_add_assign(456), false);
            /// assert_eq!(x, 579);
            ///
            /// let mut x = 123u8;
            /// assert_eq!(x.overflowing_add_assign(200), true);
            /// assert_eq!(x, 67);
            /// ```
            #[inline]
            fn overflowing_add_assign(&mut self, rhs: $t) -> bool {
                let (result, overflow) = self.overflowing_add(rhs);
                *self = result;
                overflow
            }
        }
    };
}

impl_overflowing_add!(u8);
impl_overflowing_add!(u16);
impl_overflowing_add!(u32);
impl_overflowing_add!(u64);
impl_overflowing_add!(u128);
impl_overflowing_add!(usize);
impl_overflowing_add!(i8);
impl_overflowing_add!(i16);
impl_overflowing_add!(i32);
impl_overflowing_add!(i64);
impl_overflowing_add!(i128);
impl_overflowing_add!(isize);

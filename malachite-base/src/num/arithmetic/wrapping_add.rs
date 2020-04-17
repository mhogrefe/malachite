use num::arithmetic::traits::{WrappingAdd, WrappingAddAssign};

macro_rules! impl_wrapping_add {
    ($t:ident) => {
        impl WrappingAdd<$t> for $t {
            type Output = $t;

            #[inline]
            fn wrapping_add(self, rhs: $t) -> $t {
                $t::wrapping_add(self, rhs)
            }
        }

        impl WrappingAddAssign for $t {
            /// Replaces `self` with `self + rhs`, wrapping around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingAddAssign;
            ///
            /// let mut x = 123u16;
            /// x.wrapping_add_assign(456);
            /// assert_eq!(x, 579);
            ///
            /// let mut x = 123u8;
            /// x.wrapping_add_assign(200);
            /// assert_eq!(x, 67);
            /// ```
            #[inline]
            fn wrapping_add_assign(&mut self, rhs: $t) {
                *self = self.wrapping_add(rhs);
            }
        }
    };
}

impl_wrapping_add!(u8);
impl_wrapping_add!(u16);
impl_wrapping_add!(u32);
impl_wrapping_add!(u64);
impl_wrapping_add!(u128);
impl_wrapping_add!(usize);
impl_wrapping_add!(i8);
impl_wrapping_add!(i16);
impl_wrapping_add!(i32);
impl_wrapping_add!(i64);
impl_wrapping_add!(i128);
impl_wrapping_add!(isize);

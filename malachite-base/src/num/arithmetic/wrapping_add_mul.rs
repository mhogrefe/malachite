use num::arithmetic::traits::{WrappingAddAssign, WrappingAddMul, WrappingAddMulAssign};

macro_rules! impl_wrapping_add_mul {
    ($t:ident) => {
        impl WrappingAddMul<$t> for $t {
            type Output = $t;

            /// Computes `self + y * z`, wrapping around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingAddMul;
            ///
            /// assert_eq!(2u8.wrapping_add_mul(3, 7), 23);
            /// assert_eq!((-127i8).wrapping_add_mul(-2, 100), -71);
            /// ```
            #[inline]
            fn wrapping_add_mul(self, y: $t, z: $t) -> $t {
                self.wrapping_add(y.wrapping_mul(z))
            }
        }

        impl WrappingAddMulAssign<$t> for $t {
            /// Replaces `self` with `self + y * z`, wrapping around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingAddMulAssign;
            ///
            /// let mut x = 2u8;
            /// x.wrapping_add_mul_assign(3, 7);
            /// assert_eq!(x, 23);
            ///
            /// let mut x = -127i8;
            /// x.wrapping_add_mul_assign(-2, 100);
            /// assert_eq!(x, -71);
            /// ```
            #[inline]
            fn wrapping_add_mul_assign(&mut self, y: $t, z: $t) {
                self.wrapping_add_assign(y.wrapping_mul(z));
            }
        }
    };
}

impl_wrapping_add_mul!(u8);
impl_wrapping_add_mul!(u16);
impl_wrapping_add_mul!(u32);
impl_wrapping_add_mul!(u64);
impl_wrapping_add_mul!(u128);
impl_wrapping_add_mul!(usize);
impl_wrapping_add_mul!(i8);
impl_wrapping_add_mul!(i16);
impl_wrapping_add_mul!(i32);
impl_wrapping_add_mul!(i64);
impl_wrapping_add_mul!(i128);
impl_wrapping_add_mul!(isize);

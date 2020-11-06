use num::arithmetic::traits::{
    WrappingAdd, WrappingAddAssign, WrappingAddMul, WrappingAddMulAssign, WrappingMul,
};

fn _wrapping_add_mul<T: WrappingAdd<T, Output = T> + WrappingMul<T, Output = T>>(
    x: T,
    y: T,
    z: T,
) -> T {
    x.wrapping_add(y.wrapping_mul(z))
}

fn _wrapping_add_mul_assign<T: WrappingAddAssign<T> + WrappingMul<T, Output = T>>(
    x: &mut T,
    y: T,
    z: T,
) {
    x.wrapping_add_assign(y.wrapping_mul(z));
}

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
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::WrappingAddMul;
            ///
            /// assert_eq!(2u8.wrapping_add_mul(3, 7), 23);
            /// assert_eq!((-127i8).wrapping_add_mul(-2, 100), -71);
            /// ```
            #[inline]
            fn wrapping_add_mul(self, y: $t, z: $t) -> $t {
                _wrapping_add_mul(self, y, z)
            }
        }

        impl WrappingAddMulAssign<$t> for $t {
            /// Replaces `self` with `self + y * z`, wrapping around at the boundary of the type.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
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
                _wrapping_add_mul_assign(self, y, z);
            }
        }
    };
}
apply_to_primitive_ints!(impl_wrapping_add_mul);

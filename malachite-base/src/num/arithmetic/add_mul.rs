use num::arithmetic::traits::{AddMul, AddMulAssign, WrappingAddMul, WrappingAddMulAssign};

macro_rules! impl_add_mul {
    ($t:ident) => {
        impl AddMul for $t {
            type Output = $t;

            /// Computes `self + y * z`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::AddMul;
            ///
            /// assert_eq!(2u8.add_mul(3, 7), 23);
            /// assert_eq!(127i8.add_mul(-2, 100), -73);
            /// ```
            #[inline]
            fn add_mul(self, y: $t, z: $t) -> $t {
                self.wrapping_add_mul(y, z)
            }
        }

        impl AddMulAssign for $t {
            /// Replaces `self` with `self + y * z`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::AddMulAssign;
            ///
            /// let mut x = 2u8;
            /// x.add_mul_assign(3, 7);
            /// assert_eq!(x, 23);
            ///
            /// let mut x = 127i8;
            /// x.add_mul_assign(-2, 100);
            /// assert_eq!(x, -73);
            /// ```
            #[inline]
            fn add_mul_assign(&mut self, y: $t, z: $t) {
                self.wrapping_add_mul_assign(y, z)
            }
        }
    };
}
apply_to_primitive_ints!(impl_add_mul);

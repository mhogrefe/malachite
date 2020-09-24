use num::conversion::traits::WrappingFrom;
use num::logic::traits::Rotate;

macro_rules! impl_rotate {
    ($t:ident) => {
        impl Rotate for $t {
            /// Rotate a value `n` bits to the left. Bits that leave the value from the left come
            /// back from the right.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::Rotate;
            ///
            /// assert_eq!(Rotate::rotate_left(123u8, 0), 123);
            /// assert_eq!(Rotate::rotate_left(123u8, 5), 111);
            /// assert_eq!(Rotate::rotate_left(123u8, 1005), 111);
            /// ```
            #[inline]
            fn rotate_left(self, n: u64) -> $t {
                $t::rotate_left(self, u32::wrapping_from(n))
            }

            /// Rotate a value `n` bits to the right. Bits that leave the value from the right come
            /// back from the left.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::logic::traits::Rotate;
            ///
            /// assert_eq!(Rotate::rotate_right(123u8, 0), 123);
            /// assert_eq!(Rotate::rotate_right(123u8, 3), 111);
            /// assert_eq!(Rotate::rotate_right(123u8, 1003), 111);
            /// ```
            #[inline]
            fn rotate_right(self, n: u64) -> $t {
                $t::rotate_right(self, u32::wrapping_from(n))
            }
        }
    };
}
apply_to_primitive_ints!(impl_rotate);

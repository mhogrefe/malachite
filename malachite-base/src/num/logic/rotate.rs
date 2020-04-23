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
            /// assert_eq!(Rotate::rotate_left(123u8, 1_005), 111);
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
            /// assert_eq!(Rotate::rotate_right(123u8, 1_003), 111);
            /// ```
            #[inline]
            fn rotate_right(self, n: u64) -> $t {
                $t::rotate_right(self, u32::wrapping_from(n))
            }
        }
    };
}

impl_rotate!(u8);
impl_rotate!(u16);
impl_rotate!(u32);
impl_rotate!(u64);
impl_rotate!(u128);
impl_rotate!(usize);
impl_rotate!(i8);
impl_rotate!(i16);
impl_rotate!(i32);
impl_rotate!(i64);
impl_rotate!(i128);
impl_rotate!(isize);

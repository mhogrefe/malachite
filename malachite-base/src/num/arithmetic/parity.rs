use num::arithmetic::traits::Parity;

macro_rules! impl_parity {
    ($t:ident) => {
        impl Parity for $t {
            /// Returns whether `self` is even.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::Parity;
            ///
            /// assert_eq!(0u8.even(), true);
            /// assert_eq!((-5i16).even(), false);
            /// assert_eq!(4u32.even(), true);
            /// ```
            #[inline]
            fn even(self) -> bool {
                (self & 1) == 0
            }

            /// Returns whether `self` is odd
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::Parity;
            ///
            /// assert_eq!(0u8.odd(), false);
            /// assert_eq!((-5i16).odd(), true);
            /// assert_eq!(4u32.odd(), false);
            /// ```
            #[inline]
            fn odd(self) -> bool {
                (self & 1) != 0
            }
        }
    };
}
impl_parity!(u8);
impl_parity!(u16);
impl_parity!(u32);
impl_parity!(u64);
impl_parity!(u128);
impl_parity!(usize);
impl_parity!(i8);
impl_parity!(i16);
impl_parity!(i32);
impl_parity!(i64);
impl_parity!(i128);
impl_parity!(isize);

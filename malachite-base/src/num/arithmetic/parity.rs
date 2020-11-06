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
            /// # Examples
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
            /// # Examples
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
apply_to_primitive_ints!(impl_parity);

use num::arithmetic::traits::Sign;
use std::cmp::Ordering;

macro_rules! impl_sign {
    ($t:ident) => {
        impl Sign for $t {
            /// Returns `Greater`, `Equal`, or `Less`, depending on whether `self` is positive,
            /// zero, or negative, respectively.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::Sign;
            /// use std::cmp::Ordering;
            ///
            /// assert_eq!(0u8.sign(), Ordering::Equal);
            /// assert_eq!(100u64.sign(), Ordering::Greater);
            /// assert_eq!((-100i16).sign(), Ordering::Less);
            /// ```
            #[inline]
            fn sign(&self) -> Ordering {
                self.cmp(&0)
            }
        }
    };
}
apply_to_primitive_ints!(impl_sign);

use num::arithmetic::traits::NegAssign;

macro_rules! impl_neg {
    ($t:ident) => {
        impl NegAssign for $t {
            /// Replaces `self` with its negative. Assumes that `self` can be negated.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::NegAssign;
            ///
            /// let mut x = 0i8;
            /// x.neg_assign();
            /// assert_eq!(x, 0);
            ///
            /// let mut x = 100i64;
            /// x.neg_assign();
            /// assert_eq!(x, -100);
            ///
            /// let mut x = -100i64;
            /// x.neg_assign();
            /// assert_eq!(x, 100);
            /// ```
            #[inline]
            fn neg_assign(&mut self) {
                *self = -*self;
            }
        }
    };
}
apply_to_signeds!(impl_neg);

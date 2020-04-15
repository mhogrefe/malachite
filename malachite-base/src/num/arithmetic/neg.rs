use num::arithmetic::traits::NegAssign;

macro_rules! impl_arithmetic_traits {
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

impl_arithmetic_traits!(i8);
impl_arithmetic_traits!(i16);
impl_arithmetic_traits!(i32);
impl_arithmetic_traits!(i64);
impl_arithmetic_traits!(i128);
impl_arithmetic_traits!(isize);

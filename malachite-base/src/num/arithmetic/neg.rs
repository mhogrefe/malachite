use num::arithmetic::traits::NegAssign;

macro_rules! impl_neg {
    ($t:ident) => {
        impl NegAssign for $t {
            /// Replaces `self` with its negative.
            ///
            /// Assumes that `self` can be negated.
            ///
            /// $x \gets -x$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::neg` module.
            #[inline]
            fn neg_assign(&mut self) {
                *self = -*self;
            }
        }
    };
}
apply_to_signeds!(impl_neg);

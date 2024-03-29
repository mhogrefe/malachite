use crate::num::logic::traits::NotAssign;

macro_rules! impl_not {
    ($t:ident) => {
        impl NotAssign for $t {
            /// Replaces a number with its bitwise negation.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::not#not_assign).
            #[inline]
            fn not_assign(&mut self) {
                *self = !*self;
            }
        }
    };
}
apply_to_primitive_ints!(impl_not);

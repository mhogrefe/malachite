use num::arithmetic::traits::{NextPowerOf2, NextPowerOf2Assign};

macro_rules! impl_next_power_of_2 {
    ($t:ident) => {
        impl NextPowerOf2 for $t {
            type Output = $t;

            #[inline]
            fn next_power_of_2(self) -> $t {
                $t::next_power_of_two(self)
            }
        }

        impl NextPowerOf2Assign for $t {
            /// Replaces `self` with the smallest power of 2 greater than or equal to `self`.
            ///
            /// $x \gets 2^{\lceil \log_2 x \rceil}$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if the next power of 2 is greater than the type's maximum value.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::next_power_of_2` module.
            #[inline]
            fn next_power_of_2_assign(&mut self) {
                *self = $t::next_power_of_2(*self);
            }
        }
    };
}
apply_to_unsigneds!(impl_next_power_of_2);

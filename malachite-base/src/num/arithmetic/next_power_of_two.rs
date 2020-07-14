use num::arithmetic::traits::{NextPowerOfTwo, NextPowerOfTwoAssign};

macro_rules! impl_next_power_of_two {
    ($t:ident) => {
        impl NextPowerOfTwo for $t {
            type Output = $t;

            #[inline]
            fn next_power_of_two(self) -> $t {
                $t::next_power_of_two(self)
            }
        }

        impl NextPowerOfTwoAssign for $t {
            /// Replaces `self` with the smallest power of two greater than or equal to `self`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Panics
            /// Panics if the next power of two is greater than the type's maximum value.
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::NextPowerOfTwoAssign;
            ///
            /// let mut x = 0u8;
            /// x.next_power_of_two_assign();
            /// assert_eq!(x, 1);
            ///
            /// let mut x = 4u16;
            /// x.next_power_of_two_assign();
            /// assert_eq!(x, 4);
            ///
            /// let mut x = 10u32;
            /// x.next_power_of_two_assign();
            /// assert_eq!(x, 16);
            ///
            /// let mut x = (1u64 << 40) - 5;
            /// x.next_power_of_two_assign();
            /// assert_eq!(x, 1 << 40);
            /// ```
            #[inline]
            fn next_power_of_two_assign(&mut self) {
                *self = $t::next_power_of_two(*self);
            }
        }
    };
}
apply_to_unsigneds!(impl_next_power_of_two);

use std::ops::{Add, AddAssign, Sub};

use num::arithmetic::traits::{ModAdd, ModAddAssign};

fn _mod_add<T: Copy + Ord>(x: T, other: T, m: T) -> T
where
    T: Add<T, Output = T> + Sub<T, Output = T>,
{
    let neg = m - x;
    if neg > other {
        x + other
    } else {
        other - neg
    }
}

fn _mod_add_assign<T: Copy + Ord>(x: &mut T, other: T, m: T)
where
    T: AddAssign<T> + Sub<T, Output = T>,
{
    let neg = m - *x;
    if neg > other {
        *x += other;
    } else {
        *x = other - neg;
    }
}

macro_rules! impl_mod_add {
    ($t:ident) => {
        impl ModAdd<$t> for $t {
            type Output = $t;

            /// Computes `self + other` mod `m`. Assumes the inputs are already reduced mod `m`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModAdd;
            ///
            /// assert_eq!(0u8.mod_add(3, 5), 3);
            /// assert_eq!(7u32.mod_add(5, 10), 2);
            /// ```
            ///
            /// This is nmod_add from nmod_vec.h, FLINT Dev 1.
            #[inline]
            fn mod_add(self, other: $t, m: $t) -> $t {
                _mod_add(self, other, m)
            }
        }

        impl ModAddAssign<$t> for $t {
            /// Replaces `self` with `self + other` mod `m`. Assumes the inputs are already reduced
            /// mod `m`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Example
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModAddAssign;
            ///
            /// let mut n = 0u8;
            /// n.mod_add_assign(3, 5);
            /// assert_eq!(n, 3);
            ///
            /// let mut n = 7u32;
            /// n.mod_add_assign(5, 10);
            /// assert_eq!(n, 2);
            /// ```
            ///
            /// This is nmod_add from nmod_vec.h, FLINT Dev 1, where the result is assigned to a.
            #[inline]
            fn mod_add_assign(&mut self, other: $t, m: $t) {
                _mod_add_assign(self, other, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_add);

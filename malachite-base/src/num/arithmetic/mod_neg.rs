use num::arithmetic::traits::{ModNeg, ModNegAssign};
use num::basic::traits::Zero;
use std::ops::Sub;

fn _mod_neg<T: Copy + Eq + Sub<T, Output = T> + Zero>(x: T, m: T) -> T {
    if x == T::ZERO {
        T::ZERO
    } else {
        m - x
    }
}

fn _mod_neg_assign<T: Copy + Eq + Sub<T, Output = T> + Zero>(x: &mut T, m: T) {
    if *x != T::ZERO {
        *x = m - *x;
    }
}

macro_rules! impl_mod_neg {
    ($t:ident) => {
        impl ModNeg for $t {
            type Output = $t;

            /// Computes `-self` mod `m`. Assumes the input is already reduced mod `m`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModNeg;
            ///
            /// assert_eq!(0u8.mod_neg(5), 0);
            /// assert_eq!(7u32.mod_neg(10), 3);
            /// assert_eq!(100u16.mod_neg(101), 1);
            /// ```
            ///
            /// This is nmod_neg from nmod_vec.h, FLINT Dev 1.
            #[inline]
            fn mod_neg(self, m: $t) -> $t {
                _mod_neg(self, m)
            }
        }

        impl ModNegAssign for $t {
            /// Replaces `self` with `-self` mod `m`. Assumes the input is already reduced mod `m`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::arithmetic::traits::ModNegAssign;
            ///
            /// let mut n = 0u8;
            /// n.mod_neg_assign(5);
            /// assert_eq!(n, 0);
            ///
            /// let mut n = 7u32;
            /// n.mod_neg_assign(10);
            /// assert_eq!(n, 3);
            ///
            /// let mut n = 100u16;
            /// n.mod_neg_assign(101);
            /// assert_eq!(n, 1);
            /// ```
            ///
            /// This is nmod_neg from nmod_vec.h, FLINT Dev 1, where the output is assign to a.
            #[inline]
            fn mod_neg_assign(&mut self, m: $t) {
                _mod_neg_assign(self, m)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_neg);

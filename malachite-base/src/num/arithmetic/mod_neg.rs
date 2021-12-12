use num::arithmetic::traits::{ModNeg, ModNegAssign};
use num::basic::traits::Zero;
use std::ops::Sub;

fn mod_neg<T: Copy + Eq + Sub<T, Output = T> + Zero>(x: T, m: T) -> T {
    if x == T::ZERO {
        T::ZERO
    } else {
        m - x
    }
}

fn mod_neg_assign<T: Copy + Eq + Sub<T, Output = T> + Zero>(x: &mut T, m: T) {
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
            /// $f(x, m) = y$, where $x, y < m$ and $-x \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_neg` module.
            ///
            /// This is nmod_neg from nmod_vec.h, FLINT 2.7.1.
            #[inline]
            fn mod_neg(self, m: $t) -> $t {
                mod_neg(self, m)
            }
        }

        impl ModNegAssign for $t {
            /// Replaces `self` with `-self` mod `m`. Assumes the input is already reduced mod `m`.
            ///
            /// $x \gets y$, where $x, y < m$ and $-x \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_neg` module.
            ///
            /// This is nmod_neg from nmod_vec.h, FLINT 2.7.1, where the output is assigned to a.
            #[inline]
            fn mod_neg_assign(&mut self, m: $t) {
                mod_neg_assign(self, m)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_neg);

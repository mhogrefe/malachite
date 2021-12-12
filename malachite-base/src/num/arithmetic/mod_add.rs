use num::arithmetic::traits::{ModAdd, ModAddAssign};
use std::ops::{Add, AddAssign, Sub};

fn mod_add<T: Add<T, Output = T> + Copy + Ord + Sub<T, Output = T>>(x: T, other: T, m: T) -> T {
    let neg = m - x;
    if neg > other {
        x + other
    } else {
        other - neg
    }
}

fn mod_add_assign<T: AddAssign<T> + Copy + Ord + Sub<T, Output = T>>(x: &mut T, other: T, m: T) {
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
            /// $f(x, y, m) = z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_add` module.
            ///
            /// This is nmod_add from nmod_vec.h, FLINT 2.7.1.
            #[inline]
            fn mod_add(self, other: $t, m: $t) -> $t {
                mod_add(self, other, m)
            }
        }

        impl ModAddAssign<$t> for $t {
            /// Replaces `self` with `self + other` mod `m`. Assumes the inputs are already reduced
            /// mod `m`.
            ///
            /// $x \gets z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_add` module.
            ///
            /// This is nmod_add from nmod_vec.h, FLINT 2.7.1, where the result is assigned to a.
            #[inline]
            fn mod_add_assign(&mut self, other: $t, m: $t) {
                mod_add_assign(self, other, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_add);

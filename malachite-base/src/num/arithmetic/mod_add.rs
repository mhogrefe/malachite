use crate::num::arithmetic::traits::{ModAdd, ModAddAssign};
use crate::num::basic::unsigneds::PrimitiveUnsigned;

fn mod_add<T: PrimitiveUnsigned>(x: T, y: T, m: T) -> T {
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
    assert!(y < m, "y must be reduced mod m, but {y} >= {m}");
    let neg = m - x;
    if neg > y {
        x + y
    } else {
        y - neg
    }
}

fn mod_add_assign<T: PrimitiveUnsigned>(x: &mut T, y: T, m: T) {
    assert!(*x < m, "x must be reduced mod m, but {x} >= {m}");
    assert!(y < m, "y must be reduced mod m, but {y} >= {m}");
    let neg = m - *x;
    if neg > y {
        *x += y;
    } else {
        *x = y - neg;
    }
}

macro_rules! impl_mod_add {
    ($t:ident) => {
        impl ModAdd<$t> for $t {
            type Output = $t;

            /// Adds two numbers modulo a third number $m$. The inputs must be already reduced
            /// modulo $m$.
            ///
            /// $f(x, y, m) = z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` or `other` are greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_add#mod_add).
            ///
            /// This is equivalent to `nmod_add` from `nmod_vec.h`, FLINT 2.7.1.
            #[inline]
            fn mod_add(self, other: $t, m: $t) -> $t {
                mod_add(self, other, m)
            }
        }

        impl ModAddAssign<$t> for $t {
            /// Adds two numbers modulo a third number $m$, in place. The inputs must be already
            /// reduced modulo $m$.
            ///
            /// $x \gets z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Panics
            /// Panics if `self` or `other` are greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_add#mod_add_assign).
            ///
            /// This is equivalent to `nmod_add` from `nmod_vec.h`, FLINT 2.7.1, where the result
            /// is assigned to `a`.
            #[inline]
            fn mod_add_assign(&mut self, other: $t, m: $t) {
                mod_add_assign(self, other, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_add);

use num::arithmetic::traits::{ModAdd, ModAddAssign};
use num::basic::unsigneds::PrimitiveUnsigned;

fn mod_add<T: PrimitiveUnsigned>(x: T, other: T, m: T) -> T {
    let neg = m - x;
    if neg > other {
        x + other
    } else {
        other - neg
    }
}

fn mod_add_assign<T: PrimitiveUnsigned>(x: &mut T, other: T, m: T) {
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

            /// Adds two numbers modulo a third number $m$. Assumes the inputs are already reduced
            /// modulo $m$.
            ///
            /// $f(x, y, m) = z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
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
            /// Adds two numbers modulo a third number $m$, in place. Assumes the inputs are
            /// already reduced modulo $m$.
            ///
            /// $x \gets z$, where $x, y, z < m$ and $x + y \equiv z \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
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

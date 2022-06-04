use num::arithmetic::traits::{ModSub, ModSubAssign};
use num::basic::unsigneds::PrimitiveUnsigned;

fn mod_sub<T: PrimitiveUnsigned>(x: T, other: T, m: T) -> T {
    let diff = x.wrapping_sub(other);
    if x < other {
        m.wrapping_add(diff)
    } else {
        diff
    }
}

macro_rules! impl_mod_sub {
    ($t:ident) => {
        impl ModSub<$t> for $t {
            type Output = $t;

            /// Subtracts two numbers modulo a third number $m$. Assumes the inputs are already
            /// reduced modulo $m$.
            ///
            /// $f(x, y, m) = z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_sub#mod_sub).
            ///
            /// This is equivalent to `nmod_sub` from `nmod_vec.h`, FLINT 2.7.1.
            #[inline]
            fn mod_sub(self, other: $t, m: $t) -> $t {
                mod_sub(self, other, m)
            }
        }

        impl ModSubAssign<$t> for $t {
            /// Subtracts two numbers modulo a third number $m$, in place. Assumes the inputs are
            /// already reduced modulo $m$.
            ///
            /// $x \gets z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_sub#mod_sub_assign).
            ///
            /// This is equivalent to `nmod_sub` from `nmod_vec.h`, FLINT 2.7.1, where the result
            /// is assigned to `a`.
            #[inline]
            fn mod_sub_assign(&mut self, other: $t, m: $t) {
                *self = self.mod_sub(other, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_sub);

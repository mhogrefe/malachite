use num::arithmetic::traits::{ModSub, ModSubAssign, WrappingAdd, WrappingSub};

fn _mod_sub<T: Copy + Ord + WrappingAdd<T, Output = T> + WrappingSub<T, Output = T>>(
    x: T,
    other: T,
    m: T,
) -> T {
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

            /// Computes `self - other` mod `m`. Assumes the inputs are already reduced mod `m`.
            ///
            /// $f(x, y, m) = z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_sub` module.
            ///
            /// This is nmod_sub from nmod_vec.h, FLINT Dev 1.
            #[inline]
            fn mod_sub(self, other: $t, m: $t) -> $t {
                _mod_sub(self, other, m)
            }
        }

        impl ModSubAssign<$t> for $t {
            /// Replaces `self` with `self - other` mod `m`. Assumes the inputs are already reduced
            /// mod `m`.
            ///
            /// $x \gets z$, where $x, y, z < m$ and $x - y \equiv z \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See the documentation of the `num::arithmetic::mod_sub` module.
            ///
            /// This is nmod_sub from nmod_vec.h, FLINT Dev 1, where the result is assigned to a.
            #[inline]
            fn mod_sub_assign(&mut self, other: $t, m: $t) {
                *self = self.mod_sub(other, m);
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_sub);

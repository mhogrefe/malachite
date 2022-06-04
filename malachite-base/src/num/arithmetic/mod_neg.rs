use num::arithmetic::traits::{ModNeg, ModNegAssign};
use num::basic::unsigneds::PrimitiveUnsigned;

fn mod_neg<T: PrimitiveUnsigned>(x: T, m: T) -> T {
    if x == T::ZERO {
        T::ZERO
    } else {
        m - x
    }
}

fn mod_neg_assign<T: PrimitiveUnsigned>(x: &mut T, m: T) {
    if *x != T::ZERO {
        *x = m - *x;
    }
}

macro_rules! impl_mod_neg {
    ($t:ident) => {
        impl ModNeg for $t {
            type Output = $t;

            /// Negates a number modulo another number $m$, in place. Assumes the input is already
            /// reduced modulo $m$.
            ///
            /// $f(x, m) = y$, where $x, y < m$ and $-x \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_neg#mod_neg).
            ///
            /// This is equivalent to `nmod_neg` from `nmod_vec.h`, FLINT 2.7.1.
            #[inline]
            fn mod_neg(self, m: $t) -> $t {
                mod_neg(self, m)
            }
        }

        impl ModNegAssign for $t {
            /// Negates a number modulo another number $m$. Assumes the input is already reduced
            /// modulo $m$.
            ///
            /// $x \gets y$, where $x, y < m$ and $-x \equiv y \mod m$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::mod_neg#mod_neg_assign).
            ///
            /// This is equivalent to `nmod_neg` from `nmod_vec.h`, FLINT 2.7.1, where the output
            /// is assigned to `a`.
            #[inline]
            fn mod_neg_assign(&mut self, m: $t) {
                mod_neg_assign(self, m)
            }
        }
    };
}
apply_to_unsigneds!(impl_mod_neg);

// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{Conjugate, ConjugateAssign};

macro_rules! impl_conjugate {
    ($t:ident) => {
        impl Conjugate for $t {
            type Output = $t;

            /// Computes the complex conjugate of a number. A real number is its own conjugate, so
            /// this is the identity.
            ///
            /// $f(x) = \overline{x} = x$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::conjugate#conjugate).
            #[inline]
            fn conjugate(self) -> $t {
                self
            }
        }

        impl ConjugateAssign for $t {
            /// Replaces a number with its complex conjugate. A real number is its own conjugate, so
            /// this does nothing.
            ///
            /// $x \gets \overline{x} = x$.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::conjugate#conjugate_assign).
            #[inline]
            fn conjugate_assign(&mut self) {}
        }
    };
}
apply_to_primitive_ints!(impl_conjugate);
apply_to_primitive_floats!(impl_conjugate);

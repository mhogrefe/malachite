// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::traits::{Reciprocal, ReciprocalAssign};

macro_rules! impl_reciprocal {
    ($t:ident) => {
        impl Reciprocal for $t {
            type Output = $t;

            /// Takes the reciprocal of a floating-point number.
            ///
            /// $$
            /// f(x) = 1/x+\varepsilon.
            /// $$
            /// Let $p$ be the precision of the input float (typically 24 for `f32`s and 53 for
            /// `f64`s, unless the float is subnormal).
            /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
            ///   be 0.
            /// - If $1/x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
            ///   2^{\lfloor\log_2 |1/x|\rfloor-p+1}$.
            /// - If $1/x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
            ///   2^{\lfloor\log_2 |1/x|\rfloor-p}$.
            ///
            /// If the output has a precision, it is `prec`.
            ///
            /// Special cases:
            /// - $f(\text{NaN})=\text{NaN}$
            /// - $f(\infty)=0.0$
            /// - $f(-\infty)=-0.0$
            /// - $f(0.0)=\infty$
            /// - $f(-0.0)=-\infty$
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::reciprocal#reciprocal).
            #[inline]
            fn reciprocal(self) -> $t {
                1.0 / self
            }
        }

        impl ReciprocalAssign for $t {
            /// Takes the reciprocal of a floating-point number, in place.
            ///
            /// $x \gets 1/x+\varepsilon$. Let $p$ be the precision of the input float (typically 24
            /// for `f32`s and 53 for `f64`s, unless the float is subnormal).
            /// - If $1/x$ is infinite, zero, or `NaN`, $\varepsilon$ may be ignored or assumed to
            ///   be 0.
            /// - If $1/x$ is finite and nonzero, and $m$ is not `Nearest`, then $|\varepsilon| <
            ///   2^{\lfloor\log_2 |1/x|\rfloor-p+1}$.
            /// - If $1/x$ is finite and nonzero, and $m$ is `Nearest`, then $|\varepsilon| <
            ///   2^{\lfloor\log_2 |1/x|\rfloor-p}$.
            ///
            /// See the `reciprocal` documentation for information on special cases.
            ///
            /// # Worst-case complexity
            /// Constant time and additional memory.
            ///
            /// # Examples
            /// See [here](super::reciprocal#reciprocal_assign).
            #[inline]
            fn reciprocal_assign(&mut self) {
                *self = 1.0 / *self;
            }
        }
    };
}
apply_to_primitive_floats!(impl_reciprocal);

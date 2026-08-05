// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::arithmetic::mod_div::gcdinv;
use crate::num::arithmetic::traits::ModInverse;
use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;

// The modular inverse is unique, so filtering the cofactor from `gcdinv` on the GCD being 1
// produces the same value as any other algorithm.
private_test_fn! {mod_inverse_binary<
    U: WrappingFrom<S> + PrimitiveUnsigned,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    x: U,
    m: U,
) -> Option<U> {
    assert_ne!(x, U::ZERO);
    assert!(x < m, "x must be reduced mod m, but {x} >= {m}");
    let (gcd, inverse) = gcdinv::<U, S>(x, m);
    if gcd == U::ONE {
        Some(inverse)
    } else {
        None
    }
}}

macro_rules! impl_mod_inverse {
    ($u:ident, $s:ident) => {
        impl ModInverse<$u> for $u {
            type Output = $u;

            /// Computes the multiplicative inverse of a number modulo another number $m$. The input
            /// must be already reduced modulo $m$.
            ///
            /// Returns `None` if $x$ and $m$ are not coprime.
            ///
            /// $f(x, m) = y$, where $x, y < m$, $\gcd(x, y) = 1$, and $xy \equiv 1 \mod m$.
            ///
            /// # Worst-case complexity
            /// $T(n) = O(n)$
            ///
            /// $M(n) = O(1)$
            ///
            /// where $T$ is time, $M$ is additional memory, and $n$ is
            /// `max(self.significant_bits(), m.significant_bits())`: the extended Euclidean
            /// algorithm on words performs $O(n)$ iterations of constant-cost word operations, with
            /// no allocation.
            ///
            /// # Panics
            /// Panics if `self` is greater than or equal to `m`.
            ///
            /// # Examples
            /// See [here](super::mod_inverse#mod_inverse).
            #[inline]
            fn mod_inverse(self, m: $u) -> Option<$u> {
                mod_inverse_binary::<$u, $s>(self, m)
            }
        }
    };
}
apply_to_unsigned_signed_pairs!(impl_mod_inverse);

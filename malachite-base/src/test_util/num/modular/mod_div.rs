// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;

// A simple reference implementation of `ModDiv`, using the textbook extended Euclidean algorithm.
// It computes the same cofactor as the shortcut-accelerated version in `mod_div.rs` (the quotient
// sequences are identical), so the returned quotients agree exactly.
pub fn mod_div_euclidean<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    b: U,
    c: U,
    m: U,
) -> Option<U> {
    assert!(b < m, "b must be reduced mod m, but {b} >= {m}");
    assert!(c < m, "c must be reduced mod m, but {c} >= {m}");
    if c == U::ZERO {
        return if b == U::ZERO { Some(U::ZERO) } else { None };
    }
    if b == U::ZERO {
        return Some(U::ZERO);
    }
    // Track the cofactor of c: the invariants are r ≡ v2 * c and x ≡ v1 * c mod m. The final
    // update of v2 may overflow harmlessly (its value is never used), so the updates are wrapping;
    // every used value is bounded by m / (2 * gcd).
    let mut x = m;
    let mut r = c;
    let mut v1 = S::ZERO;
    let mut v2 = S::ONE;
    while r != U::ZERO {
        let (quot, rem) = x.div_rem(r);
        x = r;
        r = rem;
        let t = v2;
        v2 = v1.wrapping_sub(S::wrapping_from(quot).wrapping_mul(v2));
        v1 = t;
    }
    let g = x;
    let (q, rem) = b.div_rem(g);
    if rem != U::ZERO {
        return None;
    }
    let mut s = U::wrapping_from(v1);
    if v1 < S::ZERO {
        s.wrapping_add_assign(m);
    }
    Some(q.mod_mul(s, m))
}

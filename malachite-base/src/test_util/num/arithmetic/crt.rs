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

// The multiplicative inverse via the textbook extended Euclidean algorithm; see `mod_div_euclidean`
// for the cofactor bookkeeping.
fn inverse_euclidean<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    c: U,
    m: U,
) -> Option<U> {
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
    if x != U::ONE {
        return None;
    }
    let mut s = U::wrapping_from(v1);
    if v1 < S::ZERO {
        s.wrapping_add_assign(m);
    }
    Some(s)
}

// A simple reference implementation of `Crt`, using the symmetric two-inverse formula x = (r1 * e1
// + r2 * e2) mod m1 * m2, where e1 is 1 mod m1 and 0 mod m2 and e2 is the reverse. The CRT solution
// is unique, so any algorithm agrees with any other.
pub fn crt_symmetric<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    r1: U,
    m1: U,
    r2: U,
    m2: U,
) -> Option<U> {
    assert!(r1 < m1, "r1 must be reduced mod m1, but {r1} >= {m1}");
    assert!(r2 < m2, "r2 must be reduced mod m2, but {r2} >= {m2}");
    let m = m1.checked_mul(m2).unwrap();
    // Each idempotent is bounded by the product of one modulus and a value reduced mod the other,
    // so both fit.
    let e1 = m2 * inverse_euclidean::<U, S>(m2 % m1, m1)?;
    let e2 = m1 * inverse_euclidean::<U, S>(m1 % m2, m2)?;
    Some(r1.mod_mul(e1, m).mod_add(r2.mod_mul(e2, m), m))
}

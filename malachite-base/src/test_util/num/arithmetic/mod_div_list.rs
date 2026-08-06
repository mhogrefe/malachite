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
use crate::test_util::num::arithmetic::mod_div::mod_div_euclidean;

// A simple reference implementation of `ModDivList`, deriving the progression from any single
// quotient: the solutions of `qc ≡ b mod m` are spaced `m / gcd(c, m)` apart, and the smallest is
// any quotient reduced modulo the spacing. The result is canonical, so this agrees exactly with the
// implementation in `mod_div_list.rs`.
pub fn mod_div_list_euclidean<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    b: U,
    c: U,
    m: U,
) -> Option<(U, U, U)> {
    let q = mod_div_euclidean::<U, S>(b, c, m)?;
    let length = c.gcd(m);
    let stride = m / length;
    Some((q % stride, stride, length))
}

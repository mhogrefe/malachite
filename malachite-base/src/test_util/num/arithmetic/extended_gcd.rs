// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;

pub fn extended_gcd_unsigned_euclidean<
    U: PrimitiveUnsigned,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    a: U,
    b: U,
) -> (U, S, S) {
    if a == U::ZERO && b == U::ZERO {
        (U::ZERO, S::ZERO, S::ZERO)
    } else if a == b || a == U::ZERO {
        (b, S::ZERO, S::ONE)
    } else {
        let (gcd, x, y) = extended_gcd_unsigned_euclidean(b % a, a);
        (gcd, y - S::wrapping_from(b / a) * x, x)
    }
}

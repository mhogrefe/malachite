// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::num::basic::signeds::PrimitiveSigned;
use crate::num::basic::unsigneds::PrimitiveUnsigned;
use crate::num::conversion::traits::WrappingFrom;
use crate::test_util::num::arithmetic::extended_gcd::extended_gcd_unsigned_euclidean;

pub fn mod_inverse_euclidean<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    x: U,
    m: U,
) -> Option<U> {
    assert_ne!(x, U::ZERO);
    assert!(x < m);
    let (gcd, inverse, _) = extended_gcd_unsigned_euclidean::<U, S>(x, m);
    if gcd == U::ONE {
        Some(if inverse >= S::ZERO {
            U::wrapping_from(inverse)
        } else {
            U::wrapping_from(inverse).wrapping_add(m)
        })
    } else {
        None
    }
}

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

pub fn mod_power_of_2_inverse_euclidean<
    U: PrimitiveUnsigned + WrappingFrom<S>,
    S: PrimitiveSigned + WrappingFrom<U>,
>(
    a: U,
    pow: u64,
) -> Option<U> {
    assert_ne!(a, U::ZERO);
    assert!(pow <= U::WIDTH);
    assert!(a.significant_bits() <= pow);
    if a.even() {
        return None;
    } else if a == U::ONE {
        return Some(U::ONE);
    }
    Some(if pow == U::WIDTH {
        let (q, r) = U::xx_div_mod_y_to_qr(U::ONE, U::ZERO, a);
        let (_, x, y) = extended_gcd_unsigned_euclidean(r, a);
        U::wrapping_from(y - S::wrapping_from(q) * x)
    } else {
        let m = U::power_of_2(pow);
        let (_, x, _) = extended_gcd_unsigned_euclidean::<U, S>(a, m);
        if x >= S::ZERO {
            U::wrapping_from(x)
        } else {
            U::wrapping_from(x).wrapping_add(m)
        }
    })
}

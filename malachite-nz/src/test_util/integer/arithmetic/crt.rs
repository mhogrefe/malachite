// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use crate::test_util::natural::arithmetic::crt::crt_simple;
use malachite_base::num::arithmetic::traits::{Mod, UnsignedAbs};
use malachite_base::num::conversion::traits::ExactFrom;

// A simple reference implementation of `BalancedCrt`: the canonical solution from `crt_simple`,
// reduced to the representative of smallest absolute value by an explicit comparison. Ties go to
// the positive representative.
pub fn balanced_crt_simple(r1: Integer, m1: Natural, r2: Natural, m2: Natural) -> Option<Integer> {
    let m = &m1 * &m2;
    let r1n = Natural::exact_from(r1.mod_op(Integer::from(&m1)));
    let x = crt_simple(r1n, m1, r2, m2)?;
    Some(if &x << 1u32 > m {
        Integer::from(x) - Integer::from(m)
    } else {
        Integer::from(x)
    })
}

// The balanced representative is congruent to the canonical one, so its unsigned residue is
// recoverable; used by tests to relate the two forms.
pub fn balanced_to_canonical(x: &Integer, m: &Natural) -> Natural {
    if *x < 0u32 {
        m - x.unsigned_abs_ref()
    } else {
        x.unsigned_abs()
    }
}

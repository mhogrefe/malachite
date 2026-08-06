// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::integer::Integer;
use crate::natural::Natural;
use malachite_base::num::arithmetic::traits::{
    AddMul, Crt, ExtendedGcd, Mod, ModMul, ModSub, UnsignedAbs,
};

// A simple reference implementation of `Crt`, computing the inverse through the public
// `ExtendedGcd` rather than the FLINT-derived kernels. The CRT solution is unique, so any algorithm
// agrees with any other.
pub fn crt_simple(r1: Natural, m1: Natural, r2: Natural, m2: Natural) -> Option<Natural> {
    assert!(r1 < m1);
    assert!(r2 < m2);
    let (g, a, _) = (&m1).extended_gcd(&m2);
    if g != 1u32 {
        return None;
    }
    // a * m1 + b * m2 = 1, so a is an inverse of m1 modulo m2; lift it to [0, m2).
    let inv = a.mod_op(Integer::from(&m2)).unsigned_abs();
    let s = r2.mod_sub(&r1 % &m2, &m2).mod_mul(inv, m2);
    Some(r1.add_mul(m1, s))
}

// A simple reference implementation of multi-modulus Chinese remaindering: a left fold of the pair
// combination `crt`. It diverges from `MultiCrt` on lists that contain a 1 among two or more
// moduli: the pair combination treats a modulus of 1 as a vacuous congruence, while `MultiCrt::new`
// rejects it. Callers should avoid such lists.
pub fn multi_crt_simple(moduli: &[Natural], values: &[Natural]) -> Option<Natural> {
    assert_eq!(moduli.len(), values.len());
    if moduli[0] == 0u32 {
        return None;
    }
    let mut x = values[0].clone();
    let mut m = moduli[0].clone();
    for (mi, v) in moduli.iter().zip(values.iter()).skip(1) {
        x = (&x).crt(&m, v, mi)?;
        m *= mi;
    }
    Some(x)
}

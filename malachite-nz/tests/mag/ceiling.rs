// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::One;
use malachite_nz::mag::Mag;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{mag_gen_var_1, natural_gen};
use malachite_nz::test_util::mag::{cmp_scaled, mag_to_scaled_natural};
use std::cmp::Ordering::*;
use std::str::FromStr;

#[test]
fn test_ceiling() {
    let test = |s, out| {
        assert_eq!(Mag::from_str(s).unwrap().ceiling().to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("3*2^-1", "2");
    test("1073741823*2^-29", "2");
    test("1073741823", "1073741823");
    test("3*2^100", "3802951800684688204490109616128");
}

#[test]
fn ceiling_properties() {
    mag_gen_var_1().test_properties(|x| {
        let c = x.ceiling();
        let (a, i) = mag_to_scaled_natural(x);
        // - the ceiling is at least x, and c - 1 is less than x
        assert_ne!(cmp_scaled(&c, 0, &a, i), Less);
        if c != 0u32 {
            assert_eq!(cmp_scaled(&(&c - Natural::ONE), 0, &a, i), Less);
        }
    });

    natural_gen().test_properties(|x| {
        // - the ceiling of the upper bound of x is at least x
        assert!(Mag::from(&x).ceiling() >= x);
    });

    assert_eq!(Mag::ZERO.ceiling(), 0u32);
    assert_eq!(Mag::ONE.ceiling(), 1u32);
}

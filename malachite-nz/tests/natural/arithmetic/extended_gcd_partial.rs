// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Gcd, UnsignedAbs};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::gcd::extended_gcd_partial::extended_gcd_partial;
use malachite_nz::test_util::generators::natural_triple_gen_var_10;
use std::str::FromStr;

#[test]
fn test_extended_gcd_partial() {
    let test = |r2, r1, l, out: (&str, &str, &str, &str)| {
        let r2 = Natural::from_str(r2).unwrap();
        let r1 = Natural::from_str(r1).unwrap();
        let l = Natural::from_str(l).unwrap();
        let (co2, co1, r2_out, r1_out) = extended_gcd_partial(r2, r1, &l);
        assert_eq!(co2.to_string(), out.0);
        assert_eq!(co1.to_string(), out.1);
        assert_eq!(r2_out.to_string(), out.2);
        assert_eq!(r1_out.to_string(), out.3);
    };
    // - r1 already at or below the bound: everything unchanged, initial cofactors
    test("13", "8", "8", ("0", "-1", "13", "8"));
    test("13", "8", "100", ("0", "-1", "13", "8"));
    // - a zero r1: no iterations
    test("13", "0", "2", ("0", "-1", "13", "0"));
    // - a small full run, three word-batched rounds (hand-computed)
    test("13", "8", "2", ("-2", "3", "3", "2"));
    // - a tiny r1 makes the first word quotient too large for the termination test, so no word step
    //   commits and a single big Euclidean step runs instead (hand-computed)
    test("13", "1", "0", ("-1", "13", "1", "0"));
    // - a bound of zero runs the remainder sequence to the end
    test("13", "8", "0", ("-5", "13", "1", "0"));
    // - single-limb values large enough to exercise several word steps per round
    test(
        "123456789012345678",
        "98765432109876543",
        "1000000",
        ("3484224932", "-5294924503", "23045262", "411531"),
    );
    // - multi-limb remainders with a multi-limb bound: the Lehmer approximation rounds
    test(
        "98765432123456789012345678990",
        "12345678987654321012345678901",
        "1000000000000000",
        (
            "56444444791",
            "-72000000442",
            "1371709188912246479",
            "41986691309592",
        ),
    );
    // - a bound of 1 runs the sequence to the end
    test(
        "98765432123456789012345678990",
        "12345678987654321012345678901",
        "1",
        (
            "-20643213072386783441340849732",
            "39061109525535002785502414629",
            "2",
            "1",
        ),
    );
}

#[test]
fn extended_gcd_partial_properties() {
    natural_triple_gen_var_10().test_properties(|(r1, r2, l)| {
        let (co2, co1, r2_out, r1_out) = extended_gcd_partial(r2.clone(), r1.clone(), &l);
        // the sequence stops at or below the bound, or at zero
        assert!(r1_out == 0u32 || r1_out <= l);
        // remainders never grow
        assert!(r2_out <= r2 || r1 > r2);
        // the cofactor identity: co2 * r1 - co1 * r2 == ±r2_orig
        let combination = &co2 * Integer::from(&r1_out) - &co1 * Integer::from(&r2_out);
        assert_eq!(combination.unsigned_abs(), r2);
        // the GCD of the pair is preserved
        assert_eq!((&r1_out).gcd(&r2_out), (&r1).gcd(&r2));
        // a bound at least r1 leaves everything untouched
        if l >= r1 {
            assert_eq!(co2, Integer::ZERO);
            assert_eq!(co1, Integer::NEGATIVE_ONE);
            assert_eq!(r2_out, r2);
            assert_eq!(r1_out, r1);
        }
        let _ = Natural::ONE;
    });
}

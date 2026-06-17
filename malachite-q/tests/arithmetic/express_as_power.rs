// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Parity, Pow};
use malachite_base::num::factorization::traits::ExpressAsPower;
use malachite_base::strings::ToDebugString;
use malachite_q::Rational;
use malachite_q::test_util::generators::{rational_gen, rational_unsigned_pair_gen_var_4};
use std::str::FromStr;

#[test]
fn test_express_as_power() {
    let test = |s: &str, out: &str| {
        assert_eq!(
            Rational::from_str(s)
                .unwrap()
                .express_as_power()
                .to_debug_string(),
            out
        );
    };
    // 0 and 1 are expressed as squares, matching the integer implementations.
    test("0", "Some((0, 2))");
    test("1", "Some((1, 2))");
    // -1 has no maximal exponent (it is (-1)^k for every odd k).
    test("-1", "None");

    // Integers (matching Natural::express_as_power).
    test("4", "Some((2, 2))");
    test("8", "Some((2, 3))");
    test("9", "Some((3, 2))");
    test("64", "Some((2, 6))");
    test("100", "Some((10, 2))");

    // Proper fractions: the exponent is positive, the root absorbs the direction.
    test("1/9", "Some((1/3, 2))");
    test("1/8", "Some((1/2, 3))");
    test("1/64", "Some((1/2, 6))");
    test("9/4", "Some((3/2, 2))");
    test("8/27", "Some((2/3, 3))");
    test("25/9", "Some((5/3, 2))");
    test("16/81", "Some((2/3, 4))");

    // Negatives: only odd exponents have a real root.
    test("-8", "Some((-2, 3))");
    test("-27", "Some((-3, 3))");
    test("-1/8", "Some((-1/2, 3))");
    test("-8/27", "Some((-2/3, 3))");

    // Not perfect powers.
    test("2", "None");
    test("6", "None");
    test("3/2", "None");
    test("1/3", "None");
    test("3/4", "None"); // 3 and 4 share no common exponent > 1
    test("-2", "None");
    test("-4", "None"); // 4 = 2^2 has only an even exponent, so -4 has no real root
    test("-1/4", "None");
    test("-9/4", "None"); // (3/2)^2 is positive; the negative needs an odd exponent
}

#[test]
fn express_as_power_properties() {
    rational_gen().test_properties(|x| {
        if let Some((p, e)) = x.express_as_power() {
            assert!(p.is_valid());
            assert!(e > 1);
            assert_eq!((&p).pow(e), x);
            // The root is itself primitive (not a perfect power), except for the 0 and 1 special
            // cases, whose roots are 0 and 1.
            if p != 0u32 && p != 1u32 && p != -1i32 {
                assert!(p.express_as_power().is_none());
            }
        }
    });

    rational_unsigned_pair_gen_var_4::<u64>().test_properties(|(x, y)| {
        // x^y is a perfect power for y > 1, except when x is +-1 or 0 (giving +-1 or 0, which lack a
        // maximal exponent or are the special cases).
        if y > 1 && x != 0u32 && x != 1u32 && x != -1i32 {
            let power = (&x).pow(y);
            let (p, e) = power.express_as_power().unwrap();
            assert!(e > 1);
            assert_eq!((&p).pow(e), power);
            // When x is itself primitive and the sign is unambiguous (positive base, or odd
            // exponent), the recovered (root, exponent) is exactly (x, y).
            if x.express_as_power().is_none() && (x > 0u32 || y.odd()) {
                assert_eq!(p, x);
                assert_eq!(e, y);
            }
        }
    });
}

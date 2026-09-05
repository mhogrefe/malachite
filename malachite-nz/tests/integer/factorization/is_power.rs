// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Parity, Pow};
use malachite_base::num::factorization::traits::{ExpressAsPower, IsPower};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::{integer_gen, integer_unsigned_pair_gen_var_3};
use std::str::FromStr;

#[test]
fn test_is_power() {
    let test = |s, out| {
        assert_eq!(Integer::from_str(s).unwrap().is_power(), out);
    };
    // - *self >= 0: delegated to the `Natural` implementation
    test("0", true);
    test("1", true);
    test("4", true);
    test("6", false);
    test("8", true);
    test("64", true);
    test("1000000000000", true);
    // - abs == 1u32: -1 is (-1)^3, but its bit length admits no exponent to search
    test("-1", true);
    // - a negative value can only be an odd perfect power, so the search finds nothing here
    test("-4", false);
    test("-6", false);
    test("-8", true);
    // - -16 is 2^4 in absolute value, and 4 has no odd divisor above 1
    test("-16", false);
    // - the odd-prime search succeeds: -64 is (-4)^3
    test("-64", true);
    // -10^12 is (-10000)^3, since 10^12 = (10^4)^3
    test("-1000000000000", true);
    test("-1000000000000000000", true);
    // 81 is 3^4, whose only exponents above 1 are 2 and 4, both even
    test("-81", false);
    test("-6561", false);
}

#[test]
fn test_express_as_power() {
    let test = |s, out: Option<(&str, u64)>| {
        assert_eq!(
            Integer::from_str(s)
                .unwrap()
                .express_as_power()
                .map(|(root, exp)| (root.to_string(), exp)),
            out.map(|(root, exp)| (root.to_string(), exp))
        );
    };
    test("8", Some(("2", 3)));
    test("6", None);
    test("-1", Some(("-1", 3)));
    test("-8", Some(("-2", 3)));
    test("-16", None);
    test("-64", Some(("-4", 3)));
}

#[test]
fn is_power_properties() {
    integer_gen().test_properties(|n| {
        let is_power = n.is_power();
        // the predicate and the witness agree
        let expressed = n.express_as_power();
        assert_eq!(is_power, expressed.is_some());
        if let Some((root, exp)) = expressed {
            assert!(exp > 1);
            assert_eq!((&root).pow(exp), n);
            // a negative perfect power has an odd exponent and a negative root
            if n < 0u32 {
                assert!(exp.odd());
                assert!(root < 0u32);
            }
        }
        // GMP agrees, negative operands included
        assert_eq!(rug::Integer::from(&n).is_perfect_power(), is_power);
        // a value and its negation agree exactly when the value is an odd power or zero
        if n != 0u32 && (-&n).is_power() {
            assert!(n.unsigned_abs_ref().is_power());
        }
    });

    integer_unsigned_pair_gen_var_3().test_properties(|(n, exp): (Integer, u64)| {
        // any perfect power with an exponent above 1 is recognized
        if exp > 1 {
            assert!((&n).pow(exp).is_power());
        }
    });
}

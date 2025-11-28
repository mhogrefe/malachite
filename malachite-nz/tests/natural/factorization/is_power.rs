// Copyright © 2025 William Youmans and Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Pow, Square};
use malachite_base::num::factorization::traits::{ExpressAsPower, IsPower};
use malachite_base::strings::ToDebugString;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{natural_gen, natural_unsigned_pair_gen_var_4};
use std::str::FromStr;

#[test]
fn test_is_power() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().is_power(), out);
        assert_eq!(
            Natural::from_str(n).unwrap().express_as_power().is_some(),
            out
        );
        assert_eq!(rug::Integer::from_str(n).unwrap().is_perfect_power(), out);
    };
    test("0", true);
    test("1", true);
    test("4", true);
    test("8", true);
    test("9", true);
    test("16", true);
    test("25", true);
    test("27", true);
    test("32", true);
    test("64", true);
    test("81", true);
    test("100", true);
    test("125", true);
    test("243", true);
    test("1024", true);
    test("1296", true);
    // - in get_perfect_power_natural
    // - pow_2 != 1 first time in get_perfect_power_natural
    // - !pow_2.is_prime() in get_perfect_power_natural
    // - !(&q).divisible_by(&prime_nat) in get_perfect_power_natural
    // - (&q).divisible_by(&prime_nat) in get_perfect_power_natural
    // - (&q).divisible_by(&prime_squared)
    // - pow_2 != 1 second time in get_perfect_power_natural
    // - q == 1u32 || pow_2.is_prime() in get_perfect_power_natural
    test(
        "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000",
        true,
    );
    // - pow_2 == 0 in get_perfect_power_natural
    // - nth <= bits in get_perfect_power_natural
    // - pow_2 == 0 && let Some(root) = n.checked_root(nth) in get_perfect_power_natural
    test("999898004334901741252806480882137569", true);
    // - pow_2 == 0 && let Some(root) != n.checked_root(nth) in get_perfect_power_natural
    // - pow_2 == 0 && q > SMALLEST_OMITTED_PRIME in get_perfect_power_natural
    test("999949000866995087", true);
    // - nth > bits in get_perfect_power_natural
    test("999983", false);
    test("115230877647233745723406127208308085892801", true);
    // - !(&q).divisible_by(&prime_squared)
    test("113", false);

    test("2", false);
    test("3", false);
    test("5", false);
    test("6", false);
    test("7", false);
    // - pow_2 == 1 first time in get_perfect_power_natural
    test("10", false);
    test("12", false);
    test("15", false);

    // - pow_2.is_prime() in get_perfect_power_natural
    test("1470862095575962348216", false);
    // - q != 1u32 && !pow_2.is_prime() in get_perfect_power_natural
    test("242811787435972937453260179", false);
    // - pow_2 != 0 in get_perfect_power_natural
    // - pow_2 % nth == 0 in get_perfect_power_natural
    // - pow_2 != 0 && let Some(root) != n.checked_root(nth) in get_perfect_power_natural
    // - pow_2 != 0 && q > SMALLEST_OMITTED_PRIME in get_perfect_power_natural
    // - pow_2 % nth != 0 in get_perfect_power_natural
    test("7176540100844819483539782848", false);
    // - pow_2 == 1 second time in get_perfect_power_natural
    test(
        "23195532513672842039279098010453211078197157913306231000360318871559723303353757940843652\
        0540453520",
        false,
    );
    // - pow_2 != 0 && let Some(root) = n.checked_root(nth) in get_perfect_power_natural
    test("4722366482869645213696", true);
    // - pow_2 != 0 && q <= SMALLEST_OMITTED_PRIME in get_perfect_power_natural
    test("36353056192134643712", false);
}

#[test]
fn test_is_power_edge_cases() {
    // Test some specific edge cases

    // Powers of 2 that are perfect powers
    let power_of_2_power: Natural = Natural::from(1u64) << 0x1000;
    assert!(power_of_2_power.is_power());

    let power_of_2_non_power = power_of_2_power + Natural::from(1u64);
    assert!(!power_of_2_non_power.is_power());

    // Large powers
    let big_base = Natural::from_str("987654321098765432109876543210").unwrap();
    let big_power = (&big_base).pow(3);
    assert!(big_power.is_power());

    let big_non_power = &big_power + Natural::from(1u32);
    assert!(!big_non_power.is_power());

    // Test prime 1009 (SMALLEST_OMITTED_PRIME) - ensures termination
    assert!(!Natural::from(1009u32).is_power());
}

#[test]
fn test_express_as_power() {
    let test = |n, out| {
        assert_eq!(
            Natural::from_str(n)
                .unwrap()
                .express_as_power()
                .to_debug_string(),
            out
        );
    };
    test("0", "Some((0, 2))");
    test("1", "Some((1, 2))");
    test("4", "Some((2, 2))");
    test("8", "Some((2, 3))");
    test("9", "Some((3, 2))");
    test("16", "Some((2, 4))");
    test("25", "Some((5, 2))");
    test("27", "Some((3, 3))");
    test("32", "Some((2, 5))");
    test("64", "Some((2, 6))");
    test("81", "Some((3, 4))");
    test("100", "Some((10, 2))");
    test("125", "Some((5, 3))");
    test("243", "Some((3, 5))");
    test("1024", "Some((2, 10))");
    test("1296", "Some((6, 4))");
    test(
        "10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        000000000000",
        "Some((10, 100))",
    );
    test("999898004334901741252806480882137569", "Some((999983, 6))");
    test(
        "115230877647233745723406127208308085892801",
        "Some((113, 20))",
    );

    test("2", "None");
    test("3", "None");
    test("5", "None");
    test("6", "None");
    test("7", "None");
    test("10", "None");
    test("12", "None");
    test("15", "None");
}

#[test]
fn is_power_properties() {
    natural_gen().test_properties(|x| {
        let is_power = x.is_power();

        // Consistency: is_power() should match express_as_power()
        assert_eq!(
            is_power,
            x.express_as_power().is_some(),
            "is_power() and express_as_power() inconsistent for {x}",
        );
        assert_eq!(rug::Integer::from(&x).is_perfect_power(), is_power);

        // Any number raised to a power >= 2 should be a perfect power
        if x > 1u32 {
            let power_2 = (&x).square();
            assert!(power_2.is_power(), "{x}^2 should be a perfect power");

            let power_3 = (&x).pow(3);
            assert!(power_3.is_power(), "{x}^3 should be a perfect power");
        }
    });

    natural_unsigned_pair_gen_var_4::<u64>().test_properties(|(x, y)| {
        if y > 1 {
            let power = (&x).pow(y);
            assert!(power.is_power());
        }
    });
}

#[test]
fn express_as_power_properties() {
    natural_gen().test_properties(|x| {
        if let Some((p, e)) = x.express_as_power() {
            assert!(e > 1);
            assert_eq!((&p).pow(e), x);
            if x > 1u32 {
                assert!(p.express_as_power().is_none());
            }
        }
    });

    natural_unsigned_pair_gen_var_4::<u64>().test_properties(|(x, y)| {
        if y > 1 {
            let power = (&x).pow(y);
            let ope = power.express_as_power();
            assert!(ope.is_some());
            let (p, e) = ope.unwrap();
            assert_eq!((&p).pow(e), power);
            if x.express_as_power().is_none() {
                assert_eq!(x, p);
                assert_eq!(y, e);
            }
        }
    });
}

// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    ModPowerOf2, ModPowerOf2Neg, ModPowerOf2Sqrt, ModPowerOf2Square, Parity,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::test_util::generators::unsigned_pair_gen_var_17;
use malachite_nz::natural::Natural;
use malachite_nz::natural::arithmetic::mod_power_of_2_sqrt::mod_power_of_2_sqrt_natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_unsigned_pair_gen_var_11, natural_unsigned_pair_gen_var_15,
};
use malachite_nz::test_util::natural::arithmetic::mod_power_of_2_sqrt::mod_power_of_2_sqrt_naive;
use std::str::FromStr;

#[test]
fn test_mod_power_of_2_sqrt() {
    let test = |x, pow, out: Option<&str>| {
        let x = Natural::from_str(x).unwrap();
        let out = out.map(|s| Natural::from_str(s).unwrap());
        assert_eq!((&x).mod_power_of_2_sqrt(pow), out);
        assert_eq!(mod_power_of_2_sqrt_natural(&x, pow), out);
        assert_eq!(x.mod_power_of_2_sqrt(pow), out);
    };
    // - x == 0
    test("0", 0, Some("0"));
    test("0", 1, Some("0"));
    test("0", 64, Some("0"));
    // - n <= 2, so u must be 1 and no Newton steps are taken
    test("1", 1, Some("1"));
    test("1", 2, Some("1"));
    test("4", 3, Some("2"));
    // - u is not 1 mod 8
    test("3", 2, None);
    test("3", 3, None);
    test("5", 3, None);
    test("12", 4, None);
    test("48", 64, None);
    // - the power of 2 dividing x is odd
    test("2", 2, None);
    test("2", 3, None);
    test("8", 4, None);
    // - Newton steps; the four candidates are distinct roots
    test("1", 3, Some("1"));
    test("9", 4, Some("3"));
    test("17", 5, Some("7"));
    test("25", 5, Some("5"));
    test("4", 5, Some("2"));
    test("16", 5, Some("4"));
    test("17", 64, Some("405959429219100393"));
    test("17825792", 64, Some("2195515552539648"));
    // - several limbs; the least root of a square y ^ 2 is y when y is less than its negation
    test(
        "152415787532388367504953515625361987875019051998750190521",
        200,
        Some("12345678901234567890123456789"),
    );
    test(
        "9754610402072855520317025000023167224001219327920012193344",
        200,
        Some("98765431209876543120987654312"),
    );
    test(
        "19509220804145711040634050000046334448002438655840024386688",
        200,
        None,
    );
    test(
        "152415787532388367504953515625361987875019051998750190523",
        200,
        None,
    );
}

#[test]
#[should_panic]
fn mod_power_of_2_sqrt_fail_1() {
    Natural::from(16u32).mod_power_of_2_sqrt(4);
}

#[test]
#[should_panic]
fn mod_power_of_2_sqrt_fail_2() {
    (&Natural::ONE).mod_power_of_2_sqrt(0);
}

#[test]
fn mod_power_of_2_sqrt_properties() {
    natural_unsigned_pair_gen_var_11().test_properties(|(x, pow)| {
        let root = (&x).mod_power_of_2_sqrt(pow);
        assert_eq!(x.clone().mod_power_of_2_sqrt(pow), root);
        // The Limb path, taken for powers up to Limb::WIDTH, agrees with the general path.
        assert_eq!(mod_power_of_2_sqrt_natural(&x, pow), root);
        // A root exists exactly when x is 0 or 4 ^ w times a number that is 1 mod 8.
        let is_square = x
            .trailing_zeros()
            .is_none_or(|v| v.even() && (&x >> v).mod_power_of_2(3) == 1u32);
        assert_eq!(root.is_some(), is_square);
        if let Some(r) = root {
            // The root is reduced, squares to x, and is no greater than its negation.
            assert!(r.significant_bits() <= pow);
            assert_eq!((&r).mod_power_of_2_square(pow), x);
            assert!(r <= (&r).mod_power_of_2_neg(pow));
        }
        // Multiplying x by 4 and the modulus by 4 doubles the least root.
        assert_eq!(
            (&x << 2u32).mod_power_of_2_sqrt(pow + 2),
            (&x).mod_power_of_2_sqrt(pow).map(|r| r << 1u32)
        );
    });

    natural_unsigned_pair_gen_var_11().test_properties(|(y, pow)| {
        // A square has a root, no greater than either of the roots it was formed from.
        let r = (&y)
            .mod_power_of_2_square(pow)
            .mod_power_of_2_sqrt(pow)
            .unwrap();
        assert!(r <= y);
        assert!(r <= (&y).mod_power_of_2_neg(pow));
    });

    natural_unsigned_pair_gen_var_15().test_properties(|(x, pow)| {
        let root = mod_power_of_2_sqrt_naive(&x, pow);
        assert_eq!((&x).mod_power_of_2_sqrt(pow), root);
        assert_eq!(mod_power_of_2_sqrt_natural(&x, pow), root);
    });

    unsigned_pair_gen_var_17::<Limb>().test_properties(|(x, pow)| {
        // The general path finds the same root as the Limb implementation.
        assert_eq!(
            mod_power_of_2_sqrt_natural(&Natural::from(x), pow),
            x.mod_power_of_2_sqrt(pow).map(Natural::from)
        );
    });

    assert_eq!(Natural::ZERO.mod_power_of_2_sqrt(0), Some(Natural::ZERO));
}

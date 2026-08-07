// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    BinomialCoefficient, CheckedRisingFactorial, Factorial, RisingFactorial,
};
use malachite_base::num::basic::traits::One;
use malachite_base::test_util::generators::unsigned_pair_gen_var_2;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_4;
use std::str::FromStr;

#[test]
fn test_rising_factorial() {
    let test = |s, n, out| {
        let x = Natural::from_str(s).unwrap();
        assert_eq!(x.clone().rising_factorial(n).to_string(), out);
        assert_eq!((&x).rising_factorial(n).to_string(), out);
    };
    // - n == 0
    test("0", 0, "1");
    test("5", 0, "1");
    // - n == 1
    test("5", 1, "5");
    test(
        "98765432123456789012345678990",
        1,
        "98765432123456789012345678990",
    );
    // - a zero base
    test("0", 5, "0");
    // - a single packed batch: the whole product fits in one limb
    test("1", 5, "120");
    test("3", 4, "360");
    test("2", 19, "2432902008176640000");
    // - several packed batches within one packed range
    test("2", 25, "403291461126605635584000000");
    // - a multi-limb base takes the splitting path down to single factors
    test(
        "98765432123456789012345678990",
        3,
        "963418329379931135941713174248942390884436947936894731379426980611626734093608547317280",
    );
    // - 40 factors: the packed path's batch loop, near the 60-factor bound
    test(
        "5",
        40,
        "110761315616185365335151075458942328763318272000000000",
    );
    // - a single-limb base so near the top that some factors overflow the limb: the word-fit guard
    //   falls through to splitting, unreachable in FLINT, whose small-operand bound leaves headroom
    test(
        "18446744073709551614",
        5,
        "2135987035920910082395021706169552114571319013679719366127862660706181393700202559776936\
        752578560",
    );
}

#[test]
fn rising_factorial_properties() {
    natural_unsigned_pair_gen_var_4::<u64>().test_properties(|(x, n)| {
        let rf = (&x).rising_factorial(n);
        assert_eq!(x.clone().rising_factorial(n), rf);
        // the identity x^(n) = binomial(x + n - 1, n) * n!
        if n != 0 && x != 0u32 {
            assert_eq!(
                Natural::binomial_coefficient(&x + Natural::from(n - 1), Natural::from(n))
                    * Natural::factorial(n),
                rf
            );
        }
        // the recurrence x^(n + 1) = x^(n) * (x + n)
        assert_eq!((&x).rising_factorial(n + 1), &rf * (&x + Natural::from(n)));
        assert_eq!((&x).rising_factorial(0), Natural::ONE);
        assert_eq!((&x).rising_factorial(1), x);
    });

    unsigned_pair_gen_var_2::<Limb, u64>().test_properties(|(x, n)| {
        // agreement with the word-sized form
        if let Some(rf) = x.checked_rising_factorial(n) {
            assert_eq!(Natural::from(x).rising_factorial(n), Natural::from(rf));
        }
    });
}

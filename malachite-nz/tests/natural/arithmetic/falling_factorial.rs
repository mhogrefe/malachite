// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.

use malachite_base::num::arithmetic::traits::{
    BinomialCoefficient, CheckedFallingFactorial, Factorial, FallingFactorial, RisingFactorial,
};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::test_util::generators::unsigned_pair_gen_var_2;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::natural_unsigned_pair_gen_var_4;
use malachite_nz::test_util::natural::arithmetic::falling_factorial::falling_factorial_naive;
use std::str::FromStr;

#[test]
fn test_falling_factorial() {
    let test = |s, n, out| {
        let x = Natural::from_str(s).unwrap();
        assert_eq!(x.clone().falling_factorial(n).to_string(), out);
        assert_eq!((&x).falling_factorial(n).to_string(), out);
        assert_eq!(falling_factorial_naive(&x, n).to_string(), out);
    };
    // - n == 0
    test("0", 0, "1");
    test("5", 0, "1");
    // - n <= x
    test("5", 1, "5");
    test("6", 4, "360");
    test("6", 6, "720");
    test("30", 25, "2210440498434925488635904000000");
    test(
        "98765432123456789012345678990",
        3,
        "963418329379931135941713174190414727389238046643799167034625875817150282035263782796680",
    );
    test(
        "18446744073709551615",
        5,
        "2135987035920910080658140367609809183782693394734202016997470406253051756283272123887756\
        645498760",
    );
    // - n > x
    test("0", 1, "0");
    test("3", 4, "0");
    test("6", 7, "0");
}

#[test]
fn falling_factorial_properties() {
    natural_unsigned_pair_gen_var_4::<u64>().test_properties(|(x, n)| {
        let ff = (&x).falling_factorial(n);
        assert_eq!(x.clone().falling_factorial(n), ff);
        assert_eq!(falling_factorial_naive(&x, n), ff);
        assert_eq!(Integer::from(&x).falling_factorial(n), Integer::from(&ff));
        if x >= n {
            // the identity x^(n) = binomial(x, n) * n!
            assert_eq!(
                Natural::binomial_coefficient(x.clone(), Natural::from(n)) * Natural::factorial(n),
                ff
            );
            // the falling factorial of x is the rising factorial of x - n + 1
            if n != 0 {
                assert_eq!((&x - Natural::from(n - 1)).rising_factorial(n), ff);
            }
        } else {
            assert_eq!(ff, Natural::ZERO);
        }
        // the recurrence x^(n + 1) = x^(n) * (x - n), where a negative x - n means x < n
        assert_eq!(
            Integer::from((&x).falling_factorial(n + 1)),
            Integer::from(&ff) * (Integer::from(&x) - Integer::from(n))
        );
        assert_eq!((&x).falling_factorial(0), Natural::ONE);
        assert_eq!((&x).falling_factorial(1), x);
        assert_eq!(Natural::from(n).falling_factorial(n), Natural::factorial(n));
    });
    unsigned_pair_gen_var_2::<Limb, u64>().test_properties(|(x, n)| {
        // agreement with the word-sized form
        if let Some(ff) = x.checked_falling_factorial(n) {
            assert_eq!(Natural::from(x).falling_factorial(n), Natural::from(ff));
        }
    });
}

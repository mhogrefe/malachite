// Copyright © 2026 Mikhail Hogrefe
//
// Uses code adopted from the FLINT Library.
//
//      Copyright © 2011 Fredrik Johansson
//
// This file is part of Malachite.

use malachite_base::num::arithmetic::traits::{FallingFactorial, Parity, RisingFactorial};
use malachite_base::num::basic::traits::One;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_unsigned_pair_gen_var_2;
use malachite_nz::test_util::integer::arithmetic::falling_factorial::falling_factorial_naive;
use std::str::FromStr;

#[test]
fn test_falling_factorial() {
    let test = |s, n, out| {
        let x = Integer::from_str(s).unwrap();
        assert_eq!(x.clone().falling_factorial(n).to_string(), out);
        assert_eq!((&x).falling_factorial(n).to_string(), out);
        assert_eq!(falling_factorial_naive(&x, n).to_string(), out);
    };
    // - n == 0, whatever the sign
    test("0", 0, "1");
    test("-5", 0, "1");
    // - n == 1 returns the base
    test("-5", 1, "-5");
    // - all factors positive
    test("6", 4, "360");
    // - the factors reach 0
    test("3", 4, "0");
    test("0", 5, "0");
    // - the factors cross 0
    test("2", 5, "0");
    // - all factors negative, odd and even lengths
    test("-2", 3, "-24");
    test("-3", 2, "12");
    test(
        "-1000000000000000",
        3,
        "-1000000000000003000000000000002000000000000000",
    );
    test(
        "-1000000000000000",
        4,
        "1000000000000006000000000000011000000000000006000000000000000",
    );
}

#[test]
fn falling_factorial_properties() {
    integer_unsigned_pair_gen_var_2::<u64>().test_properties(|(x, n)| {
        let ff = (&x).falling_factorial(n);
        assert_eq!(x.clone().falling_factorial(n), ff);
        assert_eq!(falling_factorial_naive(&x, n), ff);
        // the recurrence x^(n + 1) = x^(n) * (x - n)
        assert_eq!((&x).falling_factorial(n + 1), &ff * (&x - Integer::from(n)));
        // reflection: (-x)^(n) = (-1)^n x^(n), with the falling and rising factorials swapped
        let rf_neg = (-&x).rising_factorial(n);
        assert_eq!(if n.even() { rf_neg } else { -rf_neg }, ff);
        if x >= 0u32 {
            // agreement with the Natural form
            assert_eq!(ff, Integer::from(x.unsigned_abs_ref().falling_factorial(n)));
        }
        assert_eq!((&x).falling_factorial(0), Integer::ONE);
        assert_eq!((&x).falling_factorial(1), x);
    });
}

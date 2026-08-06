// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{CheckedRisingFactorial, Parity, RisingFactorial};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_unsigned_pair_gen_var_2;
use std::str::FromStr;

#[test]
fn test_rising_factorial() {
    let test = |s, n, out| {
        let x = Integer::from_str(s).unwrap();
        assert_eq!(x.clone().rising_factorial(n).to_string(), out);
        assert_eq!((&x).rising_factorial(n).to_string(), out);
    };
    // - n == 0, whatever the sign
    test("0", 0, "1");
    test("-5", 0, "1");
    // - n == 1 returns the base, negative bases included
    test("-5", 1, "-5");
    // - a zero base
    test("0", 5, "0");
    // - a positive base delegates to the Natural form
    test("3", 4, "360");
    // - an all-negative factor sequence, odd length: negative result
    test("-5", 3, "-60");
    // - an all-negative factor sequence, even length: positive result
    test("-4", 2, "12");
    // - a factor sequence that reaches zero exactly
    test("-3", 4, "0");
    // - a factor sequence that crosses zero
    test("-2", 5, "0");
    // - a large negative span: |x| <= n - 1 must be checked without forming the product
    test("-100", 101, "0");
    // - multi-limb negative bases, both parities
    test(
        "-1000000000000000",
        3,
        "-999999999999997000000000000002000000000000000",
    );
    test(
        "-1000000000000000",
        4,
        "999999999999994000000000000010999999999999994000000000000000",
    );
}

#[test]
fn rising_factorial_properties() {
    integer_unsigned_pair_gen_var_2::<u64>().test_properties(|(x, n)| {
        let rf = (&x).rising_factorial(n);
        assert_eq!(x.clone().rising_factorial(n), rf);
        assert_eq!((&x).rising_factorial(0), Integer::ONE);
        assert_eq!((&x).rising_factorial(1), x);
        // the recurrence x^(n + 1) = x^(n) * (x + n)
        assert_eq!((&x).rising_factorial(n + 1), &rf * (&x + Integer::from(n)));
        if x > 0u32 {
            // agreement with the Natural form
            assert_eq!(rf, Integer::from(x.unsigned_abs_ref().rising_factorial(n)));
            if n != 0 {
                assert!(rf > 0u32);
            }
        } else if x < 0u32 && n != 0 {
            if *x.unsigned_abs_ref() < n {
                // the factor sequence reaches or crosses zero
                assert_eq!(rf, Integer::ZERO);
            } else {
                // all factors negative: the sign is the parity of n
                assert_ne!(rf, Integer::ZERO);
                assert_eq!(rf < 0u32, n.odd());
            }
        }
        // agreement with the word-sized signed form
        if let Ok(sx) = i64::try_from(&x)
            && let Some(srf) = sx.checked_rising_factorial(n)
        {
            assert_eq!(rf, Integer::from(srf));
        }
    });
}

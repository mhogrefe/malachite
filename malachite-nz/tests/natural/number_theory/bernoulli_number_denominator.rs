// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::Parity;
use malachite_base::num::modular::traits::DivisibleBy;
use malachite_base::oracle_row;
use malachite_base::test_util::generators::unsigned_gen_var_5;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::natural::number_theory::bernoulli_number_denominator::*;

#[test]
fn test_bernoulli_number_denominator() {
    let test = |n, out| {
        oracle_row!(
            "flint",
            "arith_bernoulli_number_denom",
            "bernoulli_number_denominator({n}) = {out}"
        );

        assert_eq!(Natural::bernoulli_number_denominator(n).to_string(), out);
    };
    // - n <= BERNOULLI_DENOMINATOR_MAX_SMALL
    test(0, "1");
    // - n.odd()
    // - n == 1
    test(1, "2");
    test(2, "6");
    // - n != 1
    test(3, "1");
    test(4, "30");
    test(5, "1");
    test(6, "42");
    test(12, "2730");
    test(176, "1043970");
    test(178, "1074");
    test(179, "1");
    // - n > BERNOULLI_DENOMINATOR_MAX_SMALL
    // - n.divisible_by(p - 1)
    // - !n.divisible_by(p - 1)
    test(180, "7225713885390");
    test(182, "6");
    test(1000, "342999030");
    test(1001, "1");
    test(10000, "2338224387510");
}

#[test]
fn test_bernoulli_number_denominator_table_is_correct() {
    for n in 0..=180 {
        assert_eq!(
            Natural::bernoulli_number_denominator(n),
            bernoulli_number_denominator_naive(n),
            "{n}"
        );
    }
}

#[test]
fn bernoulli_number_denominator_properties() {
    unsigned_gen_var_5::<u16>().test_properties(|n| {
        let n = u64::from(n);
        let d = Natural::bernoulli_number_denominator(n);
        assert_eq!(d, bernoulli_number_denominator_naive(n));
        if n.odd() && n != 1 {
            assert_eq!(d, 1u32);
        } else if n >= 2 {
            // 2 and 3 always divide the denominator
            assert!((&d).divisible_by(Natural::from(6u32)));
        }
    });
}

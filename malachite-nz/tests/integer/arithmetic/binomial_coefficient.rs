// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{BinomialCoefficient, NegAssign, Parity};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::signed_pair_gen_var_12;
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;
use malachite_nz::test_util::generators::{integer_gen, integer_gen_var_4, integer_pair_gen_var_7};
use std::str::FromStr;

#[test]
fn test_binomial_coefficient() {
    fn test(n: &str, k: &str, out: &str) {
        let n = Integer::from_str(n).unwrap();
        let k = Integer::from_str(k).unwrap();
        let b = Integer::binomial_coefficient(n.clone(), k.clone());
        assert!(b.is_valid());
        assert_eq!(b.to_string(), out);

        let b_alt = Integer::binomial_coefficient(&n, &k);
        assert!(b_alt.is_valid());
        assert_eq!(b_alt, b);

        assert_eq!(
            rug::Integer::from(&n)
                .binomial(u32::exact_from(&k))
                .to_string(),
            out,
        );
    }
    test("0", "0", "1");
    test("1", "0", "1");
    test("1", "1", "1");
    test("2", "0", "1");
    test("2", "1", "2");
    test("2", "2", "1");
    test("3", "0", "1");
    test("3", "1", "3");
    test("3", "2", "3");
    test("3", "3", "1");
    test("4", "0", "1");
    test("4", "1", "4");
    test("4", "2", "6");
    test("4", "3", "4");
    test("4", "4", "1");
    test("1", "2", "0");
    test("10", "5", "252");
    test("100", "50", "100891344545564193334812497256");
    test("-1", "0", "1");
    test("-1", "1", "-1");
    test("-2", "0", "1");
    test("-2", "1", "-2");
    test("-2", "2", "3");
    test("-3", "0", "1");
    test("-3", "1", "-3");
    test("-3", "2", "6");
    test("-3", "3", "-10");
    test("-1", "2", "1");
    test("-10", "5", "-2002");
    test("-80", "50", "1828256793482238093393785743858493760");
    test("-128", "1", "-128");
    test("-2", "127", "-128");
}

#[test]
fn binomial_coefficient_properties() {
    integer_pair_gen_var_7().test_properties(|(n, k)| {
        let b = Integer::binomial_coefficient(n.clone(), k.clone());
        assert!(b.is_valid());

        let b_alt = Integer::binomial_coefficient(&n, &k);
        assert!(b_alt.is_valid());
        assert_eq!(b, b_alt);

        assert_eq!(
            Integer::from(&rug::Integer::from(&n).binomial(u32::exact_from(&k))),
            b
        );
        assert_eq!(b == 0, n >= 0 && n < k);
        if n >= k {
            assert_eq!(Integer::binomial_coefficient(&n, &(&n - &k)), b);
        }
        if k != 0u32 {
            let c = Integer::binomial_coefficient(&(&n - Integer::ONE), &k);
            assert_eq!(
                c + Integer::binomial_coefficient(&n - Integer::ONE, &k - Integer::ONE),
                b
            );
        }
        let mut b_alt = Integer::binomial_coefficient(&((&n - Integer::ONE) + &k), &k);
        if k.odd() {
            b_alt.neg_assign();
        }
        assert_eq!(Integer::binomial_coefficient(-n, k), b_alt);
    });

    integer_gen().test_properties(|n| {
        assert_eq!(Integer::binomial_coefficient(&n, &Integer::ONE), n);
        assert_eq!(Integer::binomial_coefficient(n, Integer::ZERO), 1);
    });

    integer_gen_var_4().test_properties(|n| {
        assert_eq!(Integer::binomial_coefficient(&n, &n), 1u32);
        if n != 0 {
            assert_eq!(Integer::binomial_coefficient(&n, &(&n - Integer::ONE)), n);
            assert_eq!(Integer::binomial_coefficient(Integer::ZERO, n), 0);
        }
    });

    signed_pair_gen_var_12::<SignedLimb>().test_properties(|(n, k)| {
        assert_eq!(
            Integer::binomial_coefficient(Integer::from(n), Integer::from(k)),
            SignedLimb::binomial_coefficient(n, k)
        );
    });
}

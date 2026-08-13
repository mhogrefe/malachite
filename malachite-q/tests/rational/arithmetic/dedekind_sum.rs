// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Gcd, UnsignedAbs};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::{integer_gen, integer_pair_gen};
use malachite_q::Rational;
use malachite_q::test_util::rational::arithmetic::dedekind_sum::dedekind_sum_naive;
use std::str::FromStr;

#[test]
fn test_dedekind_sum() {
    let test = |h, k, out| {
        let h = Integer::from_str(h).unwrap();
        let k = Integer::from_str(k).unwrap();
        assert_eq!(Rational::dedekind_sum(&h, &k).to_string(), out);
    };
    // - the trivial cases: k at most 2 (including negative k and zero) or h zero
    test("5", "0", "0");
    test("5", "1", "0");
    test("5", "2", "0");
    test("5", "-7", "0");
    test("0", "9", "0");
    // - small sums, checked against the definition
    test("1", "3", "1/18");
    test("1", "5", "1/5");
    test("2", "5", "0");
    test("3", "7", "-1/14");
    // - h negative, and h exceeding k: only the residue matters
    test("-1", "3", "-1/18");
    test("7", "4", "-1/8");
    test("22", "7", "5/14");
    // - a word-path value with a longer quotient chain
    test("100", "9973", "81147/9973");
    // - h a multiple of k, so the quotient loop never runs
    test("9", "9", "0");
    // - k too large for the word path, even quotient count
    test(
        "99991",
        "1180591620717411303425",
        "1161601689908895222680203939888798721/1180591620717411303425",
    );
    // - k too large for the word path, odd quotient count, engaging the reciprocity constant
    test(
        "5",
        "1180591620717411303427",
        "3573837371559394734193223249882220236905/181629480110370969758",
    );
    // - h a multiple of a large k
    test("2361183241434822606854", "1180591620717411303427", "0");
}

#[test]
fn dedekind_sum_properties() {
    integer_pair_gen().test_properties(|(h, k)| {
        let s = Rational::dedekind_sum(&h, &k);
        assert!(s.is_valid());
        // only the residue of h matters
        if k > 2u32 {
            assert_eq!(Rational::dedekind_sum(&(&h + &k), &k), s);
            // negating h negates the sum, unless the sum is forced to zero by h ~ 0 mod k
            let neg = Rational::dedekind_sum(&(-&h), &k);
            if &h % &k != 0u32 {
                assert_eq!(neg, -&s);
            }
        }
        // cross-check against the definition where the definition is affordable
        if k > 2u32 && k < 300u32 && h != 0u32 {
            assert_eq!(dedekind_sum_naive(&h, &k), s);
        }
    });
    // Dedekind reciprocity: for coprime positive h < k, s(h, k) + s(k, h) = -1/4 + (h/k + k/h +
    // 1/(hk))/12.
    integer_pair_gen().test_properties(|(a, b)| {
        let h = a.unsigned_abs();
        let k = b.unsigned_abs();
        if h == 0u32 || k == 0u32 {
            return;
        }
        let g = Integer::from((&h).gcd(&k));
        let h = Integer::from(h) / &g;
        let k = Integer::from(k) / g;
        if h <= 2u32 || k <= 2u32 || h == k {
            return;
        }
        let lhs = Rational::dedekind_sum(&h, &k) + Rational::dedekind_sum(&k, &h);
        let rhs = Rational::from_signeds(-1i32, 4)
            + (Rational::from_integers(h.clone(), k.clone())
                + Rational::from_integers(k.clone(), h.clone())
                + Rational::from(Integer::ONE) / Rational::from(h * k))
                / Rational::from(12u32);
        assert_eq!(lhs, rhs);
    });
    integer_gen().test_properties(|k| {
        assert_eq!(Rational::dedekind_sum(&Integer::ZERO, &k), 0);
    });
}

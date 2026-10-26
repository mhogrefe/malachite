// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{Mod, ModPermanent, Permanent};
use malachite_base::num::basic::traits::Zero;
use malachite_base::num::conversion::traits::{ExactFrom, WrappingFrom};
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_11;
use malachite_base::unsigned_matrix::arithmetic::permanent_over_ring::*;
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_pair_gen_var_3, natural_matrix_natural_pair_gen_var_4,
};
use std::panic::catch_unwind;

// Checks the permanent modulo m of `a` against every algorithm that applies to it, and against the
// permanent over the integers.
fn verify(a: &NaturalMatrix, m: &Natural) -> Natural {
    let p = a.mod_permanent(m);
    assert_eq!(a.clone().mod_permanent(m), p);
    assert_eq!(a.clone().mod_permanent(m.clone()), p);
    assert_eq!(a.mod_permanent(m.clone()), p);
    assert_eq!(a.transpose().mod_permanent(m), p);
    assert!(p < *m);
    let n = usize::exact_from(a.height());
    let xs = a.elements_ref();
    let ring = ModRing::new(m.clone());
    assert_eq!(permanent_structured(&ring, xs.to_vec(), n), p);
    if n <= 8 {
        assert_eq!(permanent_dense(&ring, xs, n), p);
        assert_eq!(
            Natural::exact_from(IntegerMatrix::from(a).permanent().mod_op(Integer::from(m))),
            p
        );
    }
    if (1..=8).contains(&n) {
        assert_eq!(permanent_ryser(&ring, xs, n), p);
        if let Some(inverse) = ring.inverse_power_of_2(u64::wrapping_from(n - 1)) {
            assert_eq!(permanent_glynn(&ring, xs, n, &inverse), p);
        }
    }
    if n <= 6 {
        assert_eq!(permanent_cofactor(&ring, xs, n), p);
    }
    p
}

#[test]
fn test_mod_permanent() {
    let test = |s, m, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        let m = Natural::from_str(m).unwrap();
        assert_eq!(verify(&a, &m).to_string(), out);
    };
    // - m fits in a limb
    test("[]_{0,0}", "7", "1");
    test("[]_{0,0}", "1", "0");
    test("[1, 2; 3, 4]", "7", "3");
    // - m fits in two limbs
    test(
        "[1267650600228229401496703205373, 1, 2; 3, 1267650600228229401496703205373, 4; \
        5, 6, 1267650600228229401496703205373]",
        "170141183460469231731687303715884105727",
        "81129638329604085074504135475118",
    );
    test(
        "[1267650600228229401496703205373, 1, 2, 3, 4; \
        5, 1267650600228229401496703205373, 6, 7, 8; \
        9, 10, 1267650600228229401496703205373, 11, 12; \
        13, 14, 15, 1267650600228229401496703205373, 16; \
        17, 18, 19, 20, 1267650600228229401496703205373]",
        "340282366920938463463374607431768211455",
        "410716259219993146308965995263323292",
    );
    // - m does not fit in two limbs and is odd: Glynn's formula
    test(
        "[1267650600228229401496703205373, 1, 2, 3, 4; \
        5, 1267650600228229401496703205373, 6, 7, 8; \
        9, 10, 1267650600228229401496703205373, 11, 12; \
        13, 14, 15, 1267650600228229401496703205373, 16; \
        17, 18, 19, 20, 1267650600228229401496703205373]",
        "8727963568087712425891397479476727340041449",
        "7716989557485794235866092694572673300914086",
    );
    // - m does not fit in two limbs and is a power of 2
    test(
        "[1267650600228229401496703205373, 1, 2, 3, 4; \
        5, 1267650600228229401496703205373, 6, 7, 8; \
        9, 10, 1267650600228229401496703205373, 11, 12; \
        13, 14, 15, 1267650600228229401496703205373, 16; \
        17, 18, 19, 20, 1267650600228229401496703205373]",
        "1606938044258990275541962092341162602522202993782792835301376",
        "327639509436188627569640903868902556",
    );
    // - m does not fit in two limbs and is a power of 2 times an odd number
    test(
        "[1267650600228229401496703205373, 1, 2, 3, 4; \
        5, 1267650600228229401496703205373, 6, 7, 8; \
        9, 10, 1267650600228229401496703205373, 11, 12; \
        13, 14, 15, 1267650600228229401496703205373, 16; \
        17, 18, 19, 20, 1267650600228229401496703205373]",
        "843993600000000000000000000000000000000000",
        "174439625136315520809134932789094870117532",
    );
}

#[test]
fn test_mod_permanent_structured() {
    // - more than 63 rows, connected, with rows of two nonzero elements: contraction
    let n = 70;
    let a = NaturalMatrix::from_owned_elements(
        n,
        n,
        (0..n * n)
            .map(|k| Natural::from((k / n).abs_diff(k % n) <= 1))
            .collect(),
    );
    let m = Natural::from_str("8727963568087712425891397479476727340041449").unwrap();
    assert_eq!(verify(&a, &m).to_string(), "308061521170129");
    let m = Natural::from_str("843993600000000000000000000000000000000000").unwrap();
    assert_eq!(verify(&a, &m).to_string(), "308061521170129");
}

#[test]
fn mod_permanent_fail() {
    // - m = 0
    let a = NaturalMatrix::from_str("[1]").unwrap();
    assert_panic!(a.mod_permanent(Natural::ZERO));
    // - not square
    let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(a.mod_permanent(Natural::from(5u32)));
    // - not reduced
    let a = NaturalMatrix::from_str("[1, 2; 3, 9]").unwrap();
    assert_panic!(a.mod_permanent(Natural::from(7u32)));
}

#[test]
fn mod_permanent_properties() {
    natural_matrix_natural_pair_gen_var_3().test_properties(|(a, m)| {
        if a.height() <= 10 {
            verify(&a, &m);
        }
    });

    natural_matrix_natural_pair_gen_var_4().test_properties(|(a, m)| {
        if a.is_square() && a.height() <= 10 {
            verify(&a, &m);
        }
    });

    unsigned_matrix_unsigned_pair_gen_var_11().test_properties(|(a, m)| {
        if a.height() <= 10 {
            // The permanents of an UnsignedMatrix and of the NaturalMatrix with the same elements
            // agree.
            assert_eq!(
                NaturalMatrix::from(a.clone()).mod_permanent(Natural::from(m)),
                Natural::from(a.mod_permanent(m))
            );
        }
    });
}

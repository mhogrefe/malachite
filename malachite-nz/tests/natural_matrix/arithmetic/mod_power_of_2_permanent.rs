// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{ModPowerOf2, ModPowerOf2Permanent, Permanent};
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::test_util::generators::unsigned_matrix_unsigned_pair_gen_var_4;
use malachite_base::unsigned_matrix::arithmetic::permanent_over_ring::*;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::natural_matrix_unsigned_pair_gen_var_4;
use std::panic::catch_unwind;

// Checks the permanent modulo 2^pow of `a` against every algorithm that applies to it, and against
// the permanent over the integers.
fn verify(a: &NaturalMatrix, pow: u64) -> Natural {
    let p = a.mod_power_of_2_permanent(pow);
    assert_eq!(a.clone().mod_power_of_2_permanent(pow), p);
    assert_eq!(a.transpose().mod_power_of_2_permanent(pow), p);
    let n = usize::exact_from(a.height());
    let xs = a.elements_ref();
    let ring = ModPowerOf2Ring::new(pow);
    assert_eq!(permanent_structured(&ring, xs.to_vec(), n), p);
    if n <= 8 {
        assert_eq!(permanent_dense(&ring, xs, n), p);
        assert_eq!(
            Natural::exact_from(IntegerMatrix::from(a).permanent().mod_power_of_2(pow)),
            p
        );
    }
    if (1..=8).contains(&n) {
        assert_eq!(permanent_ryser(&ring, xs, n), p);
        assert_eq!(permanent_glynn_mod_power_of_2(xs, n, pow), p);
    }
    if n <= 6 {
        assert_eq!(permanent_cofactor(&ring, xs, n), p);
    }
    p
}

#[test]
fn test_mod_power_of_2_permanent() {
    let test = |s, pow, out| {
        let a = NaturalMatrix::from_str(s).unwrap();
        assert_eq!(verify(&a, pow).to_string(), out);
    };
    // - 2^pow fits in a limb
    test("[]_{0,0}", 3, "1");
    test("[1, 2; 3, 4]", 3, "2");
    // - 2^pow fits in two limbs
    test(
        "[1267650600228229401496703205373, 1, 2; 3, 1267650600228229401496703205373, 4; \
        5, 6, 1267650600228229401496703205373]",
        100,
        "1267650600228229401496703205294",
    );
    test(
        "[1267650600228229401496703205373, 1, 2, 3, 4; \
        5, 1267650600228229401496703205373, 6, 7, 8; \
        9, 10, 1267650600228229401496703205373, 11, 12; \
        13, 14, 15, 1267650600228229401496703205373, 16; \
        17, 18, 19, 20, 1267650600228229401496703205373]",
        100,
        "1010844",
    );
    test(
        "[1267650600228229401496703205373, 1, 2, 3, 4; \
        5, 1267650600228229401496703205373, 6, 7, 8; \
        9, 10, 1267650600228229401496703205373, 11, 12; \
        13, 14, 15, 1267650600228229401496703205373, 16; \
        17, 18, 19, 20, 1267650600228229401496703205373]",
        128,
        "327639509436188627569640903868902556",
    );
    test(
        "[3, 1, 4, 1, 5; 9, 2, 6, 5, 3; 5, 8, 9, 7, 9; 3, 2, 3, 8, 4; 6, 2, 6, 4, 3]",
        70,
        "200637",
    );
    // - 2^pow does not fit in two limbs
    test(
        "[1267650600228229401496703205373, 1, 2, 3, 4; \
        5, 1267650600228229401496703205373, 6, 7, 8; \
        9, 10, 1267650600228229401496703205373, 11, 12; \
        13, 14, 15, 1267650600228229401496703205373, 16; \
        17, 18, 19, 20, 1267650600228229401496703205373]",
        200,
        "327639509436188627569640903868902556",
    );
}

#[test]
fn test_mod_power_of_2_permanent_structured() {
    // - more than 63 rows, connected, with rows of two nonzero elements: contraction
    let n = 70;
    let a = NaturalMatrix::from_owned_elements(
        n,
        n,
        (0..n * n)
            .map(|k| Natural::from((k / n).abs_diff(k % n) <= 1))
            .collect(),
    );
    assert_eq!(verify(&a, 100).to_string(), "308061521170129");
    assert_eq!(verify(&a, 200).to_string(), "308061521170129");
}

#[test]
fn mod_power_of_2_permanent_fail() {
    // - not square
    let a = NaturalMatrix::from_str("[1, 2]").unwrap();
    assert_panic!(a.mod_power_of_2_permanent(3));
    // - not reduced
    let a = NaturalMatrix::from_str("[1, 2; 3, 9]").unwrap();
    assert_panic!(a.mod_power_of_2_permanent(3));
}

#[test]
fn mod_power_of_2_permanent_properties() {
    natural_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        if a.is_square() && a.height() <= 10 {
            verify(&a, pow);
        }
    });

    unsigned_matrix_unsigned_pair_gen_var_4().test_properties(|(a, pow)| {
        if a.is_square() && a.height() <= 10 {
            // The permanents of an UnsignedMatrix and of the NaturalMatrix with the same elements
            // agree.
            assert_eq!(
                NaturalMatrix::from(a.clone()).mod_power_of_2_permanent(pow),
                Natural::from(a.mod_power_of_2_permanent(pow))
            );
        }
    });
}

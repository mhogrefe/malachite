// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::strings::ToDebugString;
use malachite_nz::integer::Integer;
use malachite_nz::integer_polynomial::arithmetic::scalar_add_mul::integers_add_mul_scalar_assign;
use malachite_nz::integer_vector::arithmetic::scalar_mul::integers_mul_scalar;
use malachite_nz::test_util::generators::{
    integer_vec_integer_pair_gen, integer_vec_integer_vec_integer_triple_gen,
};
use malachite_nz::test_util::integer_polynomial::arithmetic::scalar_add_mul::*;

fn parse(xs: &[&str]) -> Vec<Integer> {
    xs.iter().map(|x| Integer::from_str(x).unwrap()).collect()
}

#[test]
fn test_integers_add_mul_scalar_assign() {
    let test = |xs: &[&str], ys: &[&str], c, out| {
        let mut xs = parse(xs);
        let ys = parse(ys);
        let c = Integer::from_str(c).unwrap();
        let naive = integers_add_mul_scalar_naive(&xs, &ys, &c);
        integers_add_mul_scalar_assign(&mut xs, &ys, &c);
        assert_eq!(xs.to_debug_string(), out);
        assert_eq!(naive, xs);
    };
    test(&[], &[], "5", "[]");
    test(&[], &["1", "2"], "3", "[3, 6]");
    test(&["1", "2"], &[], "3", "[1, 2]");
    // With a zero scalar nothing changes, and a shorter vector is not extended.
    test(&["1"], &["1", "2", "3"], "0", "[1]");
    // A scalar of 1 adds, and a scalar of -1 subtracts.
    test(&["1", "2"], &["10", "20", "30"], "1", "[11, 22, 30]");
    test(&["1", "2"], &["10", "20", "30"], "-1", "[-9, -18, -30]");
    test(&["1", "2", "3"], &["10"], "-5", "[-49, 2, 3]");
    // Leading elements can cancel, and are not trimmed.
    test(&["1", "-6"], &["0", "2"], "3", "[1, 0]");
    // Elements and scalar of many limbs.
    test(
        &["1", "1000000000000000000000000"],
        &["2", "-1"],
        "1000000000000000000000000",
        "[2000000000000000000000001, 0]",
    );
}

#[test]
fn integers_add_mul_scalar_assign_properties() {
    integer_vec_integer_vec_integer_triple_gen().test_properties(|(xs, ys, c)| {
        let mut zs = xs.clone();
        integers_add_mul_scalar_assign(&mut zs, &ys, &c);
        assert_eq!(integers_add_mul_scalar_naive(&xs, &ys, &c), zs);
        // Adding the product, then the product with the scalar negated, gives `xs` back, padded
        // with zeros to the length of `ys` if it was shorter.
        integers_add_mul_scalar_assign(&mut zs, &ys, &-&c);
        let mut padded = xs.clone();
        if c != 0u32 && padded.len() < ys.len() {
            padded.resize(ys.len(), Integer::ZERO);
        }
        assert_eq!(zs, padded);
        // This is adding the product computed by `integers_mul_scalar`.
        if c != 0u32 {
            let mut zs = xs.clone();
            integers_add_mul_scalar_assign(&mut zs, &integers_mul_scalar(&ys, &c), &Integer::ONE);
            let mut ws = xs.clone();
            integers_add_mul_scalar_assign(&mut ws, &ys, &c);
            assert_eq!(zs, ws);
        }
    });

    integer_vec_integer_pair_gen().test_properties(|(xs, c)| {
        // Adding a multiple of an empty vector changes nothing.
        let mut zs = xs.clone();
        integers_add_mul_scalar_assign(&mut zs, &[], &c);
        assert_eq!(zs, xs);
        // Adding a vector's own negation gives zeros.
        let mut zs = xs.clone();
        integers_add_mul_scalar_assign(&mut zs, &xs, &Integer::NEGATIVE_ONE);
        assert!(zs.iter().all(|z| *z == 0u32));
    });
}

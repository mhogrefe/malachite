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
use malachite_nz::integer_polynomial::arithmetic::scalar_mul::{
    integers_mul_scalar, integers_mul_scalar_assign,
};
use malachite_nz::test_util::generators::{integer_vec_gen, integer_vec_integer_pair_gen};
use malachite_nz::test_util::integer_polynomial::arithmetic::scalar_mul::integers_mul_scalar_naive;

fn parse(xs: &[&str]) -> Vec<Integer> {
    xs.iter().map(|x| Integer::from_str(x).unwrap()).collect()
}

#[test]
fn test_integers_mul_scalar() {
    let test = |xs: &[&str], c, out| {
        let xs = parse(xs);
        let c = Integer::from_str(c).unwrap();
        let ys = integers_mul_scalar(&xs, &c);
        assert_eq!(ys.to_debug_string(), out);
        assert_eq!(integers_mul_scalar_naive(&xs, &c), ys);
        let mut zs = xs;
        integers_mul_scalar_assign(&mut zs, &c);
        assert_eq!(zs, ys);
    };
    test(&[], "5", "[]");
    // Multiplying by 0 gives zeros, which are not trimmed.
    test(&["1", "-2", "3"], "0", "[0, 0, 0]");
    // Multiplying by 1 changes nothing, and multiplying by -1 negates.
    test(&["1", "-2", "3"], "1", "[1, -2, 3]");
    test(&["1", "-2", "3"], "-1", "[-1, 2, -3]");
    test(&["1", "-2", "0", "3"], "4", "[4, -8, 0, 12]");
    // Trailing zeros are kept.
    test(&["-7", "0"], "-3", "[21, 0]");
    // Elements and scalar of many limbs.
    test(
        &["1000000000000000000000", "-1"],
        "-1000000000000",
        "[-1000000000000000000000000000000000, 1000000000000]",
    );
}

#[test]
fn integers_mul_scalar_properties() {
    integer_vec_integer_pair_gen().test_properties(|(xs, c)| {
        let ys = integers_mul_scalar(&xs, &c);
        assert_eq!(ys.len(), xs.len());
        assert_eq!(integers_mul_scalar_naive(&xs, &c), ys);
        let mut zs = xs.clone();
        integers_mul_scalar_assign(&mut zs, &c);
        assert_eq!(zs, ys);
        // Negating the scalar negates every product.
        assert_eq!(
            integers_mul_scalar(&xs, &-&c),
            ys.iter().map(|y| -y).collect::<Vec<_>>()
        );
    });

    integer_vec_gen().test_properties(|xs| {
        assert!(
            integers_mul_scalar(&xs, &Integer::ZERO)
                .iter()
                .all(|y| *y == 0u32)
        );
        assert_eq!(integers_mul_scalar(&xs, &Integer::ONE), xs);
        assert_eq!(
            integers_mul_scalar(&xs, &Integer::NEGATIVE_ONE),
            xs.iter().map(|x| -x).collect::<Vec<_>>()
        );
    });
}

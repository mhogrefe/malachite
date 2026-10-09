// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{L1Norm, UnsignedAbs};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::strings::ToDebugString;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::integer_vector::arithmetic::scalar_mul::{
    integers_mul_scalar, integers_mul_scalar_assign, integers_mul_scalar_to_out,
};
use malachite_nz::test_util::generators::{
    integer_vec_gen, integer_vec_integer_pair_gen, integer_vector_integer_pair_gen,
};
use malachite_nz::test_util::integer_vector::arithmetic::scalar_mul::integers_mul_scalar_naive;

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
        let mut out = vec![Integer::ZERO; xs.len()];
        integers_mul_scalar_to_out(&mut out, &xs, &c);
        assert_eq!(out, ys);
        let mut zs = xs;
        integers_mul_scalar_assign(&mut zs, &c);
        assert_eq!(zs, ys);
    };
    test(&[], "5", "[]");
    // Multiplying by 0 gives zeros, which are not trimmed.
    // - *c == 0
    test(&["1", "-2", "3"], "0", "[0, 0, 0]");
    // Multiplying by 1 changes nothing, and multiplying by -1 negates.
    // - *c == 1
    test(&["1", "-2", "3"], "1", "[1, -2, 3]");
    // - *c == -1
    test(&["1", "-2", "3"], "-1", "[-1, 2, -3]");
    // - *c != 0 && *c != 1 && *c != -1
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
        // Writing to a shorter output gives a prefix.
        let mut out = vec![Integer::ZERO; xs.len() >> 1];
        integers_mul_scalar_to_out(&mut out, &xs, &c);
        assert_eq!(out, &ys[..xs.len() >> 1]);
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

#[test]
fn test_mul_scalar() {
    let test = |s, c, out| {
        let v = IntegerVector::from_str(s).unwrap();
        let c = Integer::from_str(c).unwrap();
        // All four combinations of value and reference, on either side, and in place with both.
        let w = &v * &c;
        assert_eq!(w.to_string(), out);
        assert_eq!(&v * c.clone(), w);
        assert_eq!(v.clone() * &c, w);
        assert_eq!(v.clone() * c.clone(), w);
        assert_eq!(&c * &v, w);
        assert_eq!(&c * v.clone(), w);
        assert_eq!(c.clone() * &v, w);
        assert_eq!(c.clone() * v.clone(), w);
        let mut x = v.clone();
        x *= &c;
        assert_eq!(x, w);
        let mut x = v;
        x *= c;
        assert_eq!(x, w);
    };
    test("()", "5", "()");
    test("(1, -2, 3)", "0", "(0, 0, 0)");
    test("(1, -2, 3)", "1", "(1, -2, 3)");
    test("(1, -2, 3)", "-1", "(-1, 2, -3)");
    test("(1, -2, 3)", "-3", "(-3, 6, -9)");
}

#[test]
fn mul_scalar_properties() {
    integer_vector_integer_pair_gen().test_properties(|(v, c)| {
        let w = &v * &c;
        // The forms agree.
        assert_eq!(&v * c.clone(), w);
        assert_eq!(v.clone() * &c, w);
        assert_eq!(v.clone() * c.clone(), w);
        assert_eq!(&c * &v, w);
        assert_eq!(&c * v.clone(), w);
        assert_eq!(c.clone() * &v, w);
        assert_eq!(c.clone() * v.clone(), w);
        let mut x = v.clone();
        x *= &c;
        assert_eq!(x, w);
        let mut x = v.clone();
        x *= c.clone();
        assert_eq!(x, w);

        // Element by element, this is the scalar product, and the dimension is unchanged.
        assert_eq!(w.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x * &c);
        }
        // Multiplying by 0 gives the zero vector, by 1 changes nothing, and products of scalars act
        // one after the other.
        assert_eq!(&v * Integer::ZERO, IntegerVector::zero(v.dimension()));
        assert_eq!(&v * Integer::ONE, v);
        // Multiplying by -1 negates.
        assert_eq!(&v * Integer::NEGATIVE_ONE, -&v);
        assert_eq!(&v * (&c * &c), &w * &c);
        // It distributes over vector addition.
        assert_eq!((&v + &v) * &c, &w + &w);
        // The l^1 norm scales by the absolute value of the scalar.
        assert_eq!(w.to_l1_norm(), v.to_l1_norm() * (&c).unsigned_abs());
    });
}

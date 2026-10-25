// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::matrix::Matrix;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_nz::integer::Integer;
use malachite_nz::integer_matrix::IntegerMatrix;
use malachite_nz::natural::Natural;
use malachite_nz::natural_matrix::NaturalMatrix;
use malachite_nz::test_util::generators::{
    natural_matrix_natural_matrix_natural_triple_gen_var_7, natural_matrix_natural_pair_gen,
    natural_matrix_pair_gen_var_1,
};

#[test]
fn test_add_mul() {
    let test = |s, t, c, out| {
        let u = NaturalMatrix::from_str(s).unwrap();
        let v = NaturalMatrix::from_str(t).unwrap();
        let c = Natural::from_str(c).unwrap();
        let b = (&u).add_mul(&v, &c);
        assert_eq!(b.to_string(), out);
        assert_eq!(u.clone().add_mul(v.clone(), c.clone()), b);
        assert_eq!(u.clone().add_mul(v.clone(), &c), b);
        assert_eq!(u.clone().add_mul(&v, c.clone()), b);
        assert_eq!(u.clone().add_mul(&v, &c), b);
        let mut x = u.clone();
        x.add_mul_assign(v.clone(), c.clone());
        assert_eq!(x, b);
        let mut x = u.clone();
        x.add_mul_assign(v.clone(), &c);
        assert_eq!(x, b);
        let mut x = u.clone();
        x.add_mul_assign(&v, c.clone());
        assert_eq!(x, b);
        let mut x = u;
        x.add_mul_assign(&v, &c);
        assert_eq!(x, b);
    };
    test("[]_{0,0}", "[]_{0,0}", "5", "[]_{0,0}");
    test("[1, 2, 3]", "[4, 5, 6]", "0", "[1, 2, 3]");
    test("[1, 2, 3]", "[4, 5, 6]", "1", "[5, 7, 9]");
    test("[1, 2, 3]", "[4, 5, 6]", "10", "[41, 52, 63]");
    test(
        "[1, 0]",
        "[18446744073709551615, 1]",
        "18446744073709551617",
        "[340282366920938463463374607431768211456, 18446744073709551617]",
    );
}

#[test]
#[should_panic]
fn add_mul_fail() {
    NaturalMatrix::from_str("[1, 2]")
        .unwrap()
        .add_mul(NaturalMatrix::from_str("[1]").unwrap(), Natural::from(3u32));
}

#[test]
#[should_panic]
fn add_mul_ref_fail() {
    (&NaturalMatrix::from_str("[1, 2]").unwrap()).add_mul(
        &NaturalMatrix::from_str("[1]").unwrap(),
        &Natural::from(3u32),
    );
}

#[test]
#[should_panic]
fn add_mul_assign_fail() {
    let mut v = NaturalMatrix::from_str("[1, 2]").unwrap();
    v.add_mul_assign(NaturalMatrix::from_str("[1]").unwrap(), Natural::from(3u32));
}

#[test]
#[should_panic]
fn add_mul_assign_ref_fail() {
    let mut v = NaturalMatrix::from_str("[1, 2]").unwrap();
    v.add_mul_assign(
        &NaturalMatrix::from_str("[1]").unwrap(),
        &Natural::from(3u32),
    );
}

#[test]
fn add_mul_properties() {
    natural_matrix_natural_matrix_natural_triple_gen_var_7().test_properties(|(u, v, c)| {
        let b = (&u).add_mul(&v, &c);
        // The forms agree.
        assert_eq!(u.clone().add_mul(v.clone(), c.clone()), b);
        assert_eq!(u.clone().add_mul(v.clone(), &c), b);
        assert_eq!(u.clone().add_mul(&v, c.clone()), b);
        assert_eq!(u.clone().add_mul(&v, &c), b);
        let mut x = u.clone();
        x.add_mul_assign(v.clone(), c.clone());
        assert_eq!(x, b);
        let mut x = u.clone();
        x.add_mul_assign(v.clone(), &c);
        assert_eq!(x, b);
        let mut x = u.clone();
        x.add_mul_assign(&v, c.clone());
        assert_eq!(x, b);
        let mut x = u.clone();
        x.add_mul_assign(&v, &c);
        assert_eq!(x, b);

        // It is the sum of the matrix and the scalar product, element by element the scalar
        // add_mul, and the shape is unchanged.
        assert_eq!(b, &u + &v * &c);
        assert_eq!((b.height(), b.width()), (u.height(), u.width()));
        for ((x, y), z) in u
            .elements_ref()
            .iter()
            .zip(v.elements_ref())
            .zip(b.elements_ref())
        {
            assert_eq!(*z, x.add_mul(y, &c));
        }
        // Swapping the roles of the matrix and the scalar product changes nothing.
        assert_eq!((&v * &c).add_mul(&u, &Natural::ONE), b);
        // Adding two multiples one after the other adds their scalars.
        let d = &c + Natural::ONE;
        assert_eq!((&b).add_mul(&v, &Natural::ONE), (&u).add_mul(&v, &d));

        // As IntegerMatrix values and an Integer, the result is the same.
        assert_eq!(
            IntegerMatrix::from(b),
            IntegerMatrix::from(u) + IntegerMatrix::from(v) * Integer::from(c)
        );
    });

    natural_matrix_pair_gen_var_1().test_properties(|(u, v)| {
        // Multiplying by 0 changes nothing, and by 1 adds the matrices.
        assert_eq!((&u).add_mul(&v, &Natural::ZERO), u);
        assert_eq!((&u).add_mul(&v, &Natural::ONE), &u + &v);
        // Adding a multiple of the zero matrix changes nothing.
        assert_eq!(
            (&u).add_mul(
                &NaturalMatrix::zero(u.height(), u.width()),
                &Natural::from(3u32)
            ),
            u
        );
    });

    natural_matrix_natural_pair_gen().test_properties(|(v, c)| {
        // Adding to the zero matrix gives the scalar product.
        assert_eq!(
            NaturalMatrix::zero(v.height(), v.width()).add_mul(&v, &c),
            &v * &c
        );
    });
}

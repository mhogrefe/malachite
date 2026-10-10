// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign, SubMul};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::{
    integer_vector_integer_pair_gen, integer_vector_integer_vector_integer_triple_gen_var_1,
    integer_vector_pair_gen_var_1, natural_vector_natural_vector_natural_triple_gen_var_2,
};

#[test]
fn test_add_mul() {
    let test = |s, t, c, out| {
        let u = IntegerVector::from_str(s).unwrap();
        let v = IntegerVector::from_str(t).unwrap();
        let c = Integer::from_str(c).unwrap();
        let w = (&u).add_mul(&v, &c);
        assert_eq!(w.to_string(), out);
        assert_eq!(u.clone().add_mul(v.clone(), c.clone()), w);
        assert_eq!(u.clone().add_mul(v.clone(), &c), w);
        assert_eq!(u.clone().add_mul(&v, c.clone()), w);
        assert_eq!(u.clone().add_mul(&v, &c), w);
        let mut x = u.clone();
        x.add_mul_assign(v.clone(), c.clone());
        assert_eq!(x, w);
        let mut x = u.clone();
        x.add_mul_assign(v.clone(), &c);
        assert_eq!(x, w);
        let mut x = u.clone();
        x.add_mul_assign(&v, c.clone());
        assert_eq!(x, w);
        let mut x = u;
        x.add_mul_assign(&v, &c);
        assert_eq!(x, w);
    };
    test("()", "()", "5", "()");
    test("(1, -2, 3)", "(4, 5, -6)", "0", "(1, -2, 3)");
    test("(1, -2, 3)", "(4, 5, -6)", "1", "(5, 3, -3)");
    test("(1, -2, 3)", "(4, 5, -6)", "-1", "(-3, -7, 9)");
    test("(1, -2, 3)", "(4, 5, -6)", "10", "(41, 48, -57)");
    test("(1, -2, 3)", "(4, 5, -6)", "-10", "(-39, -52, 63)");
    test(
        "(1, 0)",
        "(18446744073709551615, -1)",
        "18446744073709551617",
        "(340282366920938463463374607431768211456, -18446744073709551617)",
    );
}

#[test]
#[should_panic]
fn add_mul_fail() {
    IntegerVector::from_str("(1, 2)")
        .unwrap()
        .add_mul(IntegerVector::from_str("(1)").unwrap(), Integer::from(3));
}

#[test]
#[should_panic]
fn add_mul_ref_fail() {
    (&IntegerVector::from_str("(1, 2)").unwrap())
        .add_mul(&IntegerVector::from_str("(1)").unwrap(), &Integer::from(3));
}

#[test]
#[should_panic]
fn add_mul_assign_fail() {
    let mut v = IntegerVector::from_str("(1, 2)").unwrap();
    v.add_mul_assign(IntegerVector::from_str("(1)").unwrap(), Integer::from(3));
}

#[test]
#[should_panic]
fn add_mul_assign_ref_fail() {
    let mut v = IntegerVector::from_str("(1, 2)").unwrap();
    v.add_mul_assign(&IntegerVector::from_str("(1)").unwrap(), &Integer::from(3));
}

#[test]
fn add_mul_properties() {
    integer_vector_integer_vector_integer_triple_gen_var_1().test_properties(|(u, v, c)| {
        let w = (&u).add_mul(&v, &c);
        // The forms agree.
        assert_eq!(u.clone().add_mul(v.clone(), c.clone()), w);
        assert_eq!(u.clone().add_mul(v.clone(), &c), w);
        assert_eq!(u.clone().add_mul(&v, c.clone()), w);
        assert_eq!(u.clone().add_mul(&v, &c), w);
        let mut x = u.clone();
        x.add_mul_assign(v.clone(), c.clone());
        assert_eq!(x, w);
        let mut x = u.clone();
        x.add_mul_assign(v.clone(), &c);
        assert_eq!(x, w);
        let mut x = u.clone();
        x.add_mul_assign(&v, c.clone());
        assert_eq!(x, w);
        let mut x = u.clone();
        x.add_mul_assign(&v, &c);
        assert_eq!(x, w);

        // It is the vector plus the scalar product, element by element the scalar add_mul, and the
        // dimension is unchanged.
        assert_eq!(w, &u + &v * &c);
        assert_eq!(w.dimension(), u.dimension());
        for ((x, y), z) in u.elements.iter().zip(&v.elements).zip(&w.elements) {
            assert_eq!(*z, x.add_mul(y, &c));
        }
        // Negating the scalar gives the opposite operation, and the operation can be undone.
        assert_eq!((&u).sub_mul(&v, &-&c), w);
        assert_eq!((&w).sub_mul(&v, &c), u);
        // Multiplying by c and then by 1 is multiplying by c + 1.
        let d = &c + Integer::ONE;
        assert_eq!((&w).add_mul(&v, &Integer::ONE), (&u).add_mul(&v, &d));
        // Negating everything negates the result.
        assert_eq!((-&u).add_mul(-&v, &c), -&w);
    });

    integer_vector_pair_gen_var_1().test_properties(|(u, v)| {
        // Multiplying by 0 changes nothing, by 1 adds the vectors, and by -1 subtracts them.
        assert_eq!((&u).add_mul(&v, &Integer::ZERO), u);
        assert_eq!((&u).add_mul(&v, &Integer::ONE), &u + &v);
        assert_eq!((&u).add_mul(&v, &Integer::NEGATIVE_ONE), &u - &v);
        // Adding a multiple of the zero vector changes nothing.
        assert_eq!(
            (&u).add_mul(&IntegerVector::zero(u.dimension()), &Integer::from(3)),
            u
        );
    });

    integer_vector_integer_pair_gen().test_properties(|(v, c)| {
        // Starting from the zero vector gives the scalar product.
        assert_eq!(IntegerVector::zero(v.dimension()).add_mul(&v, &c), &v * &c);
    });

    natural_vector_natural_vector_natural_triple_gen_var_2().test_properties(|(u, v, c)| {
        // As NaturalVectors and a Natural, the result is the same.
        assert_eq!(
            IntegerVector::from((&u).add_mul(&v, &c)),
            IntegerVector::from(u).add_mul(IntegerVector::from(v), Integer::from(c))
        );
    });
}

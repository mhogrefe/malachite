// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{AddMul, SubMul, SubMulAssign};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::vector::Vector;
use malachite_nz::test_util::generators::integer_vector_integer_vector_integer_triple_gen_var_1;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{
    rational_vector_pair_gen_var_1, rational_vector_rational_pair_gen,
    rational_vector_rational_vector_rational_triple_gen_var_1,
};

#[test]
fn test_sub_mul() {
    let test = |s, t, c, out| {
        let u = RationalVector::from_str(s).unwrap();
        let v = RationalVector::from_str(t).unwrap();
        let c = Rational::from_str(c).unwrap();
        let w = (&u).sub_mul(&v, &c);
        assert_eq!(w.to_string(), out);
        assert_eq!(u.clone().sub_mul(v.clone(), c.clone()), w);
        assert_eq!(u.clone().sub_mul(v.clone(), &c), w);
        assert_eq!(u.clone().sub_mul(&v, c.clone()), w);
        assert_eq!(u.clone().sub_mul(&v, &c), w);
        let mut x = u.clone();
        x.sub_mul_assign(v.clone(), c.clone());
        assert_eq!(x, w);
        let mut x = u.clone();
        x.sub_mul_assign(v.clone(), &c);
        assert_eq!(x, w);
        let mut x = u.clone();
        x.sub_mul_assign(&v, c.clone());
        assert_eq!(x, w);
        let mut x = u;
        x.sub_mul_assign(&v, &c);
        assert_eq!(x, w);
    };
    test("()", "()", "5", "()");
    test("(1/2, -2, 3)", "(4, 1/3, -6)", "0", "(1/2, -2, 3)");
    test("(1/2, -2, 3)", "(4, 1/3, -6)", "1", "(-7/2, -7/3, 9)");
    test("(1/2, -2, 3)", "(4, 1/3, -6)", "-1", "(9/2, -5/3, -3)");
    test("(1/2, -2, 3)", "(4, 1/3, -6)", "3/2", "(-11/2, -5/2, 12)");
    test("(1/2, -2, 3)", "(4, 1/3, -6)", "-3/2", "(13/2, -3/2, -6)");
    test(
        "(1/3, 0)",
        "(18446744073709551615/5, -1)",
        "18446744073709551617/7",
        "(-204169420152563078078024764459060926866/21, 18446744073709551617/7)",
    );
}

#[test]
#[should_panic]
fn sub_mul_fail() {
    RationalVector::from_str("(1, 2)")
        .unwrap()
        .sub_mul(RationalVector::from_str("(1)").unwrap(), Rational::from(3));
}

#[test]
#[should_panic]
fn sub_mul_ref_fail() {
    (&RationalVector::from_str("(1, 2)").unwrap()).sub_mul(
        &RationalVector::from_str("(1)").unwrap(),
        &Rational::from(3),
    );
}

#[test]
#[should_panic]
fn sub_mul_assign_fail() {
    let mut v = RationalVector::from_str("(1, 2)").unwrap();
    v.sub_mul_assign(RationalVector::from_str("(1)").unwrap(), Rational::from(3));
}

#[test]
#[should_panic]
fn sub_mul_assign_ref_fail() {
    let mut v = RationalVector::from_str("(1, 2)").unwrap();
    v.sub_mul_assign(
        &RationalVector::from_str("(1)").unwrap(),
        &Rational::from(3),
    );
}

#[test]
fn sub_mul_properties() {
    rational_vector_rational_vector_rational_triple_gen_var_1().test_properties(|(u, v, c)| {
        let w = (&u).sub_mul(&v, &c);
        // The forms agree.
        assert_eq!(u.clone().sub_mul(v.clone(), c.clone()), w);
        assert_eq!(u.clone().sub_mul(v.clone(), &c), w);
        assert_eq!(u.clone().sub_mul(&v, c.clone()), w);
        assert_eq!(u.clone().sub_mul(&v, &c), w);
        let mut x = u.clone();
        x.sub_mul_assign(v.clone(), c.clone());
        assert_eq!(x, w);
        let mut x = u.clone();
        x.sub_mul_assign(v.clone(), &c);
        assert_eq!(x, w);
        let mut x = u.clone();
        x.sub_mul_assign(&v, c.clone());
        assert_eq!(x, w);
        let mut x = u.clone();
        x.sub_mul_assign(&v, &c);
        assert_eq!(x, w);

        // It is the vector minus the scalar product, element by element the scalar sub_mul, and the
        // dimension is unchanged.
        assert_eq!(w, &u - &v * &c);
        assert_eq!(w.dimension(), u.dimension());
        for ((x, y), z) in u.elements.iter().zip(&v.elements).zip(&w.elements) {
            assert_eq!(*z, x.sub_mul(y, &c));
        }
        // Negating the scalar gives the opposite operation, and the operation can be undone.
        assert_eq!((&u).add_mul(&v, &-&c), w);
        assert_eq!((&w).add_mul(&v, &c), u);
        // Multiplying by c and then by 1 is multiplying by c + 1.
        let d = &c + Rational::ONE;
        assert_eq!((&w).sub_mul(&v, &Rational::ONE), (&u).sub_mul(&v, &d));
        // Negating everything negates the result.
        assert_eq!((-&u).sub_mul(-&v, &c), -&w);
    });

    rational_vector_pair_gen_var_1().test_properties(|(u, v)| {
        // Multiplying by 0 changes nothing, by 1 subtracts the vectors, and by -1 adds them.
        assert_eq!((&u).sub_mul(&v, &Rational::ZERO), u);
        assert_eq!((&u).sub_mul(&v, &Rational::ONE), &u - &v);
        assert_eq!((&u).sub_mul(&v, &Rational::NEGATIVE_ONE), &u + &v);
        // Subtracting a multiple of the zero vector changes nothing.
        assert_eq!(
            (&u).sub_mul(&RationalVector::zero(u.dimension()), &Rational::from(3)),
            u
        );
    });

    rational_vector_rational_pair_gen().test_properties(|(v, c)| {
        // Starting from the zero vector gives the scalar product, negated.
        assert_eq!(
            RationalVector::zero(v.dimension()).sub_mul(&v, &c),
            -(&v * &c)
        );
    });

    integer_vector_integer_vector_integer_triple_gen_var_1().test_properties(|(u, v, c)| {
        // As RationalVectors and a Rational, the result is the same.
        assert_eq!(
            RationalVector::from((&u).sub_mul(&v, &c)),
            RationalVector::from(u).sub_mul(RationalVector::from(v), Rational::from(c))
        );
    });
}

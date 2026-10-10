// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulAssign};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_vector_natural_pair_gen, natural_vector_natural_vector_natural_triple_gen_var_2,
    natural_vector_pair_gen_var_1,
};

#[test]
fn test_add_mul() {
    let test = |s, t, c, out| {
        let u = NaturalVector::from_str(s).unwrap();
        let v = NaturalVector::from_str(t).unwrap();
        let c = Natural::from_str(c).unwrap();
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
    test("(1, 2, 3)", "(4, 5, 6)", "0", "(1, 2, 3)");
    test("(1, 2, 3)", "(4, 5, 6)", "1", "(5, 7, 9)");
    test("(1, 2, 3)", "(4, 5, 6)", "10", "(41, 52, 63)");
    test(
        "(1, 0)",
        "(18446744073709551615, 1)",
        "18446744073709551617",
        "(340282366920938463463374607431768211456, 18446744073709551617)",
    );
}

#[test]
#[should_panic]
fn add_mul_fail() {
    NaturalVector::from_str("(1, 2)")
        .unwrap()
        .add_mul(NaturalVector::from_str("(1)").unwrap(), Natural::from(3u32));
}

#[test]
#[should_panic]
fn add_mul_ref_fail() {
    (&NaturalVector::from_str("(1, 2)").unwrap()).add_mul(
        &NaturalVector::from_str("(1)").unwrap(),
        &Natural::from(3u32),
    );
}

#[test]
#[should_panic]
fn add_mul_assign_fail() {
    let mut v = NaturalVector::from_str("(1, 2)").unwrap();
    v.add_mul_assign(NaturalVector::from_str("(1)").unwrap(), Natural::from(3u32));
}

#[test]
#[should_panic]
fn add_mul_assign_ref_fail() {
    let mut v = NaturalVector::from_str("(1, 2)").unwrap();
    v.add_mul_assign(
        &NaturalVector::from_str("(1)").unwrap(),
        &Natural::from(3u32),
    );
}

#[test]
fn add_mul_properties() {
    natural_vector_natural_vector_natural_triple_gen_var_2().test_properties(|(u, v, c)| {
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

        // It is the sum of the vector and the scalar product, element by element the scalar
        // add_mul, and the dimension is unchanged.
        assert_eq!(w, &u + &v * &c);
        assert_eq!(w.dimension(), u.dimension());
        for ((x, y), z) in u.elements.iter().zip(&v.elements).zip(&w.elements) {
            assert_eq!(*z, x.add_mul(y, &c));
        }
        // Swapping the roles of the vector and the scalar product changes nothing.
        assert_eq!((&v * &c).add_mul(&u, &Natural::ONE), w);
        // Adding two multiples one after the other adds their scalars.
        let d = &c + Natural::ONE;
        assert_eq!((&w).add_mul(&v, &Natural::ONE), (&u).add_mul(&v, &d));

        // As IntegerVectors and an Integer, the result is the same.
        assert_eq!(
            IntegerVector::from(w),
            IntegerVector::from(u) + IntegerVector::from(v) * Integer::from(c)
        );
    });

    natural_vector_pair_gen_var_1().test_properties(|(u, v)| {
        // Multiplying by 0 changes nothing, and by 1 adds the vectors.
        assert_eq!((&u).add_mul(&v, &Natural::ZERO), u);
        assert_eq!((&u).add_mul(&v, &Natural::ONE), &u + &v);
        // Adding a multiple of the zero vector changes nothing.
        assert_eq!(
            (&u).add_mul(&NaturalVector::zero(u.dimension()), &Natural::from(3u32)),
            u
        );
    });

    natural_vector_natural_pair_gen().test_properties(|(v, c)| {
        // Adding to the zero vector gives the scalar product.
        assert_eq!(NaturalVector::zero(v.dimension()).add_mul(&v, &c), &v * &c);
    });
}

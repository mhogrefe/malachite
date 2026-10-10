// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulShl, AddMulShlAssign, SubMulShl};
use malachite_base::num::basic::traits::Zero;
use malachite_base::vector::Vector;
use malachite_nz::test_util::generators::*;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{
    rational_vector_rational_vector_rational_triple_gen_var_1,
    rational_vector_rational_vector_rational_unsigned_quadruple_gen_var_1,
};

#[test]
fn test_add_mul_shl() {
    let test = |s, t, c, bits, out| {
        let u = RationalVector::from_str(s).unwrap();
        let v = RationalVector::from_str(t).unwrap();
        let c = Rational::from_str(c).unwrap();
        let w = (&u).add_mul_shl(&v, &c, bits);
        assert_eq!(w.to_string(), out);
        assert_eq!(u.clone().add_mul_shl(v.clone(), c.clone(), bits), w);
        assert_eq!(u.clone().add_mul_shl(v.clone(), &c, bits), w);
        assert_eq!(u.clone().add_mul_shl(&v, c.clone(), bits), w);
        assert_eq!(u.clone().add_mul_shl(&v, &c, bits), w);
        let mut x = u.clone();
        x.add_mul_shl_assign(v.clone(), c.clone(), bits);
        assert_eq!(x, w);
        let mut x = u.clone();
        x.add_mul_shl_assign(v.clone(), &c, bits);
        assert_eq!(x, w);
        let mut x = u.clone();
        x.add_mul_shl_assign(&v, c.clone(), bits);
        assert_eq!(x, w);
        let mut x = u;
        x.add_mul_shl_assign(&v, &c, bits);
        assert_eq!(x, w);
    };
    test("()", "()", "5", 3, "()");
    test("(1/2, -2, 3)", "(4, 1/3, -6)", "0", 5, "(1/2, -2, 3)");
    test("(1/2, -2, 3)", "(4, 1/3, -6)", "1", 2, "(33/2, -2/3, -21)");
    test(
        "(1/2, -2, 3)",
        "(4, 1/3, -6)",
        "-1",
        2,
        "(-31/2, -10/3, 27)",
    );
    test("(1/2, -2, 3)", "(4, 1/3, -6)", "3/2", 0, "(13/2, -3/2, -6)");
    test("(1/2, -2, 3)", "(4, 1/3, -6)", "3/2", 2, "(49/2, 0, -33)");
    test(
        "(1/3, 0)",
        "(18446744073709551615/7, -1)",
        "18446744073709551617/5",
        3,
        "(1633355361220504624624198115672487414991/21, -147573952589676412936/5)",
    );
}

#[test]
#[should_panic]
fn add_mul_shl_fail() {
    RationalVector::from_str("(1, 2)").unwrap().add_mul_shl(
        RationalVector::from_str("(1)").unwrap(),
        Rational::from(3),
        2,
    );
}

#[test]
#[should_panic]
fn add_mul_shl_ref_fail() {
    (&RationalVector::from_str("(1, 2)").unwrap()).add_mul_shl(
        &RationalVector::from_str("(1)").unwrap(),
        &Rational::from(3),
        2,
    );
}

#[test]
#[should_panic]
fn add_mul_shl_assign_fail() {
    let mut v = RationalVector::from_str("(1, 2)").unwrap();
    v.add_mul_shl_assign(
        RationalVector::from_str("(1)").unwrap(),
        Rational::from(3),
        2,
    );
}

#[test]
#[should_panic]
fn add_mul_shl_assign_ref_fail() {
    let mut v = RationalVector::from_str("(1, 2)").unwrap();
    v.add_mul_shl_assign(
        &RationalVector::from_str("(1)").unwrap(),
        &Rational::from(3),
        2,
    );
}

#[test]
fn add_mul_shl_properties() {
    rational_vector_rational_vector_rational_unsigned_quadruple_gen_var_1().test_properties(
        |(u, v, c, bits)| {
            let w = (&u).add_mul_shl(&v, &c, bits);
            // The forms agree.
            assert_eq!(u.clone().add_mul_shl(v.clone(), c.clone(), bits), w);
            assert_eq!(u.clone().add_mul_shl(v.clone(), &c, bits), w);
            assert_eq!(u.clone().add_mul_shl(&v, c.clone(), bits), w);
            assert_eq!(u.clone().add_mul_shl(&v, &c, bits), w);
            let mut x = u.clone();
            x.add_mul_shl_assign(v.clone(), c.clone(), bits);
            assert_eq!(x, w);
            let mut x = u.clone();
            x.add_mul_shl_assign(v.clone(), &c, bits);
            assert_eq!(x, w);
            let mut x = u.clone();
            x.add_mul_shl_assign(&v, c.clone(), bits);
            assert_eq!(x, w);
            let mut x = u.clone();
            x.add_mul_shl_assign(&v, &c, bits);
            assert_eq!(x, w);

            // It is the unfused combination, element by element the scalar add_mul_shl, and the
            // dimension is unchanged.
            assert_eq!(w, &u + ((&v * &c) << bits));
            assert_eq!(w.dimension(), u.dimension());
            for ((x, y), z) in u.elements.iter().zip(&v.elements).zip(&w.elements) {
                assert_eq!(*z, x.add_mul_shl(y, &c, bits));
            }
            // Negating the scalar gives the opposite operation, which undoes this one.
            assert_eq!((&u).sub_mul_shl(&v, &-&c, bits), w);
            assert_eq!((&w).sub_mul_shl(&v, &c, bits), u);
            // Negating everything negates the result.
            assert_eq!((-&u).add_mul_shl(-&v, &c, bits), -&w);
            // Shifts compose.
            assert_eq!(
                (&u).add_mul_shl(&(&v << 1u32), &c, bits),
                (&u).add_mul_shl(&v, &c, bits + 1)
            );
            // A zero scalar changes nothing, and starting from the zero vector gives the shifted
            // scalar product.
            assert_eq!((&u).add_mul_shl(&v, &Rational::ZERO, bits), u);
            assert_eq!(
                RationalVector::zero(v.dimension()).add_mul_shl(&v, &c, bits),
                (&v * &c) << bits
            );
        },
    );

    rational_vector_rational_vector_rational_triple_gen_var_1().test_properties(|(u, v, c)| {
        // With no shift, this is `add_mul`.
        assert_eq!((&u).add_mul_shl(&v, &c, 0), (&u).add_mul(&v, &c));
    });

    integer_vector_integer_vector_integer_unsigned_quadruple_gen_var_1().test_properties(
        |(u, v, c, bits)| {
            // It agrees with the `IntegerVector` implementation.
            assert_eq!(
                RationalVector::from((&u).add_mul_shl(&v, &c, bits)),
                RationalVector::from(u).add_mul_shl(
                    RationalVector::from(v),
                    Rational::from(c),
                    bits
                )
            );
        },
    );
}

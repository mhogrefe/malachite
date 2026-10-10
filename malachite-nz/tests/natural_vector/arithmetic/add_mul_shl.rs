// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{AddMul, AddMulShl, AddMulShlAssign};
use malachite_base::num::basic::traits::Zero;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_vector_natural_vector_natural_triple_gen_var_2,
    natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_2,
};

#[test]
fn test_add_mul_shl() {
    let test = |s, t, c, bits, out| {
        let u = NaturalVector::from_str(s).unwrap();
        let v = NaturalVector::from_str(t).unwrap();
        let c = Natural::from_str(c).unwrap();
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
    test("(1, 2, 3)", "(4, 5, 6)", "0", 5, "(1, 2, 3)");
    test("(1, 2, 3)", "(4, 5, 6)", "1", 2, "(17, 22, 27)");
    test("(1, 2, 3)", "(4, 5, 6)", "10", 0, "(41, 52, 63)");
    test("(1, 2, 3)", "(4, 5, 6)", "10", 2, "(161, 202, 243)");
    test(
        "(1, 0)",
        "(18446744073709551615, 1)",
        "18446744073709551617",
        3,
        "(2722258935367507707706996859454145691641, 147573952589676412936)",
    );
    test(
        "(1267650600228229401496703205376, 7)",
        "(3, 5)",
        "36893488147419103232",
        64,
        "(2041694202793281381008477046087312474112, 3402823669209384634633746074317682114567)",
    );
}

#[test]
#[should_panic]
fn add_mul_shl_fail() {
    NaturalVector::from_str("(1, 2)").unwrap().add_mul_shl(
        NaturalVector::from_str("(1)").unwrap(),
        Natural::from(3u32),
        2,
    );
}

#[test]
#[should_panic]
fn add_mul_shl_ref_fail() {
    (&NaturalVector::from_str("(1, 2)").unwrap()).add_mul_shl(
        &NaturalVector::from_str("(1)").unwrap(),
        &Natural::from(3u32),
        2,
    );
}

#[test]
#[should_panic]
fn add_mul_shl_assign_fail() {
    let mut v = NaturalVector::from_str("(1, 2)").unwrap();
    v.add_mul_shl_assign(
        NaturalVector::from_str("(1)").unwrap(),
        Natural::from(3u32),
        2,
    );
}

#[test]
#[should_panic]
fn add_mul_shl_assign_ref_fail() {
    let mut v = NaturalVector::from_str("(1, 2)").unwrap();
    v.add_mul_shl_assign(
        &NaturalVector::from_str("(1)").unwrap(),
        &Natural::from(3u32),
        2,
    );
}

#[test]
fn add_mul_shl_properties() {
    natural_vector_natural_vector_natural_unsigned_quadruple_gen_var_2().test_properties(
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
            // Shifts compose.
            assert_eq!(
                (&u).add_mul_shl(&(&v << 1u32), &c, bits),
                (&u).add_mul_shl(&v, &c, bits + 1)
            );
            // A zero scalar changes nothing, and starting from the zero vector gives the shifted
            // scalar product.
            assert_eq!((&u).add_mul_shl(&v, &Natural::ZERO, bits), u);
            assert_eq!(
                NaturalVector::zero(v.dimension()).add_mul_shl(&v, &c, bits),
                (&v * &c) << bits
            );
            // It agrees with the `IntegerVector` implementation.
            assert_eq!(
                IntegerVector::from(u).add_mul_shl(IntegerVector::from(v), Integer::from(c), bits),
                IntegerVector::from(w)
            );
        },
    );

    natural_vector_natural_vector_natural_triple_gen_var_2().test_properties(|(u, v, c)| {
        // With no shift, this is `add_mul`.
        assert_eq!((&u).add_mul_shl(&v, &c, 0), (&u).add_mul(&v, &c));
    });
}

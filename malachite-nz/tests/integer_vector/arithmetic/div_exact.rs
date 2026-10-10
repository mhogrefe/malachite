// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{DivExact, DivExactAssign};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_integer_pair_gen_var_2;

use std::panic::catch_unwind;

#[test]
fn test_div_exact() {
    let test = |s, c, out| {
        let v = IntegerVector::from_str(s).unwrap();
        let c = Integer::from_str(c).unwrap();
        let w = (&v).div_exact(&c);
        assert_eq!(w.to_string(), out);
        assert_eq!((&v).div_exact(c.clone()), w);
        assert_eq!(v.clone().div_exact(&c), w);
        assert_eq!(v.clone().div_exact(c.clone()), w);
        let mut x = v.clone();
        x.div_exact_assign(&c);
        assert_eq!(x, w);
        let mut x = v;
        x.div_exact_assign(c);
        assert_eq!(x, w);
    };
    test("()", "5", "()");
    test("(0, -6, 255)", "1", "(0, -6, 255)");
    test("(0, -6, 255)", "-1", "(0, 6, -255)");
    test("(0, -6, 255)", "3", "(0, -2, 85)");
    test("(0, -6, 255)", "-3", "(0, 2, -85)");
    test(
        "(-3802951800684688204490109616128, 6)",
        "3",
        "(-1267650600228229401496703205376, 2)",
    );
    test(
        "(1267650600228229401496703205376, 0)",
        "-1267650600228229401496703205376",
        "(-1, 0)",
    );
}

#[test]
fn div_exact_fail() {
    let v = IntegerVector::from_str("(1, 2)").unwrap();
    assert_panic!((&v).div_exact(Integer::ZERO));
    assert_panic!((&v).div_exact(&Integer::ZERO));
    assert_panic!(v.clone().div_exact(Integer::ZERO));
    assert_panic!(v.clone().div_exact(&Integer::ZERO));
    assert_panic!(
        IntegerVector::from_str("()")
            .unwrap()
            .div_exact(Integer::ZERO)
    );
    assert_panic!({
        let mut w = v.clone();
        w.div_exact_assign(Integer::ZERO);
    });
    assert_panic!({
        let mut w = v.clone();
        w.div_exact_assign(&Integer::ZERO);
    });
}

#[test]
fn div_exact_properties() {
    integer_vector_integer_pair_gen_var_2().test_properties(|(v, c)| {
        let w = (&v).div_exact(&c);
        // The forms agree.
        assert_eq!((&v).div_exact(c.clone()), w);
        assert_eq!(v.clone().div_exact(&c), w);
        assert_eq!(v.clone().div_exact(c.clone()), w);
        let mut x = v.clone();
        x.div_exact_assign(&c);
        assert_eq!(x, w);
        let mut x = v.clone();
        x.div_exact_assign(c.clone());
        assert_eq!(x, w);

        // Element by element, this is the scalar exact quotient, which is also the ordinary
        // quotient. The dimension is unchanged, and multiplying by the scalar recovers the vector.
        assert_eq!(w.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x.div_exact(&c));
            assert_eq!(*y, x / &c);
        }
        assert_eq!(&w * &c, v);
        // Dividing by 1 changes nothing.
        assert_eq!((&v).div_exact(Integer::ONE), v);
        assert_eq!((&v).div_exact(Integer::NEGATIVE_ONE), -&v);
        // Dividing by -c negates the quotient.
        assert_eq!((&v).div_exact(-&c), -&w);
    });
}

// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{
    DivExact, DivRound, EntrywiseDivRound, EntrywiseShrRound, IsPowerOf2,
};
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::{
    integer_vector_integer_pair_gen_var_1, integer_vector_integer_pair_gen_var_2,
};
use std::panic::catch_unwind;

#[test]
fn test_div() {
    let test = |s, c, out| {
        let v = IntegerVector::from_str(s).unwrap();
        let c = Integer::from_str(c).unwrap();
        let w = &v / &c;
        assert_eq!(w.to_string(), out);
        assert_eq!(&v / c.clone(), w);
        assert_eq!(v.clone() / &c, w);
        assert_eq!(v.clone() / c.clone(), w);
        let mut x = v.clone();
        x /= &c;
        assert_eq!(x, w);
        let mut x = v;
        x /= c;
        assert_eq!(x, w);
    };
    test("()", "5", "()");
    test("(0, -7, 255)", "1", "(0, -7, 255)");
    test("(0, -7, 255)", "-1", "(0, 7, -255)");
    test("(0, -7, 255)", "2", "(0, -3, 127)");
    test("(0, -7, 255)", "-2", "(0, 3, -127)");
    test(
        "(-3802951800684688204490109616128, 7)",
        "3",
        "(-1267650600228229401496703205376, 2)",
    );
}

#[test]
fn div_fail() {
    let v = IntegerVector::from_str("(1, 2)").unwrap();
    assert_panic!(&v / Integer::ZERO);
    assert_panic!(&v / &Integer::ZERO);
    assert_panic!(v.clone() / Integer::ZERO);
    assert_panic!(v.clone() / &Integer::ZERO);
    assert_panic!(IntegerVector::from_str("()").unwrap() / Integer::ZERO);
    assert_panic!({
        let mut w = v.clone();
        w /= Integer::ZERO;
    });
    assert_panic!({
        let mut w = v.clone();
        w /= &Integer::ZERO;
    });
}

#[test]
fn div_properties() {
    integer_vector_integer_pair_gen_var_1().test_properties(|(v, c)| {
        let w = &v / &c;
        // The forms agree.
        assert_eq!(&v / c.clone(), w);
        assert_eq!(v.clone() / &c, w);
        assert_eq!(v.clone() / c.clone(), w);
        let mut x = v.clone();
        x /= &c;
        assert_eq!(x, w);
        let mut x = v.clone();
        x /= c.clone();
        assert_eq!(x, w);

        // Element by element, this is the scalar quotient, rounded toward zero, and the dimension
        // is unchanged.
        assert_eq!(w.dimension(), v.dimension());
        for (x, y) in v.elements.iter().zip(&w.elements) {
            assert_eq!(*y, x / &c);
            assert_eq!(*y, x.div_round(&c, Down).0);
        }
        // It is entrywise division rounding toward zero.
        assert_eq!((&v).entrywise_div_round(&c, Down), w);
        // Dividing by 1 changes nothing.
        assert_eq!(&v / Integer::ONE, v);
        assert_eq!(&v / Integer::NEGATIVE_ONE, -&v);
        // Dividing by -c negates the quotient.
        assert_eq!(&v / -&c, -&w);
        // Dividing by a positive power of 2 is a right shift rounding toward zero.
        if c > 0u32 && c.unsigned_abs_ref().is_power_of_2() {
            let bits = c.unsigned_abs_ref().trailing_zeros().unwrap();
            assert_eq!((&v).entrywise_shr_round(bits, Down), w);
        }
    });

    integer_vector_integer_pair_gen_var_2().test_properties(|(v, c)| {
        // When the division is exact, it agrees with exact division.
        assert_eq!(&v / &c, (&v).div_exact(&c));
    });
}

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
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::rounding_modes::RoundingMode::*;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{
    natural_vector_natural_pair_gen_var_1, natural_vector_natural_pair_gen_var_3,
};
use std::panic::catch_unwind;

#[test]
fn test_div() {
    let test = |s, c, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let c = Natural::from_str(c).unwrap();
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
    test("(0, 7, 255)", "1", "(0, 7, 255)");
    test("(0, 7, 255)", "2", "(0, 3, 127)");
    test(
        "(3802951800684688204490109616128, 7)",
        "3",
        "(1267650600228229401496703205376, 2)",
    );
}

#[test]
fn div_fail() {
    let v = NaturalVector::from_str("(1, 2)").unwrap();
    assert_panic!(&v / Natural::ZERO);
    assert_panic!(&v / &Natural::ZERO);
    assert_panic!(v.clone() / Natural::ZERO);
    assert_panic!(v.clone() / &Natural::ZERO);
    assert_panic!(NaturalVector::from_str("()").unwrap() / Natural::ZERO);
    assert_panic!({
        let mut w = v.clone();
        w /= Natural::ZERO;
    });
    assert_panic!({
        let mut w = v.clone();
        w /= &Natural::ZERO;
    });
}

#[test]
fn div_properties() {
    natural_vector_natural_pair_gen_var_1().test_properties(|(v, c)| {
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
        // It is entrywise division rounding down.
        assert_eq!((&v).entrywise_div_round(&c, Floor), w);
        assert_eq!((&v).entrywise_div_round(&c, Down), w);
        // Dividing by 1 changes nothing.
        assert_eq!(&v / Natural::ONE, v);
        // Dividing by a power of 2 is a right shift.
        if c.is_power_of_2() {
            let bits = c.trailing_zeros().unwrap();
            assert_eq!(&v >> bits, w);
            assert_eq!((&v).entrywise_shr_round(bits, Down), w);
        }
        // As an IntegerVector, the quotient is the same.
        assert_eq!(
            IntegerVector::from(w.clone()),
            IntegerVector::from(v.clone()) / Integer::from(&c)
        );
    });

    natural_vector_natural_pair_gen_var_3().test_properties(|(v, c)| {
        // When the division is exact, it agrees with exact division.
        assert_eq!(&v / &c, (&v).div_exact(&c));
    });
}

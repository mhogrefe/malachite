// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Abs, Content, ContentChained, Gcd};
use malachite_base::num::basic::traits::Zero;
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vec_natural_pair_gen;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{
    rational_vector_pair_gen, rational_vector_rational_pair_gen,
};

#[test]
fn test_content_chained() {
    let test = |s, x, out| {
        let v = RationalVector::from_str(s).unwrap();
        let x = Rational::from_str(x).unwrap();
        let g = (&v).content_chained(&x);
        assert_eq!(g.to_string(), out);
        assert_eq!((&v).content_chained(x.clone()), g);
        assert_eq!(v.clone().content_chained(&x), g);
        assert_eq!(v.content_chained(x), g);
    };
    test("()", "1/3", "1/3");
    test("()", "0", "0");
    test("(0, 0)", "-2/5", "2/5");
    test("(1/2, -3/4)", "1/6", "1/12");
    test("(1/2, -3/4)", "0", "1/4");
    test("(2, 4, 6)", "3", "1");
    test("(2, 4, 6)", "8", "2");
}

#[test]
fn content_chained_properties() {
    rational_vector_rational_pair_gen().test_properties(|(v, x)| {
        let g = (&v).content_chained(&x);
        assert_eq!((&v).content_chained(x.clone()), g);
        assert_eq!(v.clone().content_chained(&x), g);
        assert_eq!(v.clone().content_chained(x.clone()), g);
        assert!(g >= 0u32);
        // It is the GCD of `|x|` and the elements, taken one at a time.
        assert_eq!(v.elements.iter().fold((&x).abs(), Gcd::gcd), g);
    });

    rational_vector_pair_gen().test_properties(|(v, w)| {
        // Chaining accumulates the content of several vectors.
        let mut xs = v.to_elements();
        xs.extend(w.to_elements());
        assert_eq!(
            w.content_chained(v.content_chained(Rational::ZERO)),
            RationalVector::from_owned_elements(xs).content()
        );
    });

    integer_vec_natural_pair_gen().test_properties(|(xs, x)| {
        // It agrees with the IntegerVector implementation.
        let v = IntegerVector::from_owned_elements(xs);
        assert_eq!(
            RationalVector::from(v.clone()).content_chained(Rational::from(&x)),
            Rational::from(v.content_chained(x))
        );
    });
}

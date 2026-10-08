// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::cmp::Ordering::{self, *};
use core::str::FromStr;
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::common::test_custom_cmp_helper;
use malachite_base::vector::Vector;
use malachite_q::Rational;
use malachite_q::rational_vector::{
    RationalVector, ShortlexRationalVector, ShortlexRationalVectorRef,
};
use malachite_q::test_util::generators::{
    rational_vec_gen, rational_vector_gen, rational_vector_pair_gen, rational_vector_triple_gen,
};
use malachite_q::test_util::rational_vector::comparison::shortlex_cmp::*;

fn shortlex(v: &RationalVector, w: &RationalVector) -> Ordering {
    ShortlexRationalVectorRef(v).cmp(&ShortlexRationalVectorRef(w))
}

#[test]
fn test_shortlex_cmp() {
    // In ascending order: the 0-dimensional vector first, then every 1-dimensional vector by its
    // element, then the 2-dimensional ones lexicographically, and so on.
    test_custom_cmp_helper::<RationalVector, _>(
        &[
            "()",
            "(-1)",
            "(-1/2)",
            "(0)",
            "(1/2)",
            "(1)",
            "(100)",
            "(-1, 0)",
            "(0, 0)",
            "(0, 1/2)",
            "(0, 1)",
            "(0, 100)",
            "(1, 0)",
            "(5, 5)",
            "(0, 0, 0)",
            "(0, 0, 1)",
            "(1000000000000000000000000, 0, 0)",
            "(0, 0, 0, 0)",
        ],
        shortlex,
    );
}

#[test]
fn test_shortlex_cmp_vs_lexicographic() {
    // The two orders agree except where the dimensions differ.
    let test = |s, t, shortlex_out, lex_out| {
        let v = RationalVector::from_str(s).unwrap();
        let w = RationalVector::from_str(t).unwrap();
        assert_eq!(shortlex(&v, &w), shortlex_out, "shortlex {s} vs {t}");
        assert_eq!(
            v.elements.cmp(&w.elements),
            lex_out,
            "lexicographic {s} vs {t}"
        );
    };
    // Dimension decides under shortlex; the first element decides lexicographically.
    test("(5)", "(0, 0)", Less, Greater);
    test("()", "(0)", Less, Less);
    test("(1, 0, 0)", "(2, 0)", Greater, Less);
    // At equal dimensions they always agree.
    test("(0, 1)", "(1, 0)", Less, Less);
    test("(3, 4)", "(3, 4)", Equal, Equal);
}

#[test]
fn shortlex_cmp_properties() {
    rational_vector_pair_gen().test_properties(|(v, w)| {
        let c = shortlex(&v, &w);
        // Comparison is antisymmetric, and agrees with `Eq`.
        assert_eq!(shortlex(&w, &v), c.reverse());
        assert_eq!(v == w, c == Equal);

        // The default comparison gives the same answer as an explicit walk over the elements.
        assert_eq!(rational_vector_shortlex_cmp_naive(&v, &w), c);

        // The owned wrapper and the borrowing one agree, and both agree with `as_ref`.
        let owned = ShortlexRationalVector(v.clone());
        let other_owned = ShortlexRationalVector(w.clone());
        assert_eq!(owned.cmp(&other_owned), c);
        assert_eq!(owned.as_ref().cmp(&other_owned.as_ref()), c);
        assert_eq!(owned.partial_cmp(&other_owned), Some(c));

        // Shortlex is lexicographic order on (dimension, elements).
        assert_eq!(
            (v.dimension(), &v.elements).cmp(&(w.dimension(), &w.elements)),
            c
        );

        // Dimension decides outright; at equal dimensions it is lexicographic order.
        match v.dimension().cmp(&w.dimension()) {
            Equal => assert_eq!(v.elements.cmp(&w.elements), c),
            d => assert_eq!(c, d),
        }
    });

    rational_vector_gen().test_properties(|v| {
        // Reflexivity, and the 0-dimensional vector is the least of them all.
        assert_eq!(shortlex(&v, &v), Equal);
        assert!(
            ShortlexRationalVector(v)
                >= ShortlexRationalVector(RationalVector {
                    elements: Vec::new()
                })
        );
    });

    rational_vector_triple_gen().test_properties(|(u, v, w)| {
        // Transitivity, in the two forms that make the order a total one.
        if shortlex(&u, &v) == Less && shortlex(&v, &w) == Less {
            assert_eq!(shortlex(&u, &w), Less);
        } else if shortlex(&u, &v) == Greater && shortlex(&v, &w) == Greater {
            assert_eq!(shortlex(&u, &w), Greater);
        }
    });

    rational_vec_gen().test_properties(|mut xs| {
        // Appending an element moves a vector up, past everything of its old dimension.
        let v = RationalVector {
            elements: xs.clone(),
        };
        xs.push(Rational::ZERO);
        let w = RationalVector { elements: xs };
        assert_eq!(shortlex(&v, &w), Less);
    });
}

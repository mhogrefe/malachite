// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.
use core::str::FromStr;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::{IntegerVector, ShortlexIntegerVectorRef};
use malachite_nz::natural_vector::{NaturalVector, ShortlexNaturalVectorRef};
use malachite_nz::test_util::generators::{natural_vector_gen, natural_vector_pair_gen};

#[test]
fn test_from_natural_vector() {
    let test = |s| {
        let v = NaturalVector::from_str(s).unwrap();
        assert_eq!(IntegerVector::from(v).to_string(), s);
    };
    test("()");
    test("(0)");
    test("(1, 2, 3)");
    test("(123456789012345678901234567890, 0)");
}

#[test]
fn from_natural_vector_properties() {
    natural_vector_gen().test_properties(|v| {
        let w = IntegerVector::from(v.clone());
        // Nothing is lost: the dimension, every element, and the written form all survive.
        assert_eq!(w.dimension(), v.dimension());
        assert_eq!(w.to_string(), v.to_string());
        assert_eq!(
            w.elements,
            v.elements
                .iter()
                .map(|x| Integer::from(x.clone()))
                .collect::<Vec<_>>()
        );
        // The result is never negative anywhere, since it came from `Natural`s.
        assert!(w.elements.iter().all(|x| *x >= 0u32));
        // Reading the string back as an `IntegerVector` gives the same thing.
        assert_eq!(IntegerVector::from_str(&v.to_string()).unwrap(), w);
    });

    natural_vector_pair_gen().test_properties(|(v, w)| {
        let v_i = IntegerVector::from(v.clone());
        let w_i = IntegerVector::from(w.clone());
        // The two types order their vectors the same way.
        assert_eq!(
            ShortlexNaturalVectorRef(&v).cmp(&ShortlexNaturalVectorRef(&w)),
            ShortlexIntegerVectorRef(&v_i).cmp(&ShortlexIntegerVectorRef(&w_i))
        );
        assert_eq!(v == w, v_i == w_i);
    });
}

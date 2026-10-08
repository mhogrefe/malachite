// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::basic::traits::One;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::{NaturalVector, ShortlexNaturalVectorRef};
use malachite_nz::test_util::generators::{natural_vector_gen, natural_vector_pair_gen};
use malachite_q::Rational;
use malachite_q::rational_vector::{RationalVector, ShortlexRationalVectorRef};

#[test]
fn test_from_natural_vector() {
    let test = |s| {
        let v = NaturalVector::from_str(s).unwrap();
        assert_eq!(RationalVector::from(v).to_string(), s);
    };
    test("()");
    test("(0)");
    test("(1, 2, 3)");
    test("(123456789012345678901234567890, 0)");
}

#[test]
fn from_natural_vector_properties() {
    natural_vector_gen().test_properties(|v| {
        let w = RationalVector::from(v.clone());
        // Nothing is lost: the dimension, every element, and the written form all survive.
        assert_eq!(w.dimension(), v.dimension());
        assert_eq!(w.to_string(), v.to_string());
        assert_eq!(
            w.elements,
            v.elements
                .iter()
                .map(|x| Rational::from(x.clone()))
                .collect::<Vec<_>>()
        );
        // Every element is an integer.
        assert!(w.elements.iter().all(|x| *x.denominator_ref() == 1u32));
        // The result is never negative anywhere, since it came from `Natural`s.
        assert!(w.elements.iter().all(|x| *x >= 0u32));
        // Reading the string back as a `RationalVector` gives the same thing.
        assert_eq!(RationalVector::from_str(&v.to_string()).unwrap(), w);
        // Clearing the denominators gives back the elements, over the denominator 1.
        assert_eq!(
            w.to_numerators_and_denominator(),
            (IntegerVector::from(v.clone()), Natural::ONE)
        );
        // Going through an `IntegerVector` gives the same thing.
        assert_eq!(RationalVector::from(IntegerVector::from(v)), w);
    });

    natural_vector_pair_gen().test_properties(|(v, w)| {
        // The two types order their vectors the same way.
        assert_eq!(
            ShortlexNaturalVectorRef(&v).cmp(&ShortlexNaturalVectorRef(&w)),
            ShortlexRationalVectorRef(&RationalVector::from(v.clone()))
                .cmp(&ShortlexRationalVectorRef(&RationalVector::from(w.clone())))
        );
        assert_eq!(v == w, RationalVector::from(v) == RationalVector::from(w));
    });
}

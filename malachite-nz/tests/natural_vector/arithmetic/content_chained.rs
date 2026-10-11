// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Content, ContentChained, DivisibleBy, Gcd};
use malachite_base::num::basic::traits::Zero;
use malachite_base::test_util::generators::unsigned_vec_unsigned_pair_gen;
use malachite_base::unsigned_vector::UnsignedVector;
use malachite_base::vector::Vector;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::platform::Limb;
use malachite_nz::test_util::generators::{
    natural_vector_natural_pair_gen, natural_vector_pair_gen,
};

#[test]
fn test_content_chained() {
    let test = |s, x, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let x = Natural::from_str(x).unwrap();
        let g = (&v).content_chained(&x);
        assert_eq!(g.to_string(), out);
        assert_eq!((&v).content_chained(x.clone()), g);
        assert_eq!(v.clone().content_chained(&x), g);
        assert_eq!(v.content_chained(x), g);
    };
    // - all elements zero, or none
    test("()", "6", "6");
    test("()", "0", "0");
    test("(0, 0)", "6", "6");
    // - one element
    test("(4)", "6", "2");
    // - zeros at both ends are skipped
    test("(0, 4, 0)", "6", "2");
    // - an element of 1 at either end
    test("(1, 6)", "4", "1");
    test("(6, 1)", "4", "1");
    // - pairs from both ends, then one element left
    test("(6, 4, 10)", "4", "2");
    test("(12, 24, 36, 48, 60)", "12", "12");
    // - the GCD reaches 1 with elements left
    test("(2, 6, 6, 3)", "6", "1");
    test(
        "(1000000000000000000000, 3000000000000000000000, 0)",
        "5000000000000000000000",
        "1000000000000000000000",
    );
}

#[test]
fn content_chained_properties() {
    natural_vector_natural_pair_gen().test_properties(|(v, x)| {
        let g = (&v).content_chained(&x);
        assert_eq!((&v).content_chained(x.clone()), g);
        assert_eq!(v.clone().content_chained(&x), g);
        assert_eq!(v.clone().content_chained(x.clone()), g);
        // It is the GCD of `x` and the elements, taken one at a time.
        assert_eq!(v.elements.iter().fold(x.clone(), Gcd::gcd), g);
        assert_eq!((&x).gcd((&v).content()), g);
        assert!((&x).divisible_by(&g));
        assert!(v.elements.iter().all(|c| c.divisible_by(&g)));
        // It agrees with the IntegerVector implementation.
        assert_eq!(IntegerVector::from(v.clone()).content_chained(&x), g);
    });

    natural_vector_pair_gen().test_properties(|(v, w)| {
        // Chaining accumulates the content of several vectors.
        let mut xs = v.to_elements();
        xs.extend(w.to_elements());
        assert_eq!(
            w.content_chained(v.content_chained(Natural::ZERO)),
            NaturalVector::from_owned_elements(xs).content()
        );
    });

    unsigned_vec_unsigned_pair_gen::<Limb, Limb>().test_properties(|(xs, x)| {
        // It agrees with the UnsignedVector implementation.
        let v = UnsignedVector::from_owned_elements(xs);
        assert_eq!(
            NaturalVector::from(v.clone()).content_chained(Natural::from(x)),
            v.content_chained(x)
        );
    });
}

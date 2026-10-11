// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{ElementProduct, Parity};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::vector::Vector;
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_q::Rational;
use malachite_q::rational_vector::RationalVector;
use malachite_q::test_util::generators::{rational_vector_gen, rational_vector_pair_gen};

#[test]
fn test_element_product() {
    let test = |s, out| {
        let v = RationalVector::from_str(s).unwrap();
        let product = v.to_element_product();
        assert_eq!(product.to_string(), out);
        assert_eq!(v.into_element_product(), product);
    };
    test("()", "1");
    test("(-5/2)", "-5/2");
    test("(1/2, -2/3)", "-1/3");
    test("(2/3, 3/2)", "1");
    test("(1/2, 0)", "0");
}

#[test]
fn element_product_properties() {
    rational_vector_gen().test_properties(|v| {
        let product = v.to_element_product();
        assert_eq!(v.clone().into_element_product(), product);
        assert_eq!(
            v.elements.iter().fold(Rational::ONE, |acc, x| acc * x),
            product
        );
        // The product is zero exactly when some element is, and negating the vector negates the
        // product when the dimension is odd.
        assert_eq!(product == 0u32, v.elements.contains(&Rational::ZERO));
        let negated = (-&v).into_element_product();
        assert_eq!(
            negated,
            if v.dimension().odd() {
                -product
            } else {
                product
            }
        );
    });

    rational_vector_pair_gen().test_properties(|(v, w)| {
        // The product of the elements of a concatenation is the product of the two products.
        let mut xs = v.to_elements();
        xs.extend(w.to_elements());
        assert_eq!(
            RationalVector::from_owned_elements(xs).into_element_product(),
            v.to_element_product() * w.to_element_product()
        );
    });

    integer_vector_gen().test_properties(|v| {
        // It agrees with the IntegerVector implementation.
        assert_eq!(
            RationalVector::from(v.clone()).into_element_product(),
            Rational::from(v.into_element_product())
        );
    });
}

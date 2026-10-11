// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::ElementProduct;
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{natural_vector_gen, natural_vector_pair_gen};

#[test]
fn test_element_product() {
    let test = |s, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let product = v.to_element_product();
        assert_eq!(product.to_string(), out);
        assert_eq!(v.into_element_product(), product);
    };
    test("()", "1");
    test("(5)", "5");
    test("(2, 3, 7)", "42");
    test("(2, 0, 7)", "0");
    test("(2, 3, 5, 7, 11)", "2310");
    test(
        "(18446744073709551615, 18446744073709551615)",
        "340282366920938463426481119284349108225",
    );
}

#[test]
fn element_product_properties() {
    natural_vector_gen().test_properties(|v| {
        let product = v.to_element_product();
        assert_eq!(v.clone().into_element_product(), product);
        assert_eq!(
            v.elements.iter().fold(Natural::ONE, |acc, x| acc * x),
            product
        );
        // The product is zero exactly when some element is.
        assert_eq!(product == 0u32, v.elements.contains(&Natural::ZERO));
        // It agrees with the IntegerVector implementation.
        assert_eq!(
            IntegerVector::from(v).into_element_product(),
            Integer::from(&product)
        );
    });

    natural_vector_pair_gen().test_properties(|(v, w)| {
        // The product of the elements of a concatenation is the product of the two products.
        let mut xs = v.to_elements();
        xs.extend(w.to_elements());
        assert_eq!(
            NaturalVector::from_owned_elements(xs).into_element_product(),
            v.to_element_product() * w.to_element_product()
        );
    });
}

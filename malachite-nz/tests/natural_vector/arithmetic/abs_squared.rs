// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{AbsSquared, Dot};
use malachite_base::vector::Vector;
use malachite_nz::natural_vector::NaturalVector;
use malachite_nz::test_util::generators::{integer_vector_gen, natural_vector_gen};
use malachite_nz::test_util::natural_vector::arithmetic::abs_squared::*;

#[test]
fn test_abs_squared() {
    let test = |s, out| {
        let v = NaturalVector::from_str(s).unwrap();
        let n = (&v).abs_squared();
        assert!(n.is_valid());
        assert_eq!(n.to_string(), out);
        assert_eq!(natural_vector_abs_squared_naive(&v), n);
        assert_eq!(v.abs_squared(), n);
    };
    test("()", "0");
    test("(0)", "0");
    test("(5)", "25");
    test("(3, 4, 12)", "169");
    test(
        "(1000000000000000000000, 1)",
        "1000000000000000000000000000000000000000001",
    );
    test(
        "(18446744073709551615, 18446744073709551615)",
        "680564733841876926852962238568698216450",
    );
}

#[test]
fn abs_squared_properties() {
    natural_vector_gen().test_properties(|v| {
        let n = (&v).abs_squared();
        assert!(n.is_valid());
        assert_eq!(v.clone().abs_squared(), n);
        assert_eq!(natural_vector_abs_squared_naive(&v), n);
        // It is the dot product of the vector with itself, and it is zero exactly when the vector
        // is.
        assert_eq!((&v).dot(&v), n);
        assert_eq!(n == 0u32, v.elements.iter().all(|x| *x == 0u32));
    });

    integer_vector_gen().test_properties(|v| {
        // It agrees with the IntegerVector implementation on the absolute values.
        let abs = NaturalVector::from_owned_elements(
            v.elements
                .iter()
                .map(|x| x.unsigned_abs_ref().clone())
                .collect(),
        );
        assert_eq!(abs.abs_squared(), v.abs_squared());
    });
}

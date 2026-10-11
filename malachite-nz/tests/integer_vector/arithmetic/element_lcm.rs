// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{DivisibleBy, ElementLcm, Lcm};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{integer_vector_gen, integer_vector_pair_gen};

#[test]
fn test_element_lcm() {
    let test = |s, out| {
        let v = IntegerVector::from_str(s).unwrap();
        let lcm = v.to_element_lcm();
        assert!(lcm.is_valid());
        assert_eq!(lcm.to_string(), out);
        assert_eq!(v.into_element_lcm(), lcm);
    };
    test("()", "1");
    test("(-5)", "5");
    test("(-4, 6, -10)", "60");
    test("(-4, 0, 6)", "0");
    test("(-1, 1)", "1");
    test(
        "(-1000000000000000000000, 3000000000000000000000)",
        "3000000000000000000000",
    );
}

#[test]
fn element_lcm_properties() {
    integer_vector_gen().test_properties(|v| {
        let lcm = v.to_element_lcm();
        assert!(lcm.is_valid());
        assert_eq!(v.clone().into_element_lcm(), lcm);
        // It is the LCM of the absolute values of the elements, taken one at a time.
        assert_eq!(
            v.elements
                .iter()
                .fold(Natural::ONE, |l, c| l.lcm(c.unsigned_abs_ref())),
            lcm
        );
        // It is zero exactly when an element is; otherwise every element divides it.
        assert_eq!(lcm == 0u32, v.elements.contains(&Integer::ZERO));
        if lcm != 0u32 {
            assert!(
                v.elements
                    .iter()
                    .all(|c| Integer::from(&lcm).divisible_by(c))
            );
        }
        // Negating the vector does not change it.
        assert_eq!((-&v).into_element_lcm(), lcm);
    });

    integer_vector_pair_gen().test_properties(|(v, w)| {
        // The LCM of a concatenation is the LCM of the two LCMs.
        let mut xs = v.to_elements();
        xs.extend(w.to_elements());
        assert_eq!(
            IntegerVector::from_owned_elements(xs).into_element_lcm(),
            v.into_element_lcm().lcm(w.into_element_lcm())
        );
    });
}

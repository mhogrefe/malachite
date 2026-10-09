// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use core::str::FromStr;
use malachite_base::num::arithmetic::traits::{Height, L1Norm};
use malachite_base::num::logic::traits::SignificantBits;
use malachite_base::vector::Vector;
use malachite_nz::integer::Integer;
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::integer_vector_gen;
use malachite_nz::test_util::integer_vector::arithmetic::sum_max_bits::vec_sum_max_bits_naive;

#[test]
fn test_l1_norm() {
    let test = |s, out, bits| {
        let v = IntegerVector::from_str(s).unwrap();
        let norm = v.to_l1_norm();
        assert_eq!(norm.to_string(), out);
        assert_eq!(v.l1_norm_significant_bits(), bits);
        assert_eq!(v.into_l1_norm(), norm);
    };
    test("()", "0", 0);
    test("(1, -5, 2)", "8", 4);
}

#[test]
fn l1_norm_properties() {
    integer_vector_gen().test_properties(|v| {
        let norm = v.to_l1_norm();
        assert_eq!(v.clone().into_l1_norm(), norm);
        let bits = v.l1_norm_significant_bits();
        assert_eq!(norm.significant_bits(), bits);
        assert_eq!(
            v.elements
                .iter()
                .map(Integer::unsigned_abs_ref)
                .sum::<Natural>(),
            norm
        );
        assert_eq!(vec_sum_max_bits_naive(&v.elements).0, bits);
        // The norm is at least the height, and at most the dimension times the height, and negating
        // the vector does not change it.
        let height = v.to_height();
        assert!(height <= norm);
        assert!(norm <= &height * Natural::from(v.dimension()));
        assert_eq!((-&v).to_l1_norm(), norm);
    });
}

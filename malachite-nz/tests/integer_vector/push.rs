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
use malachite_nz::integer_vector::IntegerVector;
use malachite_nz::test_util::generators::integer_vector_pair_gen;

#[test]
fn test_push() {
    let mut v = IntegerVector::from_str("(1, 2)").unwrap();
    v.push(Integer::from(-3));
    assert_eq!(v.to_string(), "(1, 2, -3)");
    let mut v = IntegerVector::zero(0);
    v.push(Integer::from(5));
    assert_eq!(v.to_string(), "(5)");
}

#[test]
fn push_properties() {
    integer_vector_pair_gen().test_properties(|(v, w)| {
        let mut x = v.clone();
        for e in &w.elements {
            let dimension = x.dimension();
            x.push(e.clone());
            assert_eq!(x.dimension(), dimension + 1);
            assert_eq!(x.elements.last(), Some(e));
        }
        // Pushing the elements of `w` one at a time is extending by `w`.
        let mut y = v.clone();
        y.extend(w);
        assert_eq!(x, y);
    });
}

// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{Conjugate, ConjugateAssign};
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_gen;
use std::str::FromStr;

#[test]
fn test_conjugate() {
    let test = |s| {
        let x = Integer::from_str(s).unwrap();

        let conjugate = x.clone().conjugate();
        assert!(conjugate.is_valid());
        assert_eq!(conjugate, x);

        let conjugate = (&x).conjugate();
        assert!(conjugate.is_valid());
        assert_eq!(conjugate, x);

        let mut conjugate = x.clone();
        conjugate.conjugate_assign();
        assert_eq!(conjugate, x);
    };
    test("0");
    test("123");
    test("-123");
    test("1000000000000");
}

#[test]
fn conjugate_properties() {
    integer_gen().test_properties(|x| {
        let conjugate = x.clone().conjugate();
        assert!(conjugate.is_valid());
        assert_eq!(conjugate, x);
        assert_eq!((&x).conjugate(), x);
        let mut x_alt = x.clone();
        x_alt.conjugate_assign();
        assert_eq!(x_alt, x);
    });
}

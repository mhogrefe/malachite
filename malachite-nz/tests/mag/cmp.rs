// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::mag::Mag;
use malachite_nz::test_util::generators::{mag_gen, mag_pair_gen, mag_triple_gen};
use malachite_nz::test_util::mag::{cmp_scaled, mag_to_scaled_natural};
use std::cmp::Ordering::{self, *};
use std::str::FromStr;

#[test]
fn test_cmp() {
    let test = |s, t, out: Ordering| {
        let x = Mag::from_str(s).unwrap();
        let y = Mag::from_str(t).unwrap();
        assert_eq!(x.cmp(&y), out);
        assert_eq!(x.partial_cmp(&y), Some(out));
        assert_eq!(y.cmp(&x), out.reverse());
    };
    test("0", "0", Equal);
    test("0", "1", Less);
    test("1", "1", Equal);
    test("1", "3*2^-1", Less);
    // - equal exponents, so the mantissas decide
    test("3*2^-1", "5*2^-2", Greater);
    // - a smaller value with a larger mantissa
    test("1073741823*2^-30", "1", Less);
    test("1*2^-100", "0", Greater);
    test("1*2^100", "1*2^99", Greater);
}

#[test]
fn cmp_properties() {
    mag_pair_gen().test_properties(|(x, y)| {
        let ord = x.cmp(&y);
        let (a, i) = mag_to_scaled_natural(x);
        let (b, j) = mag_to_scaled_natural(y);
        assert_eq!(cmp_scaled(&a, i, &b, j), ord);
        assert_eq!(y.cmp(&x), ord.reverse());
        assert_eq!(x == y, ord == Equal);
        assert_eq!(x.partial_cmp(&y), Some(ord));
        assert_eq!(x.max(y), if ord == Less { y } else { x });
        assert_eq!(x.min(y), if ord == Greater { y } else { x });
    });

    mag_gen().test_properties(|x| {
        assert_eq!(x.cmp(&x), Equal);
        assert!(x >= Mag::ZERO);
    });

    mag_triple_gen().test_properties(|(x, y, z)| {
        if x < y && y < z {
            assert!(x < z);
        } else if x > y && y > z {
            assert!(x > z);
        }
    });
}

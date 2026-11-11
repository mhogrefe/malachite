// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::mag::Mag;
use malachite_nz::test_util::generators::{mag_gen, mag_pair_gen};
use malachite_nz::test_util::mag::assert_mag_sqrt_upper_bound;
use std::str::FromStr;

#[test]
fn test_sqrt() {
    let test = |s, out| {
        assert_eq!(Mag::from_str(s).unwrap().sqrt().to_string(), out);
    };
    test("0", "0");
    test("1", "536870913*2^-29");
    test("4", "536870913*2^-28");
    test("2", "759250125*2^-29");
    // - an odd exponent
    test("1*2^-101", "759250125*2^-80");
    test("1073741823*2^100", "536870913*2^36");
}

#[test]
fn sqrt_properties() {
    mag_gen().test_properties(|x| {
        let r = x.sqrt();
        assert_mag_sqrt_upper_bound(r, x, 2);
        assert_eq!(r.is_zero(), x.is_zero());
    });

    mag_pair_gen().test_properties(|(x, y)| {
        if x <= y {
            assert!(x.sqrt() <= y.sqrt());
        }
    });
}

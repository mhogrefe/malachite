// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::mag::Mag;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{mag_gen, mag_pair_gen, mag_unsigned_pair_gen};
use malachite_nz::test_util::mag::{assert_mag_upper_bound, mag_to_scaled_natural};
use std::str::FromStr;

#[test]
fn test_mul() {
    let test = |s, t, out| {
        let x = Mag::from_str(s).unwrap();
        let y = Mag::from_str(t).unwrap();
        assert_eq!(Mag::mul(x, y).to_string(), out);
        assert_eq!(Mag::mul(y, x).to_string(), out);
    };
    test("0", "0", "0");
    test("0", "1", "0");
    test("1", "1", "268435457*2^-28");
    test("3", "5", "503316481*2^-25");
    test("1073741823", "1073741823", "1073741823*2^30");
    test("3*2^-100", "5*2^200", "503316481*2^75");
}

#[test]
fn test_mul_u64() {
    let test = |s, y: u64, out| {
        assert_eq!(Mag::from_str(s).unwrap().mul_u64(y).to_string(), out);
    };
    test("0", 5, "0");
    test("3", 0, "0");
    test("3", 5, "503316481*2^-25");
    test("1", u64::MAX, "268435457*2^36");
}

#[test]
fn mul_properties() {
    mag_pair_gen().test_properties(|(x, y)| {
        let z = Mag::mul(x, y);
        let (a, i) = mag_to_scaled_natural(x);
        let (b, j) = mag_to_scaled_natural(y);
        assert_mag_upper_bound(z, &(a * b), i + j, 2);
        assert_eq!(Mag::mul(y, x), z);
    });

    mag_gen().test_properties(|x| {
        assert_eq!(Mag::mul(x, Mag::ZERO), Mag::ZERO);
        assert_eq!(Mag::mul(Mag::ZERO, x), Mag::ZERO);
    });
}

#[test]
fn mul_u64_properties() {
    mag_unsigned_pair_gen().test_properties(|(x, y)| {
        let z = x.mul_u64(y);
        let (a, i) = mag_to_scaled_natural(x);
        assert_mag_upper_bound(z, &(a * Natural::from(y)), i, 4);
        assert_eq!(Mag::mul(x, Mag::from(y)), z);
    });
}

// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::unsigned_gen;
use malachite_nz::mag::Mag;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::mag::{assert_mag_lower_bound, assert_mag_upper_bound};

#[test]
fn test_from_u64() {
    let test = |x: u64, out, out_lower| {
        assert_eq!(Mag::from(x).to_string(), out);
        assert_eq!(Mag::from_u64_lower(x).to_string(), out_lower);
    };
    test(0, "0", "0");
    test(1, "1", "1");
    test(12, "3*2^2", "3*2^2");
    test((1 << 30) - 1, "1073741823", "1073741823");
    // - 31 bits: rounded
    test((1 << 31) - 1, "1*2^31", "1073741823*2^1");
    test(1 << 31, "536870913*2^2", "1*2^31");
    test((1 << 31) + 1, "536870913*2^2", "1*2^31");
    test(u64::MAX, "1*2^64", "1073741823*2^34");
}

#[test]
fn from_u64_properties() {
    unsigned_gen::<u64>().test_properties(|x| {
        let n = Natural::from(x);
        let upper = Mag::from(x);
        let lower = Mag::from_u64_lower(x);
        assert_mag_upper_bound(upper, &n, 0, 1);
        assert_mag_lower_bound(lower, &n, 0, 1);
        assert!(lower <= upper);
        assert_eq!(upper.is_zero(), x == 0);
        // - values with at most `MAG_BITS` bits are exact
        if x < 1 << 30 {
            assert_eq!(upper, lower);
        }
        assert!(Mag::from(&n) >= upper);
    });
    assert_eq!(Mag::from(0u64), Mag::ZERO);
    assert_eq!(Mag::from(1u64), Mag::ONE);
}

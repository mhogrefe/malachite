// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::traits::Zero;
use malachite_nz::mag::Mag;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::natural_gen;
use malachite_nz::test_util::mag::assert_mag_upper_bound;
use std::str::FromStr;

#[test]
fn test_from_natural() {
    let test = |s, out| {
        assert_eq!(Mag::from(&Natural::from_str(s).unwrap()).to_string(), out);
    };
    test("0", "0");
    test("1", "1");
    test("536870911", "536870911");
    // - exactly `MAG_BITS` bits, but still rounded up, as in FLINT, unlike `Mag::from(u64)`
    test("1073741823", "1*2^30");
    test("2147483647", "1*2^31");
    test("18446744073709551615", "1*2^64");
    // - two 64-bit words, with a top word of more than `MAG_BITS` bits
    test("340282366920938463463374607431768211455", "1*2^128");
    test("340282366920938463463374607431768211457", "536870913*2^99");
    // - a top word of fewer than `MAG_BITS` bits, so the second word supplies the rest
    test("18446744073709551617", "536870913*2^35");
    test("36893488147419103231", "1*2^65");
    test("1000000000000000000000000000000", "211758237*2^72");
}

#[test]
fn from_natural_properties() {
    natural_gen().test_properties(|x| {
        let m = Mag::from(&x);
        assert_mag_upper_bound(m, &x, 0, 1);
        assert_eq!(m.is_zero(), x == 0u32);
        // - at least as large as the bound from a `u64`, which is exact for up to `MAG_BITS` bits
        if let Ok(y) = u64::try_from(&x) {
            assert!(Mag::from(y) <= m);
        }
        // - exact below 2^(`MAG_BITS` - 1)
        if x < 1u32 << 29 {
            assert_eq!(m, Mag::from(u64::try_from(&x).unwrap()));
        }
    });
    assert_eq!(Mag::from(&Natural::ZERO), Mag::ZERO);
}

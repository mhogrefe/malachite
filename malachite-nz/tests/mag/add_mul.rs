// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::mag::Mag;
use malachite_nz::natural::Natural;
use malachite_nz::test_util::generators::{mag_pair_gen, mag_triple_gen};
use malachite_nz::test_util::mag::{assert_mag_upper_bound, mag_to_scaled_natural};
use std::str::FromStr;

// x * 2^i + y * 2^j, as (z, k) with the sum equal to z * 2^k.
fn add_scaled(x: Natural, i: i64, y: Natural, j: i64) -> (Natural, i64) {
    let k = i.min(j);
    (
        (x << u64::try_from(i - k).unwrap()) + (y << u64::try_from(j - k).unwrap()),
        k,
    )
}

#[test]
fn test_add_mul_assign() {
    let test = |s, t, u, out| {
        let mut z = Mag::from_str(s).unwrap();
        z.add_mul_assign(Mag::from_str(t).unwrap(), Mag::from_str(u).unwrap());
        assert_eq!(z.to_string(), out);
    };
    // - self is zero, so the result is the product
    test("0", "3", "5", "503316481*2^-25");
    // - a zero factor leaves self unchanged
    test("7", "0", "5", "7");
    test("7", "3", "0", "7");
    // - the product is at most self's size
    test("7", "3", "5", "369098753*2^-24");
    // - the product is far below self
    test("1*2^100", "3", "5", "536870913*2^71");
    // - the product is larger than self
    test("7", "3*2^10", "5", "251772929*2^-14");
    // - self is far below the product
    test("7", "3*2^100", "5", "251658241*2^76");
    // - the sum carries into the next power of 2
    test("1073741823*2^-30", "1*2^-31", "1", "1");
}

#[test]
fn add_mul_assign_properties() {
    mag_triple_gen().test_properties(|(z, x, y)| {
        let mut r = z;
        r.add_mul_assign(x, y);
        let (a, i) = mag_to_scaled_natural(x);
        let (b, j) = mag_to_scaled_natural(y);
        let (c, k) = mag_to_scaled_natural(z);
        let (sum, e) = add_scaled(a * b, i + j, c, k);
        assert_mag_upper_bound(r, &sum, e, 4);
        // - adding never makes the result smaller
        assert!(r >= z);
    });

    mag_pair_gen().test_properties(|(x, y)| {
        let mut r = Mag::ZERO;
        r.add_mul_assign(x, y);
        assert_eq!(r, Mag::mul(x, y));
        let mut r = x;
        r.add_mul_assign(y, Mag::ZERO);
        assert_eq!(r, x);
    });
}

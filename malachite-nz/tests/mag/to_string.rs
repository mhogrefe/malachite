// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_nz::mag::Mag;
use malachite_nz::test_util::generators::mag_gen;
use std::str::FromStr;

#[test]
fn test_to_string() {
    let test = |x: Mag, out| {
        assert_eq!(x.to_string(), out);
    };
    test(Mag::ZERO, "0");
    test(Mag::ONE, "1");
    test(Mag::from(3u64), "3");
    test(Mag::from(12u64), "3*2^2");
    test(Mag::from_mantissa_and_exponent(3 << 28, 0), "3*2^-2");
    test(
        Mag::from_mantissa_and_exponent((1 << 30) - 1, 0),
        "1073741823*2^-30",
    );
    // - u64::MAX has too many bits, so it rounds up to 2^64
    test(Mag::from(u64::MAX), "1*2^64");
}

#[test]
fn to_string_properties() {
    mag_gen().test_properties(|x| {
        let s = x.to_string();
        assert_eq!(Mag::from_str(&s).unwrap(), x);
    });
}

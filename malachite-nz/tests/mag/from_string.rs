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
fn test_from_str() {
    let test = |s, out: Option<(u64, i64)>| {
        assert_eq!(Mag::from_str(s).ok().map(Mag::mantissa_and_exponent), out);
    };
    test("0", Some((0, 0)));
    test("1", Some((1 << 29, 1)));
    test("3", Some((3 << 28, 2)));
    test("12", Some((3 << 28, 4)));
    test("3*2^2", Some((3 << 28, 4)));
    test("3*2^-2", Some((3 << 28, 0)));
    // - trailing zeros of m are absorbed into the exponent
    test("12*2^-2", Some((3 << 28, 2)));
    test("1*2^64", Some((1 << 29, 65)));
    test("1073741823", Some(((1 << 30) - 1, 30)));
    // - m has 31 bits after removing trailing zeros
    test("2147483647", None);
    // - zero with a nonzero power
    test("0*2^1", None);
    test("", None);
    test("-1", None);
    test("+1", None);
    test("1*2^", None);
    test("1*2^x", None);
    test("1*3^2", None);
    test("1.5", None);
    test("1*2^9223372036854775807", None);
}

#[test]
fn from_str_properties() {
    mag_gen().test_properties(|x| {
        assert_eq!(Mag::from_str(&x.to_string()).unwrap(), x);
    });
    assert_eq!(Mag::from_str("0").unwrap(), Mag::ZERO);
}

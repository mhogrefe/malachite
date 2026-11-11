// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::logic::traits::SignificantBits;
use malachite_float::test_util::big_exponent_float::{
    big_exponent_float_power_of_2, big_exponent_float_to_float, parse_big_exponent_float_hex,
};
use malachite_float::test_util::generators::{
    big_exponent_float_gen, big_exponent_float_gen_var_1,
};
use malachite_nz::integer::Integer;
use std::str::FromStr;

#[test]
fn test_complexity_and_significant_bits() {
    let test = |s, complexity, bits| {
        let x = parse_big_exponent_float_hex(s);
        assert_eq!(x.complexity(), complexity);
        assert_eq!(x.significant_bits(), bits);
    };
    test("NaN", 1, 1);
    test("Infinity", 1, 1);
    test("0x0.0", 1, 1);
    test("0x1.0#1", 1, 1);
    test("0x1.0000000000000000000000000#100", 1, 1);
    test("0x3.243f6a8885a30#53", 50, 50);
    test("0x1.0E+25#1", 100, 1);
    test("0x1.0E-25#1", 100, 1);
    // - an exponent too large for a `u64` saturates
    let x = big_exponent_float_power_of_2(Integer::from_str("100000000000000000000000").unwrap());
    assert_eq!(x.complexity(), u64::MAX);
    assert_eq!(x.significant_bits(), 1);
    let x = big_exponent_float_power_of_2(Integer::from(1u64 << 40));
    assert_eq!(x.complexity(), 1 << 40);
}

#[test]
fn complexity_properties() {
    big_exponent_float_gen().test_properties(|x| {
        let complexity = x.complexity();
        let bits = x.significant_bits();
        assert_ne!(bits, 0);
        assert!(complexity >= bits);
        assert_eq!(bits, x.get_prec().unwrap_or(1));
    });

    big_exponent_float_gen_var_1().test_properties(|x| {
        let f = big_exponent_float_to_float(&x).unwrap();
        assert_eq!(x.complexity(), f.complexity());
        assert_eq!(x.significant_bits(), f.significant_bits());
    });
}

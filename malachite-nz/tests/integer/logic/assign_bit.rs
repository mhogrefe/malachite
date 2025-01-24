// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_unsigned_bool_triple_gen_var_1;
use rug;
use std::str::FromStr;

#[test]
fn test_assign_bit() {
    let test = |u, index, bit, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.assign_bit(index, bit);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n.set_bit(u32::exact_from(index), bit);
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, true, "1024");
    test("100", 0, true, "101");
    test("1000000000000", 10, true, "1000000001024");
    test(
        "1000000000000",
        100,
        true,
        "1267650600228229402496703205376",
    );
    test("5", 100, true, "1267650600228229401496703205381");
    test("0", 10, false, "0");
    test("0", 100, false, "0");
    test("1024", 10, false, "0");
    test("101", 0, false, "100");
    test("1000000001024", 10, false, "1000000000000");
    test("1000000001024", 100, false, "1000000001024");
    test(
        "1267650600228229402496703205376",
        100,
        false,
        "1000000000000",
    );
    test("1267650600228229401496703205381", 100, false, "5");
}

#[test]
fn assign_bit_properties() {
    integer_unsigned_bool_triple_gen_var_1().test_properties(|(n, index, bit)| {
        let mut mut_n = n.clone();
        mut_n.assign_bit(index, bit);
        assert!(mut_n.is_valid());
        let result = mut_n;

        let mut rug_n = rug::Integer::from(&n);
        rug_n.set_bit(u32::exact_from(index), bit);
        assert_eq!(Integer::from(&rug_n), result);
    });
}

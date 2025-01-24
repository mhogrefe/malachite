// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitAccess;
use malachite_nz::integer::Integer;
use malachite_nz::test_util::generators::integer_unsigned_pair_gen_var_2;
use rug;
use std::str::FromStr;

#[test]
fn test_flip_bit() {
    let test = |u, index, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.flip_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n.toggle_bit(u32::exact_from(index));
        assert_eq!(n.to_string(), out);
    };
    test("0", 10, "1024");
    test("1024", 10, "0");
    test("100", 0, "101");
    test("101", 0, "100");
    test("1000000000000", 10, "1000000001024");
    test("1000000001024", 10, "1000000000000");
    test("1000000000000", 100, "1267650600228229402496703205376");
    test("1267650600228229402496703205376", 100, "1000000000000");
    test("5", 100, "1267650600228229401496703205381");
    test("1267650600228229401496703205381", 100, "5");
    test("-4294967296", 0, "-4294967295");
    test("-4294967295", 0, "-4294967296");
}

#[test]
fn flip_bit_properties() {
    integer_unsigned_pair_gen_var_2().test_properties(|(n, index)| {
        let mut mut_n = n.clone();
        mut_n.flip_bit(index);
        assert!(mut_n.is_valid());
        let result = mut_n;

        assert_ne!(result, n);

        let mut rug_n = rug::Integer::from(&n);
        rug_n.toggle_bit(u32::exact_from(index));
        assert_eq!(Integer::from(&rug_n), result);

        let mut mut_result = result.clone();
        mut_result.flip_bit(index);
        assert_eq!(mut_result, n);

        assert_eq!(n ^ Integer::power_of_2(index), result);
    });
}

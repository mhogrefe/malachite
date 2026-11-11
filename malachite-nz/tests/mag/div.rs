// Copyright © 2026 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::generators::unsigned_gen_var_1;
use malachite_nz::mag::Mag;
use malachite_nz::test_util::generators::mag_unsigned_pair_gen_var_1;
use malachite_nz::test_util::mag::assert_mag_div_u64_upper_bound;
use std::str::FromStr;

#[test]
fn test_div_u64() {
    let test = |s, y: u64, out| {
        assert_eq!(Mag::from_str(s).unwrap().div_u64(y).to_string(), out);
    };
    test("0", 5, "0");
    test("1", 1, "536870913*2^-29");
    test("15", 5, "805306369*2^-28");
    test("1", 3, "715827883*2^-31");
    test("1073741823", 1073741823, "536870913*2^-29");
    test("1", u64::MAX, "536870913*2^-93");
}

#[test]
#[should_panic]
fn div_u64_fail() {
    Mag::ONE.div_u64(0);
}

#[test]
fn div_u64_properties() {
    mag_unsigned_pair_gen_var_1().test_properties(|(x, y)| {
        assert_mag_div_u64_upper_bound(x.div_u64(y), x, y, 4);
    });

    unsigned_gen_var_1().test_properties(|y: u64| {
        assert_eq!(Mag::ZERO.div_u64(y), Mag::ZERO);
    });
}

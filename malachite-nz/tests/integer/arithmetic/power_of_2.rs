// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::{BitAccess, LowMask};
use malachite_base::test_util::generators::{unsigned_gen_var_16, unsigned_gen_var_5};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;

#[test]
fn test_power_of_2() {
    let test = |pow, out| assert_eq!(Integer::power_of_2(pow).to_string(), out);
    test(0, "1");
    test(1, "2");
    test(2, "4");
    test(3, "8");
    test(32, "4294967296");
    test(100, "1267650600228229401496703205376");
}

#[test]
fn power_of_2_properties() {
    unsigned_gen_var_5().test_properties(|pow| {
        let n = Integer::power_of_2(pow);
        assert!(n.is_valid());

        assert_eq!(n, Integer::ONE << pow);
        assert_eq!(n, Integer::low_mask(pow) + Integer::ONE);
        assert_eq!(Natural::exact_from(&n), Natural::power_of_2(pow));
        let mut n = n;
        n.clear_bit(pow);
        assert_eq!(n, 0);
    });

    unsigned_gen_var_16::<SignedLimb>().test_properties(|pow| {
        assert_eq!(SignedLimb::power_of_2(pow), Integer::power_of_2(pow));
    });
}

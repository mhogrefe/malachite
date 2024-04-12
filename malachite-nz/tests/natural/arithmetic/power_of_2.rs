// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::{
    CheckedLogBase2, CheckedRoot, IsPowerOf2, Pow, PowerOf2,
};
use malachite_base::num::basic::traits::{One, Two};
use malachite_base::num::logic::traits::{BitAccess, LowMask};
use malachite_base::test_util::generators::{unsigned_gen_var_15, unsigned_gen_var_5};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

#[test]
fn test_power_of_2() {
    let test = |pow, out| assert_eq!(Natural::power_of_2(pow).to_string(), out);
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
        let n = Natural::power_of_2(pow);
        assert!(n.is_valid());
        assert_eq!(n, Natural::ONE << pow);
        assert_eq!(n, Natural::TWO.pow(pow));
        assert_eq!(n, Natural::low_mask(pow) + Natural::ONE);
        assert!(n.is_power_of_2());
        assert_eq!(n.checked_log_base_2().unwrap(), pow);
        if pow != 0 {
            assert_eq!((&n).checked_root(pow).unwrap(), 2);
        }
        let mut n = n;
        n.clear_bit(pow);
        assert_eq!(n, 0);
    });

    unsigned_gen_var_15::<Limb>().test_properties(|pow| {
        assert_eq!(Limb::power_of_2(pow), Natural::power_of_2(pow));
    });
}

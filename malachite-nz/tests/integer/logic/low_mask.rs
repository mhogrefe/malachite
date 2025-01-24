// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::One;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::logic::traits::BitScan;
use malachite_base::num::logic::traits::LowMask;
use malachite_base::test_util::generators::{unsigned_gen_var_15, unsigned_gen_var_5};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::SignedLimb;

#[test]
fn test_low_mask() {
    let test = |bits, out| assert_eq!(Integer::low_mask(bits).to_string(), out);
    test(0, "0");
    test(1, "1");
    test(2, "3");
    test(3, "7");
    test(32, "4294967295");
    test(100, "1267650600228229401496703205375");
}

#[test]
fn low_mask_properties() {
    unsigned_gen_var_5().test_properties(|bits| {
        let n = Integer::low_mask(bits);
        assert!(n.is_valid());

        assert_eq!(n, Integer::power_of_2(bits) - Integer::ONE);
        assert_eq!(Natural::exact_from(&n), Natural::low_mask(bits));
        assert_eq!(n.index_of_next_false_bit(0), Some(bits));
    });

    unsigned_gen_var_15::<SignedLimb>().test_properties(|bits| {
        assert_eq!(SignedLimb::low_mask(bits), Integer::low_mask(bits));
    });
}

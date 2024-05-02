// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::PowerOf2;
use malachite_base::num::basic::traits::One;
use malachite_base::num::logic::traits::{BitScan, CountOnes, LowMask};
use malachite_base::test_util::generators::{unsigned_gen_var_5, unsigned_gen_var_9};
use malachite_nz::natural::logic::low_mask::limbs_low_mask;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_low_mask() {
    let test = |bits, out: &[Limb]| assert_eq!(limbs_low_mask(bits), out);
    test(0, &[]);
    test(1, &[1]);
    test(2, &[3]);
    test(3, &[7]);
    test(32, &[u32::MAX]);
    test(100, &[u32::MAX, u32::MAX, u32::MAX, 15]);
}

#[test]
fn test_low_mask() {
    let test = |bits, out| assert_eq!(Natural::low_mask(bits).to_string(), out);
    test(0, "0");
    test(1, "1");
    test(2, "3");
    test(3, "7");
    test(32, "4294967295");
    test(100, "1267650600228229401496703205375");
}

#[test]
fn limbs_low_mask_properties() {
    unsigned_gen_var_5().test_properties(|bits| {
        assert_eq!(
            Natural::from_owned_limbs_asc(limbs_low_mask(bits)),
            Natural::low_mask(bits)
        );
    });
}

#[test]
fn low_mask_properties() {
    unsigned_gen_var_5().test_properties(|bits| {
        let n = Natural::low_mask(bits);
        assert!(n.is_valid());
        assert_eq!(n, Natural::power_of_2(bits) - Natural::ONE);
        assert_eq!(n.count_ones(), bits);
        assert_eq!(n.index_of_next_false_bit(0), Some(bits));
    });

    unsigned_gen_var_9::<Limb>().test_properties(|bits| {
        assert_eq!(Limb::low_mask(bits), Natural::low_mask(bits));
    });
}

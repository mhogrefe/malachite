// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::slices::{slice_set_zero, slice_test_zero};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_vec_gen;

#[test]
fn test_slice_test_zero() {
    let test = |xs: &[u32], out| {
        assert_eq!(slice_test_zero(xs), out);
    };
    test(&[], true);
    test(&[0], true);
    test(&[0, 0, 0], true);
    test(&[123], false);
    test(&[123, 0], false);
    test(&[0, 123, 0, 0, 0], false);
}

#[test]
fn slice_test_zero_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    config.insert("mean_stripe_n", 16 << u8::LOG_WIDTH);
    config.insert("mean_stripe_d", 1);
    unsigned_vec_gen::<u8>().test_properties_with_config(&config, |xs| {
        let xs_are_zero = slice_test_zero(&xs);
        let mut new_xs = xs.clone();
        slice_set_zero(&mut new_xs);
        assert_eq!(xs == new_xs, xs_are_zero);
    });
}

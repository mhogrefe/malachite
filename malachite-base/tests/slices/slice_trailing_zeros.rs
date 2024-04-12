// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::slices::{slice_leading_zeros, slice_test_zero, slice_trailing_zeros};
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::unsigned_vec_gen;

#[test]
fn test_slice_trailing_zeros() {
    let test = |xs: &[u32], out| {
        assert_eq!(slice_trailing_zeros(xs), out);
    };
    test(&[], 0);
    test(&[0], 1);
    test(&[0, 0, 0], 3);
    test(&[123], 0);
    test(&[123, 0], 1);
    test(&[0, 123, 0, 0, 0], 3);
    test(&[1, 2, 3], 0);
    test(&[1, 2, 3, 0, 0, 0], 3);
}

#[test]
fn slice_trailing_zeros_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    config.insert("mean_stripe_n", 16 << u8::LOG_WIDTH);
    config.insert("mean_stripe_d", 1);
    unsigned_vec_gen::<u8>().test_properties_with_config(&config, |xs| {
        let trailing_zeros = slice_trailing_zeros(&xs);
        assert!(trailing_zeros <= xs.len());
        assert_eq!(trailing_zeros == xs.len(), slice_test_zero(&xs));
        let mut new_xs = xs;
        new_xs.reverse();
        assert_eq!(slice_leading_zeros(&new_xs), trailing_zeros);
    });
}

// Copyright © 2025 Mikhail Hogrefe
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
fn test_slice_set_zero() {
    let test = |xs: &[u32], out: &[u32]| {
        let mut mut_xs = xs.to_vec();
        slice_set_zero(&mut mut_xs);
        assert_eq!(mut_xs, out);
    };
    test(&[], &[]);
    test(&[0], &[0]);
    test(&[0, 0, 0], &[0, 0, 0]);
    test(&[123], &[0]);
    test(&[123, 0], &[0, 0]);
    test(&[0, 123, 0, 0, 0], &[0, 0, 0, 0, 0]);
}

#[test]
fn slice_set_zero_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    config.insert("mean_stripe_n", 16 << u8::LOG_WIDTH);
    config.insert("mean_stripe_d", 1);
    unsigned_vec_gen::<u8>().test_properties_with_config(&config, |mut xs| {
        let old_xs = xs.clone();
        slice_set_zero(&mut xs);
        assert_eq!(old_xs.len(), xs.len());
        assert!(slice_test_zero(&xs));
    });
}

// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::slices::slice_move_left;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_vec_gen, unsigned_vec_unsigned_pair_gen_var_1,
};
use malachite_base::test_util::slices::slice_move_left_naive;

#[test]
fn test_slice_move_left() {
    let test = |xs_in: &[u32], amount, xs_out: &[u32]| {
        let mut xs = xs_in.to_vec();
        slice_move_left(&mut xs, amount);
        assert_eq!(xs, xs_out);

        let mut xs = xs_in.to_vec();
        slice_move_left_naive::<u32>(&mut xs, amount);
        assert_eq!(xs, xs_out);
    };
    test(&[], 0, &[]);
    test(&[1], 0, &[1]);
    test(&[1], 1, &[1]);
    test(&[1, 2, 3], 0, &[1, 2, 3]);
    test(&[1, 2, 3], 1, &[2, 3, 3]);
    test(&[1, 2, 3], 2, &[3, 2, 3]);
    test(&[1, 2, 3], 3, &[1, 2, 3]);
    test(&[1, 2, 3, 4, 5, 6], 2, &[3, 4, 5, 6, 5, 6]);
}

#[test]
#[should_panic]
fn slice_move_left_fail_1() {
    let xs = &mut [];
    slice_move_left::<u32>(xs, 1);
}

#[test]
#[should_panic]
fn slice_move_left_fail_2() {
    let xs = &mut [1, 2, 3];
    slice_move_left::<u32>(xs, 4);
}

#[test]
fn slice_move_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    config.insert("mean_stripe_n", 16 << u8::LOG_WIDTH);
    config.insert("mean_stripe_d", 1);
    unsigned_vec_unsigned_pair_gen_var_1::<u8>().test_properties_with_config(
        &config,
        |(mut xs, amount)| {
            let old_xs = xs.clone();
            slice_move_left(&mut xs, amount);
            let boundary = old_xs.len() - amount;
            let (xs_lo, xs_hi) = xs.split_at(boundary);
            assert_eq!(xs_lo, &old_xs[amount..]);
            assert_eq!(xs_hi, &old_xs[boundary..]);

            let mut xs_alt = old_xs;
            slice_move_left_naive(&mut xs_alt, amount);
            assert_eq!(xs_alt, xs);
        },
    );

    unsigned_vec_gen::<u8>().test_properties_with_config(&config, |mut xs| {
        let old_xs = xs.clone();
        slice_move_left(&mut xs, 0);
        assert_eq!(xs, old_xs);

        let mut xs = old_xs.clone();
        slice_move_left(&mut xs, old_xs.len());
        assert_eq!(xs, old_xs);
    });
}

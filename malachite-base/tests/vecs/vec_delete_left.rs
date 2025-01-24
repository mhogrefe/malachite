// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_vec_gen, unsigned_vec_unsigned_pair_gen_var_1,
};
use malachite_base::vecs::vec_delete_left;

#[test]
fn test_vec_delete_left() {
    let test = |xs: &[u32], delete_size: usize, out: &[u32]| {
        let mut mut_xs = xs.to_vec();
        vec_delete_left(&mut mut_xs, delete_size);
        assert_eq!(mut_xs, out);
    };
    test(&[], 0, &[]);
    test(&[1, 2, 3, 4, 5], 0, &[1, 2, 3, 4, 5]);
    test(&[1, 2, 3, 4, 5], 3, &[4, 5]);
    test(&[1, 2, 3, 4, 5], 5, &[]);
}

#[test]
#[should_panic]
fn vec_delete_left_fail_1() {
    let mut xs: Vec<u32> = Vec::new();
    vec_delete_left(&mut xs, 1);
}

#[test]
#[should_panic]
fn vec_delete_left_fail_2() {
    let mut xs: Vec<u32> = vec![1, 2, 3];
    vec_delete_left(&mut xs, 4);
}

#[test]
fn vec_delete_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    config.insert("mean_stripe_n", 16 << u8::LOG_WIDTH);
    config.insert("mean_stripe_d", 1);
    unsigned_vec_unsigned_pair_gen_var_1::<u8>().test_properties_with_config(
        &config,
        |(mut xs, amount)| {
            let old_xs = xs.clone();
            vec_delete_left(&mut xs, amount);
            assert_eq!(xs == old_xs, amount == 0);
            assert_eq!(xs.is_empty(), amount == old_xs.len());
            assert_eq!(xs.len(), old_xs.len() - amount);
            assert_eq!(&old_xs[amount..], xs);
        },
    );

    unsigned_vec_gen::<u8>().test_properties_with_config(&config, |mut xs| {
        let old_xs = xs.clone();
        vec_delete_left(&mut xs, old_xs.len());
        assert!(xs.is_empty());

        let mut xs = old_xs.clone();
        vec_delete_left(&mut xs, 0);
        assert_eq!(xs, old_xs);
    });
}

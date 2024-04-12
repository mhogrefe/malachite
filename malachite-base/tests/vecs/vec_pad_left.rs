// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{
    unsigned_pair_gen_var_2, unsigned_vec_unsigned_pair_gen,
    unsigned_vec_unsigned_unsigned_triple_gen_var_1,
};
use malachite_base::vecs::{vec_delete_left, vec_pad_left};

#[test]
fn test_vec_pad_left() {
    let test = |xs: &[u32], pad_size: usize, pad_value: u32, out: &[u32]| {
        let mut mut_xs = xs.to_vec();
        vec_pad_left(&mut mut_xs, pad_size, pad_value);
        assert_eq!(mut_xs, out);
    };
    test(&[], 3, 6, &[6, 6, 6]);
    test(&[1, 2, 3], 0, 10, &[1, 2, 3]);
    test(&[1, 2, 3], 5, 10, &[10, 10, 10, 10, 10, 1, 2, 3]);
}

#[test]
fn vec_pad_left_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    config.insert("mean_stripe_n", 16 << u8::LOG_WIDTH);
    config.insert("mean_stripe_d", 1);
    config.insert("small_unsigned_length_n", 32);
    config.insert("small_unsigned_length_d", 1);
    unsigned_vec_unsigned_unsigned_triple_gen_var_1::<u8, usize, u8>().test_properties_with_config(
        &config,
        |(mut xs, pad_size, pad_value)| {
            let old_xs = xs.clone();
            vec_pad_left(&mut xs, pad_size, pad_value);
            assert_eq!(xs == old_xs, pad_size == 0);
            assert_eq!(xs.len(), old_xs.len() + pad_size);
            assert!(xs[..pad_size].iter().all(|&x| x == pad_value));
            assert_eq!(&xs[pad_size..], old_xs);
            vec_delete_left(&mut xs, pad_size);
            assert_eq!(xs, old_xs);
        },
    );

    unsigned_vec_unsigned_pair_gen::<u8, u8>().test_properties_with_config(
        &config,
        |(mut xs, pad_value)| {
            let old_xs = xs.clone();
            vec_pad_left(&mut xs, 0, pad_value);
            assert_eq!(xs, old_xs);
        },
    );

    unsigned_pair_gen_var_2::<u8, usize>().test_properties_with_config(
        &config,
        |(pad_value, pad_size)| {
            let mut xs = Vec::new();
            vec_pad_left(&mut xs, pad_size, pad_value);
            assert_eq!(xs, vec![pad_value; pad_size]);
        },
    );
}

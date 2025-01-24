// Copyright © 2025 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::num::arithmetic::traits::DivisibleBy;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::slices::min_repeating_len;
use malachite_base::test_util::generators::common::GenConfig;
use malachite_base::test_util::generators::{unsigned_gen, unsigned_vec_gen_var_4};

#[test]
fn test_min_repeating_len() {
    let test = |xs: &[u32], out| {
        assert_eq!(min_repeating_len(xs), out);
    };
    test(&[1, 2, 1, 2, 1, 2], 2);
    test(&[1, 2, 1, 2, 1, 3], 6);
    test(&[5, 5, 5], 1);
    test(&[100], 1);
}

#[test]
#[should_panic]
fn min_repeating_len_fail() {
    min_repeating_len::<u8>(&[]);
}

#[test]
fn min_repeating_len_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_length_n", 32);
    config.insert("mean_length_d", 1);
    config.insert("mean_stripe_n", 16 << u8::LOG_WIDTH);
    config.insert("mean_stripe_d", 1);
    unsigned_vec_gen_var_4::<u8>().test_properties_with_config(&config, |xs| {
        let len = xs.len();
        let repeating_len = min_repeating_len(&xs);
        assert_ne!(repeating_len, 0);
        assert!(len.divisible_by(repeating_len));
        let rep = &xs[..repeating_len];
        assert_eq!(min_repeating_len(rep), repeating_len);
        assert!(Iterator::eq(rep.iter().cycle().take(len), xs.iter()));
    });

    unsigned_gen::<u8>().test_properties_with_config(&config, |x| {
        assert_eq!(min_repeating_len(&[x]), 1);
    });
}
